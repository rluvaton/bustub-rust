use parking_lot::{Mutex, MutexGuard};
use std::collections::{HashMap, LinkedList};
use std::ops::Deref;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use common::{SharedFuture, SharedPromise};
use disk_storage::{DiskScheduler};
use pages::{Page, PageAndGuard, PageAndReadGuard, PageAndWriteGuard, AtomicPageId, PageId, INVALID_PAGE_ID};

#[cfg(feature = "tracing")]
use tracy_client::span;
use buffer_common::{AccessType, FrameId};
use eviction_policy::{EvictionPolicy};
use recovery_log_manager::LogManager;
use crate::{errors, BufferPool, PageReadGuard, PageWriteGuard};
use crate::builder::BufferPoolManagerBuilder;

#[cfg(feature = "statistics")]
use crate::BufferPoolManagerStats;

///
/// BufferPoolManager reads disk pages to and from its internal buffer pool.
///
pub struct BufferPoolManager {
    /// The next page id to be allocated
    pub(super) next_page_id: AtomicPageId,

    /// Number of pages in the buffer pool
    /// This will not change after initial set
    pub(super) pool_size: usize,


    /// Pointer to the log manager. Please ignore this for P1.
    // LogManager *log_manager_ __attribute__((__unused__));
    #[allow(unused)]
    pub(super) log_manager: Option<Arc<LogManager>>,

    pub(super) inner: Mutex<InnerBufferPoolManager>,

    /// Pending fetch requests from disk
    pub(super) pending_fetch_requests: Mutex<HashMap<PageId, SharedFuture<()>>>,

    #[cfg(feature = "statistics")]
    /// Statistics on buffer pool
    pub(super) stats: BufferPoolManagerStats,
}

unsafe impl Sync for BufferPoolManager {}

pub(super) struct InnerBufferPoolManager {
    /** Array of buffer pool pages. */
    // The index is the frame_id
    pub(super) pages: Vec<Page>,

    /// Page table for keeping track of buffer pool pages.
    ///
    /// ## Original type:
    /// ```cpp
    /// std::unordered_map<page_id_t, frame_id_t> page_table_;
    /// ```
    ///
    /// this is a thread safe hashmap
    pub(super) page_table: HashMap<PageId, FrameId>,

    /// Eviction policy to find unpinned pages for replacement.
    pub(super) eviction_policy: Box<dyn EvictionPolicy>,

    /// List of free frames that don't have any pages on them.
    pub(super) free_list: LinkedList<FrameId>,

    /// Pointer to the disk scheduler.
    /// This is inside `Arc` to allow dropping `inner` when calling disk scheduler
    /// It is our responsibility to not leave dangling scheduler outside the mutex guard
    pub(super) disk_scheduler: Arc<DiskScheduler>,
}

impl BufferPoolManager {
    pub fn builder() -> BufferPoolManagerBuilder {
        BufferPoolManagerBuilder::default()
    }

    /// Allocate a page on disk. Caller should acquire the latch before calling this function.
    ///
    /// # Arguments
    ///
    /// * `inner`:
    ///
    /// returns: PageId page id of the allocated page
    ///
    fn allocate_page(&self) -> PageId {
        self.next_page_id.fetch_add(1, Ordering::SeqCst)
    }

    /**
     * @brief Deallocate a page on disk. Caller should acquire the latch before calling this function.
     * @param page_id id of the page to deallocate
     */
    fn deallocate_page(&self, _page_id: PageId) {
        // This is a no-nop right now without a more complex data structure to track deallocated pages
        // TODO - call disk scheduler to deallocate the page
    }

