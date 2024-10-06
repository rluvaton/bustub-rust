use common::config::{AtomicPageId, FrameId, PageData, PageId, INVALID_PAGE_ID, LRUK_REPLACER_K};
use dashmap::DashMap;
use parking_lot::{Mutex, MutexGuard};
use std::collections::{HashMap, LinkedList};
use std::sync::atomic::Ordering;
use std::sync::Arc;
use common::{Future, Promise, UnsafeSingleRefData, UnsafeSingleRefMutData};
use crate::buffer::buffer_pool_manager::*;
use crate::buffer::{AccessType, LRUKReplacerImpl, Replacer};
use crate::buffer::buffer_pool_manager::errors::{DeletePageError, FetchPageError};
use crate::recovery::LogManager;
use crate::storage::{DiskManager, DiskScheduler, Page, PageAndGuard, PageAndReadGuard, PageAndWriteGuard, ReadDiskRequest, UnderlyingPage, WriteDiskRequest};

///
/// BufferPoolManager reads disk pages to and from its internal buffer pool.
///
pub struct BufferPoolManager {
    /// The next page id to be allocated
    next_page_id: AtomicPageId,

    /// Number of pages in the buffer pool
    /// This will not change after initial set
    /// TODO - remove pub(crate) and expose getter to avoid user setting the value
    pool_size: usize,

    // TODO - panic will release the parking lot Mutex lock which can leave undesired state
    //        replace to original Mutex
    /// This latch protects the root level data until we get to the actual page instance, this is here to be the gateway in the inner data
    root_level_latch: Mutex<()>,

    /// This is just container to the inner buffer pool manager, so we can do locking with better granularity
    /// as it allow for multiple mutable reference at the same time but it's ok as we are managing it
    // inner: UnsafeCell<InnerBufferPoolManager>,

    /// Pointer to the disk scheduler.
    /// This is mutex to avoid writing and reading the same page twice
    disk_scheduler: Arc<Mutex<DiskScheduler>>,

    /** Pointer to the log manager. Please ignore this for P1. */
    // LogManager *log_manager_ __attribute__((__unused__));
    #[allow(unused)]
    log_manager: Option<LogManager>,

    inner: Mutex<InnerBufferPoolManager>,

    /// Pending fetch requests from disk
    pending_fetch_requests: Mutex<HashMap<PageId, Future<()>>>,

    /// Statistics on buffer pool
    stats: BufferPoolManagerStats,
}

unsafe impl Sync for BufferPoolManager {}

struct InnerBufferPoolManager {
    /** Array of buffer pool pages. */
    // The index is the frame_id
    pages: Vec<Page>,

    /// Page table for keeping track of buffer pool pages.
    ///
    /// ## Original type:
    /// ```cpp
    /// std::unordered_map<page_id_t, frame_id_t> page_table_;
    /// ```
    ///
    /// this is a thread safe hashmap
    page_table: HashMap<PageId, FrameId>,

    /// Replacer to find unpinned pages for replacement.
    /// TODO - change type to just implement Replacer
    replacer: LRUKReplacerImpl,

    /// List of free frames that don't have any pages on them.
    // std::list<frame_id_t> free_list_;
    free_list: LinkedList<FrameId>,
}

impl BufferPoolManager {
    pub fn new(
        pool_size: usize,
        disk_manager: Arc<Mutex<(impl DiskManager + 'static)>>,
        replacer_k: Option<usize>,
        log_manager: Option<LogManager>,
    ) -> Arc<Self> {
        // Initially, every page is in the free list.
        let mut free_list = LinkedList::new();

        for i in 0..pool_size {
            free_list.push_back(i as i32)
        }

        let this = BufferPoolManager {
            next_page_id: AtomicPageId::new(0),
            pool_size,

            root_level_latch: Mutex::new(()),

            log_manager,

            inner: Mutex::new(InnerBufferPoolManager {

                // we allocate a consecutive memory space for the buffer pool
                // TODO - avoid having lock here as well
                pages: Vec::with_capacity(pool_size),

                replacer: LRUKReplacerImpl::new(
                    pool_size,
                    replacer_k.unwrap_or(LRUK_REPLACER_K),
                ),

                // TODO - remove mutex
                page_table: HashMap::with_capacity(pool_size),
                free_list,
            }),

            disk_scheduler: Arc::new(Mutex::new(DiskScheduler::new(disk_manager))),

            pending_fetch_requests: Mutex::new(HashMap::new()),

            stats: BufferPoolManagerStats::default(),
        };

        Arc::new(this)
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
            inner.replacer.evict().ok_or(errors::NoAvailableFrameFound)
        }
    }

    fn request_read_page(disk_scheduler: &mut DiskScheduler, page_id: PageId, page_data: &mut PageData) -> Future<bool> {
        let data = unsafe { UnsafeSingleRefMutData::new(page_data) };

        let promise = Promise::new();
        let future = promise.get_future();
        let req = ReadDiskRequest::new(page_id, data, promise);

        disk_scheduler.schedule(req.into());

        future
    }

    fn request_write_page(disk_scheduler: &mut DiskScheduler, page_id: PageId, page_data: &PageData) -> Future<bool> {
        let data = unsafe { UnsafeSingleRefData::new(page_data) };
        let promise = Promise::new();
        let future = promise.get_future();
        let req = WriteDiskRequest::new(page_id, data, promise);

        disk_scheduler.schedule(req.into());

        future
    }

    fn wait_for_pending_request_page_to_finish(&self, requests_map: &Mutex<HashMap<PageId, Future<()>>>, page_id: PageId) -> MutexGuard<InnerBufferPoolManager> {
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

    fn finish_current_pending_fetch_page_request(&self, page_id: PageId, fetch_promise: Promise<()>) {
        // First removing the pending requests
        self.pending_fetch_requests.lock().remove(&page_id);

        // Then release the lock
        fetch_promise.set_value(());
    }

    fn fetch_page<PageAndGuardImpl: PageAndGuard, R, F: FnOnce(Arc<Self>, PageAndGuardImpl) -> R>(self: &Arc<Self>, page_id: PageId, access_type: AccessType, create_guard: F) -> Result<R, errors::FetchPageError> {
        // Find available frame

        // 1. Hold replacer guard as all pin and unpin must first hold the replacer to avoid getting replaced in the middle
        let mut inner = self.wait_for_pending_fetch_page_to_finish(page_id);

        // We have 3 cases when fetching page
        // 1. page already exists in the buffer pool
        // 2. page does not exist in the buffer pool and we DO NOT NEED to flush existing page
        // 3. page does not exist in the buffer pool and we NEED to flush existing page
        // 3. Page exists in the buffer pool
        if let Some(&frame_id) = inner.page_table.get(&page_id) {
            // 3.1. Record access to the frame
            inner.replacer.record_access(frame_id, access_type);

            // 3.2. Avoid evicting the frame
            inner.replacer.set_evictable(frame_id, false);

            // 3.3 Get page
            let page = inner.pages[frame_id as usize].clone();

            // 3.4 Assert page table correctness
            // TODO - fix this?
            // assert_eq!(page.get_page_id(), page_id, "Page ID must be the same as the requested");

            // 3.5 Pin before returning to the caller to avoid page to be evicted in the meantime
            page.pin();

            // 3.6 Drop all locks before waiting for the page read lock
            drop(inner);

            let page_and_guard = PageAndGuardImpl::from(page);

            return Ok(create_guard(self.clone(), page_and_guard));
        }

        // Option 2, page does not exists in the buffer pool

        // 4. Find replacement frame
        let frame_id = self.find_replacement_frame(&mut inner)?;

        // 5. Create promise for when the entire fetch is finished
        let current_fetch_promise = Promise::new();

        // 6. Register the promise
        self.pending_fetch_requests.lock().insert(page_id, current_fetch_promise.get_future());

        // 7. Record access so the frame will be inserted and the replacer algorithm will work
        inner.replacer.record_access(frame_id, access_type);

        // 8. Avoid evicting the frame in the meantime
        inner.replacer.set_evictable(frame_id, false);

        // 9. Register the requested page in the page table
        inner.page_table.insert(page_id, frame_id);

        // We now have 2 options:
        // 1. the frame we got is currently have a page in it that we need to replace (and possibly flush)
        // 2. the frame we got is empty frame (we got it from the free list)

        let page_to_replace: Option<Page> = inner.pages.get_mut(frame_id as usize).cloned();

        // 7. If frame match existing page
        if let Some(page_to_replace) = page_to_replace {
            // Option 1, replacing existing frame

            let page_to_replace_backup = page_to_replace.clone();

            // 1. Get write lock, the page to replace must never have write lock created outside the buffer pool as it is about to be pinned
            let mut page_to_replace_guard = PageAndWriteGuard::from(page_to_replace);

            // 2. Pin page
            page_to_replace_guard.page_ref().pin();

            // 3. Remove the old page from the page table so it won't be available
            inner.page_table.remove(&page_to_replace_guard.get_page_id());

            // 4. If page to replace is dirty, need to flush it
            if page_to_replace_guard.page_ref().is_dirty() {
                // 5. Get the scheduler
                let mut scheduler = self.disk_scheduler.lock();

                // 6. Add flush message to the scheduler
                let flush_page_future = BufferPoolManager::request_write_page(&mut scheduler, page_to_replace_guard.get_page_id(), page_to_replace_guard.get_data());

                // TODO - only request read if the write page was successful
                //        as otherwise, if the write failed we will read and lose the data
                let fetch_page_future = BufferPoolManager::request_read_page(&mut scheduler, page_id, page_to_replace_guard.get_data_mut());

                // 8. release all locks as we don't want to hold the entire lock while flushing to disk
                drop(scheduler);
                drop(inner);

                // 9. Wait for the flush to finish
                // TODO - handle errors in flushing
                let flush_page_result = flush_page_future.wait();

                if !flush_page_result {
                    self.finish_current_pending_fetch_page_request(page_id, current_fetch_promise);

                    // TODO - reset page in page table
                    panic!("Must be able to flush page");
                }

                // 9.1. Reset page to the current page
                page_to_replace_guard.page_ref().set_is_dirty(false);

                // 10. Wait for the fetch to finish
                let fetch_page_result = fetch_page_future.wait();

                if !fetch_page_result {
                    self.finish_current_pending_fetch_page_request(page_id, current_fetch_promise);

                    panic!("Must be able to fetch page");
                }

                // 11. Set page id to be the correct page id
                page_to_replace_guard.page_ref().set_is_dirty(false);
                page_to_replace_guard.partial_reset(page_id);

                // Convert write lock to the desired lock
                drop(page_to_replace_guard);
                let page_to_replace_requested_guard = PageAndGuardImpl::from(page_to_replace_backup);

                self.finish_current_pending_fetch_page_request(page_id, current_fetch_promise);

                Ok(create_guard(self.clone(), page_to_replace_requested_guard))
            } else {
                // If page is not dirty we can just fetch the page

                // 5. Get the scheduler
                let mut scheduler = self.disk_scheduler.lock();

                // 6. Request read page
                let fetch_page_future = BufferPoolManager::request_read_page(&mut scheduler, page_id, page_to_replace_guard.get_data_mut());

                // 7. release all locks as we don't want to hold the entire lock while flushing to disk
                drop(scheduler);
                drop(inner);

                // 8. Wait for the fetch to finish
                let fetch_page_result = fetch_page_future.wait();

                if !fetch_page_result {
                    self.finish_current_pending_fetch_page_request(page_id, current_fetch_promise);

                    panic!("Must be able to fetch page");
                }

                // 11. Set page id to be the correct page id
                page_to_replace_guard.page_ref().set_is_dirty(false);
                page_to_replace_guard.partial_reset(page_id);

                // Convert write lock to the requested guard
                drop(page_to_replace_guard);
                let page_to_replace_requested_guard = PageAndGuardImpl::from(page_to_replace_backup);

                self.finish_current_pending_fetch_page_request(page_id, current_fetch_promise);

                Ok(create_guard(self.clone(), page_to_replace_requested_guard))
            }
        } else {
            // If no page exists in the pages for the current frame, just fetch it

            // 1. Create new page
            let page = Page::new(page_id);

            // 2. Pin page
            page.pin();

            // 5. Get the scheduler
            let mut scheduler = self.disk_scheduler.lock();

            // 6. Request read page
            let fetch_page_future = BufferPoolManager::request_read_page(&mut scheduler, page_id, page.write().get_data_mut());

            // 7. release all locks as we don't want to hold the entire lock while flushing to disk
            drop(scheduler);
            drop(inner);

            // 8. Wait for the fetch to finish
            let fetch_page_result = fetch_page_future.wait();

            if !fetch_page_result {
                self.finish_current_pending_fetch_page_request(page_id, current_fetch_promise);

                panic!("Must be able to fetch page");
            }

            let requested_guard = PageAndGuardImpl::from(page);

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
    pub(super) fn unpin_page(&self, page_id: PageId, access_type: AccessType) -> bool {
        // 1. first hold the replacer
        let mut inner = self.inner.lock();

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
                inner.replacer.set_evictable(frame_id, true);
            }

            true
        } else {
            false
        }
    }

    pub fn get_stats(&self) -> &BufferPoolManagerStats {
        &self.stats
    }
}

impl BufferPool for Arc<BufferPoolManager> {
    fn get_pool_size(&self) -> usize {
        self.pool_size
    }