    /// Find replacement frame from either the free list or the replacer, always use the free list first
    ///
    /// # Arguments
    ///
    /// * `replacer_guard`: the replacer guard to use, this must be passed instead of locking in this function for thread safety
    ///
    /// returns: Result<FrameId, NoAvailableFrameFound> Frame id if available frame found or error if not
    ///
    fn find_replacement_frame(&self, inner: &mut InnerBufferPoolManager) -> Result<FrameId, errors::NoAvailableFrameFound> {
        // Pick the replacement frame from the free list first
        if !inner.free_list.is_empty() {
            inner.free_list.pop_front().ok_or(errors::NoAvailableFrameFound)
        } else {
            // pick replacement from the replacer, can't be empty
            inner.eviction_policy.evict().ok_or(errors::NoAvailableFrameFound)
        }
    }

    fn wait_for_pending_request_page_to_finish(&self, requests_map: &Mutex<HashMap<PageId, SharedFuture<()>>>, page_id: PageId) -> MutexGuard<InnerBufferPoolManager> {
        // TODO - wait for condvar to avoid taking cpu time

        // 1. Hold replacer guard as all pin and unpin must first hold the replacer to avoid getting replaced in the middle
        let mut inner = self.inner.lock();

        // 2. Wait for the fetch from disk to finish
        loop {

            // 2.1. Lock pending fetch requests
            let pending_fetch_request = requests_map.lock().get(&page_id).cloned();

            // 2.2. If pending fetch requests has the current requested page, release teh replacer and wait for the pending to finish
            if let Some(pending_fetch_request) = pending_fetch_request {
                // 2.2.1. Release locks so we won't block while we wait for the other fetch to finish
                drop(inner);

                // 2.2.2. Wait for the fetch to finish
                pending_fetch_request.wait();

                // 2.2.3. Try to acquire again the replacer so we nothing can add to the pending again
                inner = self.inner.lock();
            } else {
                // 2.2.1 No pending fetch requested page is running
                return inner;
            }
        }
    }

    fn wait_for_pending_fetch_page_to_finish(&self, page_id: PageId) -> MutexGuard<InnerBufferPoolManager> {
        self.wait_for_pending_request_page_to_finish(&self.pending_fetch_requests, page_id)
    }

    fn finish_current_pending_fetch_page_request(&self, page_id: PageId, fetch_promise: SharedPromise<()>) {
        // First removing the pending requests
        self.pending_fetch_requests.lock().remove(&page_id);

        // Then release the lock
        fetch_promise.set_value(());
    }

    fn fetch_page<'a, PageAndGuardImpl: PageAndGuard<'a>, R, F: FnOnce(Arc<Self>, PageAndGuardImpl) -> R>(self: &Arc<Self>, page_id: PageId, access_type: AccessType, create_guard: F) -> Result<R, errors::FetchPageError> {
        // Find available frame

        // 1. Hold replacer guard as all pin and unpin must first hold the replacer to avoid getting replaced in the middle
        let mut inner = self.wait_for_pending_fetch_page_to_finish(page_id);

        #[cfg(any(feature = "tracing", feature = "statistics"))]
        let holding_inner_latch = (
            #[cfg(feature = "tracing")]
            span!("[fetch_page] Holding root lock"),
            #[cfg(feature = "statistics")]
            self.stats.holding_inner_latch.create_single(),
        );

        // We have 3 cases when fetching page
        // 1. page already exists in the buffer pool
        // 2. page does not exist in the buffer pool and we DO NOT NEED to flush existing page
        // 3. page does not exist in the buffer pool and we NEED to flush existing page
        // 3. Page exists in the buffer pool
        if let Some(&frame_id) = inner.page_table.get(&page_id) {
            // 3.1. Record access so the frame will be inserted and the replacer algorithm will work and avoid eviction in the meantime
            inner.record_access_and_avoid_eviction(frame_id, access_type);

            // 3.2 Get page
            let page = inner.pages[frame_id as usize].clone();

            // 3.3 Assert page table correctness
            // TODO - fix this?
            // assert_eq!(page.get_page_id(), page_id, "Page ID must be the same as the requested");

            // 3.4 Pin before returning to the caller to avoid page to be evicted in the meantime
            page.pin();

            // 3.5 Drop all locks before waiting for the page read lock
            #[cfg(any(feature = "tracing", feature = "statistics"))]
            drop(holding_inner_latch);
            drop(inner);

            let page_and_guard = PageAndGuardImpl::from(page);

            return Ok(create_guard(self.clone(), page_and_guard));
        }

        // Option 2, page does not exists in the buffer pool

        // 4. Find replacement frame
        let frame_id = self.find_replacement_frame(&mut inner)?;

        // 5. Create promise for when the entire fetch is finished
        let current_fetch_promise = SharedPromise::new();

        // 6. Register the promise
        self.pending_fetch_requests.lock().insert(page_id, current_fetch_promise.get_future());

        // 7. Record access so the frame will be inserted and the replacer algorithm will work and avoid eviction in the meantime
        inner.record_access_and_avoid_eviction(frame_id, access_type);

        // 8. Register the requested page in the page table
        inner.page_table.insert(page_id, frame_id);

        // We now have 2 options:
        // 1. the frame we got is currently have a page in it that we need to replace (and possibly flush)
        // 2. the frame we got is empty frame (we got it from the free list)

        let page_to_replace: Option<Page> = inner.pages.get_mut(frame_id as usize).cloned();

        // 9. If frame match existing page
        if let Some(page_to_replace) = page_to_replace {
            // Option 1, replacing existing frame

            // 1. Get write lock, the page to replace must never have write lock created outside the buffer pool as it is about to be pinned
            let mut page_to_replace_guard = PageAndWriteGuard::from(page_to_replace);

            // 2. Pin page
            page_to_replace_guard.page().pin();

            // 3. Remove the old page from the page table so it won't be available
            inner.page_table.remove(&page_to_replace_guard.get_page_id());

            // 4. If page to replace is dirty, need to flush it
            if page_to_replace_guard.page().is_dirty() {
                // 6. Add flush + read message to the scheduler
                let (flush_and_fetch_page_result, _) = inner.disk_scheduler.clone().write_and_read_page_from_disk(
                    page_to_replace_guard.write_guard_mut(),
                    page_id,
                    #[inline]
                    || {
                        // 8. release all locks as we don't want to hold the entire lock while flushing to disk

                        #[cfg(any(feature = "tracing", feature = "statistics"))]
                        drop(holding_inner_latch);
                        drop(inner);

                        // 9. Wait for the flush and fetch to finish
                        // TODO - handle errors in flushing
                    }
                );

                if !flush_and_fetch_page_result {
                    self.finish_current_pending_fetch_page_request(page_id, current_fetch_promise);

                    // TODO - reset page in page table
                    panic!("Must be able to flush and fetch page");
                }

                // 10. Mark page as not dirty after read from disk
                page_to_replace_guard.page().set_is_dirty(false);

                // 11. Set page id to be the correct page id
                page_to_replace_guard.set_page_id(page_id);

                // Convert write lock to the desired lock
                let page_to_replace_requested_guard = PageAndGuardImpl::from(page_to_replace_guard);

                self.finish_current_pending_fetch_page_request(page_id, current_fetch_promise);

                Ok(create_guard(self.clone(), page_to_replace_requested_guard))
            } else {
                // If page is not dirty we can just fetch the page
                page_to_replace_guard.set_page_id(page_id);

                // 6. Request read page
                let (fetch_page_result, _) = inner.disk_scheduler.clone().read_page_from_disk(
                    page_to_replace_guard.write_guard_mut(),
                    || {
                        // 7. release all locks as we don't want to hold the entire lock while flushing to disk
                        #[cfg(any(feature = "tracing", feature = "statistics"))]
                        drop(holding_inner_latch);
                        drop(inner);
                    });


                // 8. Wait for the fetch to finish

                if !fetch_page_result {
                    self.finish_current_pending_fetch_page_request(page_id, current_fetch_promise);

                    panic!("Must be able to fetch page");
                }


                // Convert write lock to the requested guard
                let page_to_replace_requested_guard = PageAndGuardImpl::from(page_to_replace_guard);

                self.finish_current_pending_fetch_page_request(page_id, current_fetch_promise);

                Ok(create_guard(self.clone(), page_to_replace_requested_guard))
            }
        } else {
            // If no page exists in the pages for the current frame, just fetch it

            // 1. Create new page
            let page = Page::new(page_id);

            // 2. Pin page
            page.pin();

            let mut write_guard = PageAndWriteGuard::from(page);

            // 5. Get the scheduler

            // 6. Request read page
            let (fetch_page_result, _) = inner.disk_scheduler.clone().read_page_from_disk(
                write_guard.write_guard_mut(),
                || {
                    // 7. release all locks as we don't want to hold the entire lock while flushing to disk
                    #[cfg(any(feature = "tracing", feature = "statistics"))]
                    drop(holding_inner_latch);
                    drop(inner);

                    // 8. Wait for the fetch to finish
                }
            );


            if !fetch_page_result {
                self.finish_current_pending_fetch_page_request(page_id, current_fetch_promise);

                panic!("Must be able to fetch page");
            }

            let requested_guard = PageAndGuardImpl::from(write_guard);

            self.finish_current_pending_fetch_page_request(page_id, current_fetch_promise);

            Ok(create_guard(self.clone(), requested_guard))
        }
    }