    fn new_page<'a>(&self, access_type: AccessType) -> Result<PageWriteGuard<'a>, errors::NewPageError> {
        // Find available frame

        // 1. Hold replacer guard as all pin and unpin must first hold the replacer to avoid getting replaced in the middle
        let mut inner = self.inner.lock();

        // 2. Find replacement frame
        let frame_id = self.find_replacement_frame(&mut inner)?;

        // 3. Record access so the frame will be inserted and the replacer algorithm will work
        inner.replacer.record_access(frame_id, access_type);

        // 4. Avoid evicting the frame in the meantime
        inner.replacer.set_evictable(frame_id, false);

        // 5. Allocate page id
        let page_id = self.allocate_page();

        // 6. Register the new page in the page table
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
            page_and_write.page_ref().pin();

            // 3. Remove the old page from the page table so it won't be available
            inner.page_table.remove(&page_and_write.get_page_id());

            // 4. If page to replace is dirty, need to flush it
            if page_and_write.page_ref().is_dirty() {
                // 5. Get the scheduler
                let mut scheduler = self.disk_scheduler.lock();

                // 6. Add flush message to the scheduler
                let flush_page_future = BufferPoolManager::request_write_page(&mut scheduler, page_and_write.get_page_id(), page_and_write.get_data());

                // 7. release all locks as we don't want to hold the entire lock while flushing to disk
                drop(scheduler);
                drop(inner);

                // 8. Wait for the flush to finish
                // TODO - handle errors in flushing
                assert_eq!(flush_page_future.wait(), true, "Must be able to flush pages");

                // 9. Reset dirty
                page_and_write.page_ref().set_is_dirty(false);
            }

            // 5. Reset page
            // 6. Change page id to be this page id
            page_and_write.page_ref().set_is_dirty(false);
            page_and_write.reset(page_id);

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
        let mut inner = self.inner.lock();

        if !inner.page_table.contains_key(&page_id) {
            // Page is missing
            return false;
        }

        let &frame_id = inner.page_table.get(&page_id).unwrap();

        let page = inner.pages[frame_id as usize].clone();

        if page.is_locked_exclusive() {
            panic!("Possible deadlock detected when trying to flush page {} when the page has already exclusive lock", page_id);
        }

        // Avoid evicting in the middle
        inner.replacer.set_evictable(frame_id, false);
        page.pin();

        let mut scheduler = self.disk_scheduler.lock();

        // Add flush message to the scheduler
        let flush_page_future = BufferPoolManager::request_write_page(&mut scheduler, page_id, page.read().get_data());

        // release all locks as we don't want to hold the entire lock while flushing to disk
        drop(scheduler);
        drop(inner);

        assert_eq!(flush_page_future.wait(), true, "Must be able to flush page");

        page.set_is_dirty(false);

        self.unpin_page(page_id, AccessType::Unknown);

        true
    }