    /// Unpin page, must only be used by page guards
    ///
    /// # Arguments
    ///
    /// * `page_id`: Page id to unpin
    /// * `access_type`: for leaderboard
    ///
    /// returns: bool whether was able to unpin or not
    ///
    pub(super) fn unpin_page(&self, page_id: PageId, _access_type: AccessType) -> bool {

        // 1. first hold the replacer
        let mut inner = {
            let _stat = (
                #[cfg(feature = "tracing")]
                span!("[unpin_page] waiting for root lock"),
                #[cfg(feature = "statistics")]
                self.stats.waiting_for_inner_latch.create_single(),
            );
            self.inner.lock()
        };

        let _holding_inner_latch = (
            #[cfg(feature = "tracing")]
            span!("[unpin_page] holding root lock"),
            #[cfg(feature = "statistics")]
            self.stats.holding_inner_latch.create_single(),
        );

        // 2. check if the page table the page exists
        if let Some(&frame_id) = inner.page_table.get(&page_id) {

            // 3. Get the page to unpin
            let page = inner.pages[frame_id as usize].clone();

            // 4. If already evictable
            if page.get_pin_count() == 0 {
                return false;
            }

            // 5. unpin
            page.unpin();

            // 6. If we reached to pin count 0, this means we need to set as evictable
            if page.get_pin_count() == 0 {
                inner.eviction_policy.set_evictable(frame_id, true);
            }

            true
        } else {
            false
        }
    }

    #[cfg(feature = "statistics")]
    pub fn get_stats(&self) -> &BufferPoolManagerStats {
        &self.stats
    }
}

impl InnerBufferPoolManager {
    #[inline(always)]
    fn record_access_and_avoid_eviction(&mut self, frame_id: FrameId, access_type: AccessType) {

        // Avoid evicting the frame
        self.eviction_policy.set_evictable(frame_id, false);


        // Record access to the frame so the LRU-K would work
        // this is done after the set evictable for performance reasons (avoiding updating the evictable heap twice)
        self.eviction_policy.record_access(frame_id, access_type);
    }
}

impl BufferPool for Arc<BufferPoolManager> {
    fn get_pool_size(&self) -> usize {
        self.pool_size
    }

    fn new_page<'a>(&self, access_type: AccessType) -> Result<PageWriteGuard<'a>, errors::NewPageError> {
        // Find available frame

        // 1. Hold replacer guard as all pin and unpin must first hold the replacer to avoid getting replaced in the middle
        let mut inner = {
            let _stat = (
                #[cfg(feature = "tracing")]
                span!("[new_page] waiting for root lock"),
                #[cfg(feature = "statistics")]
                self.stats.waiting_for_inner_latch.create_single(),
            );
            self.inner.lock()
        };

        #[cfg(any(feature = "tracing", feature = "statistics"))]
        let holding_root_lock = (
            #[cfg(feature = "tracing")]
            span!("[new_page] holding root lock"),
            #[cfg(feature = "statistics")]
            self.stats.holding_inner_latch.create_single(),
        );

        // 2. Find replacement frame
        let frame_id = self.find_replacement_frame(&mut inner)?;

        // 3. Record access so the frame will be inserted and the replacer algorithm will work and avoid eviction in the meantime
        inner.record_access_and_avoid_eviction(frame_id, access_type);

        // 4. Allocate page id
        let page_id = self.allocate_page();

        // 5. Register the new page in the page table
        inner.page_table.insert(page_id, frame_id);

        // We now have 2 options:
        // 1. the frame we got is currently have a page in it that we need to replace (and possibly flush)
        // 2. the frame we got is empty frame (we got it from the free list)

        let page_to_replace: Option<Page> = inner.pages.get_mut(frame_id as usize).cloned();

        // 6. If frame match existing page
        if let Some(page_to_replace) = page_to_replace {
            // Option 1, replacing existing frame

            // 1. Get write lock, the page to replace must never have write lock created outside the buffer pool
            let mut page_and_write = PageAndWriteGuard::from(page_to_replace);

            // 2. Pin page
            page_and_write.page().pin();

            // 3. Remove the old page from the page table so it won't be available
            inner.page_table.remove(&page_and_write.get_page_id());

            // 4. If page to replace is dirty, need to flush it
            if page_and_write.page().is_dirty() {
                // 6. Add flush message to the scheduler
                let (flush_page_result, _) = inner.disk_scheduler.clone().write_page_to_disk(
                    page_and_write.deref(),
                    || {
                        // 7. release all locks as we don't want to hold the entire lock while flushing to disk
                        #[cfg(any(feature = "tracing", feature = "statistics"))]
                        drop(holding_root_lock);
                        drop(inner);

                        // 8. Wait for the flush to finish
                    }
                );

                // TODO - handle errors in flushing
                assert_eq!(flush_page_result, true, "Must be able to flush page");

                // 9. Reset dirty
                page_and_write.page().set_is_dirty(false);
            }

            // 5. Reset page data + Change page id to be this page id
            page_and_write.clear_page(page_id);

            // Return page write guard
            Ok(PageWriteGuard::new(self.clone(), page_and_write))
        } else {
            // Option 2, empty frame

            // 1. Create empty page
            let page = Page::new(page_id);

            // 2. Pin page
            page.pin();
            inner.pages.insert(frame_id as usize, page.clone());

            let page_and_write = PageAndWriteGuard::from(page);

            // 3. Return the PageWriteGuard
            Ok(PageWriteGuard::new(self.clone(), page_and_write))
        }
    }

    fn fetch_page_read(&self, page_id: PageId, access_type: AccessType) -> Result<PageReadGuard, errors::FetchPageError> {
        BufferPoolManager::fetch_page(self, page_id, access_type, |bpm, guard: PageAndReadGuard| {
            PageReadGuard::new(bpm, guard)
        })
    }

    fn fetch_page_write(&self, page_id: PageId, access_type: AccessType) -> Result<PageWriteGuard, errors::FetchPageError> {
        BufferPoolManager::fetch_page(self, page_id, access_type, |bpm, guard: PageAndWriteGuard| {
            PageWriteGuard::new(bpm, guard)
        })
    }