    fn flush_all_pages(&self) {
        todo!()
    }

    fn delete_page(&self, page_id: PageId) -> Result<bool, DeletePageError> {
        if page_id == INVALID_PAGE_ID {
            return Err(DeletePageError::InvalidPageId);
        }

        let mut inner = self.inner.lock();

        if !inner.page_table.contains_key(&page_id) {
            // Page is missing
            return Ok(true);
        }

        let &frame_id = inner.page_table.get(&page_id).unwrap();
        let page = inner.pages[frame_id as usize].clone();

        if page.is_pinned() {
            return Err(DeletePageError::PageIsNotEvictable(page_id));
        }

        // TODO - what about if page is dirty?
        (*inner).page_table.remove(&page_id);
        (*inner).free_list.push_front(frame_id);

        (*inner).replacer.remove(frame_id);

        page.with_write(|u| {
            // Do not remove the item, and instead change it to INVALID_PAGE_ID
            // so we won't change the frame location
            u.reset(INVALID_PAGE_ID)
        });
        page.set_is_dirty(false);

        BufferPoolManager::deallocate_page(self, page_id);

        Ok(true)
    }

    fn get_pin_count(&self, page_id: PageId) -> Option<usize> {
        // 1. first hold the replacer
        let inner = self.inner.lock();

        // 2. check if the page table the page exists

        if let Some(&frame_id) = inner.page_table.get(&page_id) {

            // 3. Get the page to unpin
            Some(inner.pages[frame_id as usize].get_pin_count())
        } else {
            None
        }
    }
}