    fn flush_page(&self, page_id: PageId) -> bool {

        // 1. Hold replacer guard as all pin and unpin must first hold the replacer to avoid getting replaced in the middle
        let mut inner = {
            let _stat = (
                #[cfg(feature = "tracing")]
                span!("[flush_page] waiting for root lock"),
                #[cfg(feature = "statistics")]
                self.stats.waiting_for_inner_latch.create_single(),
            );
            self.inner.lock()
        };

        #[cfg(any(feature = "tracing", feature = "statistics"))]
        let holding_root_lock = (
            #[cfg(feature = "tracing")]
            span!("[flush_page] holding root lock"),
            #[cfg(feature = "statistics")]
            self.stats.holding_inner_latch.create_single(),
        );


        if !inner.page_table.contains_key(&page_id) {
            // Page is missing
            return false;
        }

        let &frame_id = inner.page_table.get(&page_id).unwrap();

        let page = inner.pages[frame_id as usize].clone();

        assert_eq!(page.is_locked_exclusive(), false, "Possible deadlock detected when trying to flush page {} when the page has already exclusive lock", page_id);

        // Avoid evicting in the middle
        inner.eviction_policy.set_evictable(frame_id, false);
        page.pin();

        let page_guard = page.read();

        // Add flush message to the scheduler
        let (flush_page_result, _) = inner.disk_scheduler.clone().write_page_to_disk(
            page_guard.deref(),
            #[inline]
            || {
                // release all locks as we don't want to hold the entire lock while flushing to disk
                #[cfg(any(feature = "tracing", feature = "statistics"))]
                drop(holding_root_lock);
                drop(inner);

                // Wait for the flush to finish
            }
        );

        // TODO - handle errors in flushing
        assert_eq!(flush_page_result, true, "Must be able to flush page");

        page.set_is_dirty(false);

        drop(page_guard);

        self.unpin_page(page_id, AccessType::Unknown);

        true
    }

    fn flush_all_pages(&self) {
        todo!()
    }

    fn delete_page(&self, page_id: PageId) -> Result<bool, errors::DeletePageError> {
        if page_id == INVALID_PAGE_ID {
            return Err(errors::DeletePageError::InvalidPageId);
        }

        let mut inner = {
            let _stat = (
                #[cfg(feature = "tracing")]
                span!("[delete_page] waiting for root lock"),
                #[cfg(feature = "statistics")]
                self.stats.waiting_for_inner_latch.create_single(),
            );
            self.inner.lock()
        };

        let _holding_root_lock = (
            #[cfg(feature = "tracing")]
            span!("[delete_page] holding root lock"),
            #[cfg(feature = "statistics")]
            self.stats.holding_inner_latch.create_single(),
        );


        if !inner.page_table.contains_key(&page_id) {
            // Page is missing
            return Ok(true);
        }

        let &frame_id = inner.page_table.get(&page_id).unwrap();
        let page = inner.pages[frame_id as usize].clone();

        if page.is_pinned() {
            return Err(errors::DeletePageError::PageIsNotEvictable(page_id));
        }

        // TODO - what about if page is dirty?
        (*inner).page_table.remove(&page_id);
        (*inner).free_list.push_front(frame_id);

        (*inner).eviction_policy.remove(frame_id);

        page.with_write(|u| {
            // Do not remove the item, and instead change it to INVALID_PAGE_ID
            // so we won't change the frame location
            u.clear_page(INVALID_PAGE_ID)
        });
        page.set_is_dirty(false);

        BufferPoolManager::deallocate_page(self, page_id);

        Ok(true)
    }

    fn get_pin_count(&self, page_id: PageId) -> Option<usize> {
        // 1. first hold the inner lock
        let inner = {
            let _stat = (
                #[cfg(feature = "tracing")]
                span!("[get_pin_count] waiting for root lock"),
                #[cfg(feature = "statistics")]
                self.stats.waiting_for_inner_latch.create_single(),
            );
            self.inner.lock()
        };

        let _holding_root_lock = (
            #[cfg(feature = "tracing")]
            span!("[get_pin_count] holding root lock"),
            #[cfg(feature = "statistics")]
            self.stats.holding_inner_latch.create_single(),
        );

        // 2. check if the page table the page exists

        if let Some(&frame_id) = inner.page_table.get(&page_id) {

            // 3. Get the page to unpin
            Some(inner.pages[frame_id as usize].get_pin_count())
        } else {
            None
        }
    }
}

impl Default for BufferPoolManager {
    fn default() -> Self {
        Self::builder().build()
    }
}
