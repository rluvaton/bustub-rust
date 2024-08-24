use crate::buffer_pool_manager::manager::{BufferPoolManager, InnerBufferPoolManager};
use crate::lru_k_replacer::{AccessType, LRUKReplacer};
use common::config::{AtomicPageId, FrameId, PageId, INVALID_PAGE_ID, LRUK_REPLACER_K};
use common::Promise;
use log::warn;
use parking_lot::Mutex;
use recovery::LogManager;
use std::cell::{Cell, UnsafeCell};
use std::collections::{HashMap, HashSet, LinkedList};
use std::ops::{Deref, DerefMut};
use std::sync::atomic::Ordering;
use std::sync::Arc;
use storage::{BasicPageGuard, DiskManager, DiskScheduler, Page, ReadDiskRequest, ReadPageGuard, UnderlyingPage, WriteDiskRequest, WritePageGuard};

impl BufferPoolManager {
    pub fn new(
        pool_size: usize,
        disk_manager: Arc<Mutex<(impl DiskManager + 'static)>>,
        replacer_k: Option<usize>,
        log_manager: Option<LogManager>,
    ) -> Self {

        // TODO(students): remove this line after you have implemented the buffer pool manager
        // unimplemented!(
        //     "BufferPoolManager is not implemented yet. If you have finished implementing BPM, please remove the throw "
        //     "exception line in `buffer_pool_manager.cpp`.");


        // Initially, every page is in the free list.
        let mut free_list = LinkedList::new();

        for i in 0..pool_size {
            free_list.push_back(i as i32)
        }

        BufferPoolManager {
            pool_size,

            root_level_latch: Mutex::new(()),

            inner: UnsafeCell::new(InnerBufferPoolManager {
                next_page_id: AtomicPageId::new(0),
                log_manager: log_manager,

                // we allocate a consecutive memory space for the buffer pool
                // TODO - change to array
                pages: Vec::with_capacity(pool_size),

                replacer: LRUKReplacer::new(
                    pool_size,
                    replacer_k.unwrap_or(LRUK_REPLACER_K),
                ),

                disk_scheduler: Arc::new(Mutex::new(DiskScheduler::new(disk_manager))),
                page_table: HashMap::with_capacity(pool_size),
                free_list,

            }),

        }
    }

    /** @brief Return the size (number of frames) of the buffer pool. */
    pub fn get_pool_size(&self) -> usize {
        self.pool_size
    }

    /** @brief Return the pointer to all the pages in the buffer pool. */
    pub fn get_pages(&self) -> &Vec<Page> {
        // &self.latch.lock().pages
        unimplemented!()
    }

    /**
     * TODO(P1): Add implementation
     *
     * @brief Create a new page in the buffer pool. Set page_id to the new page's id, or nullptr if all frames
     * are currently in use and not evictable (in another word, pinned).
     *
     * You should pick the replacement frame from either the free list or the replacer (always find from the free list
     * first), and then call the allocate_page() method to get a new page id. If the replacement frame has a dirty page,
     * you should write it back to the disk first. You also need to reset the memory and metadata for the new page.
     *
     * Remember to "Pin" the frame by calling replacer.SetEvictable(frame_id, false)
     * so that the replacer wouldn't evict the frame before the buffer pool manager "Unpin"s it.
     * Also, remember to record the access history of the frame in the replacer for the lru-k algorithm to work.
     *
     * Original: @param[out] page_id id of created page, (--- no need as can get from the page itself ---)
     * Original: @return nullptr if no new pages could be created, otherwise pointer to new page
     */
    pub fn new_page(&self) -> Option<Page> {
        // First acquire the lock for thread safety
        let root_latch_guard = self.root_level_latch.lock();

        // Get the inner data
        let inner = self.inner.get();

        unsafe {
            let frame_id = Self::find_replacement_frame(&mut (*inner))?;

            // Add to The Replacement Policy
            (*inner).replacer.with_lock(|replacer| {
                // Record access so the frame will be inserted and the LRU-k algorithm will work
                replacer.record_access(frame_id, AccessType::Unknown);

                // Then set evictable state after frame inserted
                // "Pin" the frame so that the replacer wouldn't evict the frame before the buffer pool manager "Unpin"s it.
                replacer.set_evictable(frame_id, false);
            });

            // Get new page id
            let new_page_id = Self::allocate_page(&mut *inner);

            // Add it to the page id to the frame mapping
            (*inner).page_table.insert(new_page_id, frame_id);

            let old_page: Option<&mut Page> = (*inner).pages.get_mut(frame_id as usize);

            // If we evicted and we have old page we will release the root lock to avoid blocking other processes while we flush the old page to disk
            if let Some(old_page) = old_page {

                // 1. Hold writable lock on the page to make sure no one can access the page content before we finish
                //    replacing the old page content with the new one
                old_page.with_write(|mut underlying| {
                    // 1. Remove reference to the old page in the page table
                    (*inner).page_table.remove(&underlying.get_page_id());

                    // #############################################################################
                    //                              Root Lock Release
                    // #############################################################################
                    // 2. Once we hold the lock we can release the root lock
                    // Until we finish flushing we don't want to allow fetching the old page id,
                    // and it will not as the disk scheduler is behind a Mutex
                    drop(root_latch_guard);

                    // 3. Flush the old page content if it's dirty
                    if underlying.is_dirty() {
                        Self::flush_specific_page_unchecked(&mut *inner, &mut underlying);
                    }

                    // 3. Create new page
                    underlying.reset(new_page_id);

                    // 4. Pin the old page as it will be our new page
                    underlying.increment_pin_count_unchecked();
                });

                return Some(old_page.clone());
            }

            // In new page we're holding the root lock until we finish creating the new page

            // If we have a new page, create it
            let mut new_page = Page::new(new_page_id);

            // Add to the frame table
            (*inner).pages.insert(frame_id as usize, new_page.clone());

            // Pin before returning to the caller to avoid page to be evicted in the meantime
            new_page.pin();

            Some(new_page)
        }
    }

    /**
     * TODO(P2): Add implementation
     *
     * @brief PageGuard wrapper for new_page
     *
     * Functionality should be the same as new_page, except that
     * instead of returning a pointer to a page, you return a
     * BasicPageGuard structure.
     *
     * @param[out] page_id, the id of the new page
     * @return BasicPageGuard holding a new page
     */
    pub fn new_page_guarded(&self, page_id: PageId) -> BasicPageGuard {
        unimplemented!()
    }

    /**
     * TODO(P1): Add implementation
     *
     * @brief Fetch the requested page from the buffer pool. Return nullptr if page_id needs to be fetched from the disk
     * but all frames are currently in use and not evictable (in another word, pinned).
     *
     * First search for page_id in the buffer pool. If not found, pick a replacement frame from either the free list or
     * the replacer (always find from the free list first), read the page from disk by scheduling a read DiskRequest with
     * disk_scheduler_->Schedule(), and replace the old page in the frame. Similar to new_page(), if the old page is dirty,
     * you need to write it back to disk and update the metadata of the new page
     *
     * In addition, remember to disable eviction and record the access history of the frame like you did for new_page().
     *
     * @param page_id id of page to be fetched
     * @param access_type type of access to the page, only needed for leaderboard tests. - TODO - default for  = AccessType::Unknown
     * @return nullptr if page_id cannot be fetched, otherwise pointer to the requested page
     */
    pub fn fetch_page(&self, page_id: PageId, access_type: AccessType) -> Option<Page> {
        if page_id == INVALID_PAGE_ID {
            return None;
        }

        self.fetch_page_unchecked(page_id, access_type)
    }

    fn fetch_page_unchecked(&self, page_id: PageId, access_type: AccessType) -> Option<Page> {
        assert_ne!(page_id, INVALID_PAGE_ID);

        // First acquire the lock for thread safety
        let root_latch_guard = self.root_level_latch.lock();

        unsafe {
            // Get the inner data
            let inner = self.inner.get();

            // First search for page_id in the buffer pool
            if let Some(&frame_id) = (*inner).page_table.get(&page_id) {
                // Page exists in the table

                // Prevent the frame to be evictable
                (*inner).replacer.with_lock(|replacer| {
                    replacer.set_evictable(frame_id, false);
                    replacer.record_access(frame_id, access_type);
                });

                let mut p = (*inner).pages[frame_id as usize].clone();

                // Pin before returning to the caller to avoid page to be evicted in the meantime
                p.pin();

                // We keep the root lock while we have the page id in the buffer pool as it's not a long operation
                return Some(p);
            }

            // Need to fetch from disk

            let frame_id = Self::find_replacement_frame(&mut (*inner))?;

            // Add to The Replacement Policy
            (*inner).replacer.with_lock(|replacer| {
                // Record access so the frame will be inserted and the LRU-k algorithm will work
                replacer.record_access(frame_id, AccessType::Unknown);

                // Then set evictable state after frame inserted
                // "Pin" the frame so that the replacer wouldn't evict the frame before the buffer pool manager "Unpin"s it.
                replacer.set_evictable(frame_id, false);
            });

            // Add it to the page id to the frame mapping
            (*inner).page_table.insert(page_id, frame_id);

            let old_page: Option<&mut Page> = (*inner).pages.get_mut(frame_id as usize);

            // If we evicted and we have old page we will release the root lock to avoid blocking other processes while we flush the old page to disk
            if let Some(old_page) = old_page {
                // 1. Hold writable lock on the page to make sure no one can access the page content before we finish
                //    replacing the old page content with the new one
                old_page.with_write(|mut underlying| {
                    // 1. Remove reference to the old page in the page table
                    (*inner).page_table.remove(&underlying.get_page_id());

                    // #############################################################################
                    //                              Root Lock Release
                    // #############################################################################
                    // 2. Once we hold the lock we can release the root lock
                    // Until we finish flushing we don't want to allow fetching the old page id,
                    // and it will not as the disk scheduler is behind a Mutex
                    drop(root_latch_guard);

                    // 3. Flush the old page content if it's dirty
                    if underlying.is_dirty() {
                        Self::flush_specific_page_unchecked(&mut *inner, &mut underlying);
                    }

                    // 3. Create new page
                    underlying.partial_reset(page_id);

                    // 4. Pin the old page as it will be our new page
                    underlying.increment_pin_count_unchecked();

                    // 5. Fetch data from disk
                    Self::fetch_specific_page_unchecked(&mut *inner, underlying);
                });

                return Some(old_page.clone());
            }

            // We create a new page

            let mut page: Page = Page::new(page_id);

            // 1. Pin the page
            page.pin();

            // 2. Add to the page table
            (*inner).page_table.insert(page_id, frame_id);

            // 3. Add to the frame list
            (*inner).pages.insert(frame_id as usize, page.clone());

            page.with_write(|mut underlying| {
                // #############################################################################
                //                              Root Lock Release
                // #############################################################################
                // 1. Once we hold the lock we can release the root lock
                // Until we finish flushing we don't want to allow fetching the old page id,
                // and it will not as the disk scheduler is behind a Mutex
                drop(root_latch_guard);

                Self::fetch_specific_page_unchecked(&mut *inner, &mut underlying);
            });

            Some(page)
        }
    }

    fn fetch_specific_page_unchecked(inner: &InnerBufferPoolManager, page: &mut UnderlyingPage) {
        // TODO - this will clone the data rather than passing it so it will write on it
        // TODO - fix this -
        // TODO - ##################### should update in place
        let data = Arc::new(Mutex::new(*page.get_data()));
        let promise = Promise::new();
        let future = promise.get_future();
        let req = ReadDiskRequest::new(page.get_page_id(), Arc::clone(&data), promise);

        let mut scheduler = inner.disk_scheduler.lock();
        scheduler.schedule(req.into());

        // TODO - should wait for X ms and then timeout?
        assert_eq!(future.wait(), true, "Should be able to fetch");

        page.set_data(*data.lock());
    }

    /**
     * TODO(P2): Add implementation
     *
     * @brief PageGuard wrappers for fetch_page
     *
     * Functionality should be the same as fetch_page, except
     * that, depending on the function called, a guard is returned.
     * If FetchPageRead or FetchPageWrite is called, it is expected that
     * the returned page already has a read or write latch held, respectively.
     *
     * @param page_id, the id of the page to fetch
     * @return PageGuard holding the fetched page
     */
    pub fn fetch_page_basic(&self, page_id: PageId) -> BasicPageGuard {
        unimplemented!()
    }
    pub fn fetch_page_read(&self, page_id: PageId) -> ReadPageGuard {
        unimplemented!()
    }
    pub fn fetch_page_write(&self, page_id: PageId) -> WritePageGuard {
        unimplemented!()
    }

    /**
     * TODO(P1): Add implementation
     *
     * @brief Unpin the target page from the buffer pool. If page_id is not in the buffer pool or its pin count is already
     * 0, return false.
     *
     * Decrement the pin count of a page. If the pin count reaches 0, the frame should be evictable by the replacer.
     * Also, set the dirty flag on the page to indicate if the page was modified.
     *
     * @param page_id id of page to be unpinned
     * @param is_dirty true if the page should be marked as dirty, false otherwise
     * @param access_type type of access to the page, only needed for leaderboard tests. TODO - default  = AccessType::Unknown
     * @return false if the page is not in the page table or its pin count is <= 0 before this call, true otherwise
     */
    pub fn unpin_page(&self, page_id: PageId, is_dirty: bool, _access_type: AccessType) -> bool {
        // First acquire the lock for thread safety
        let _root_latch_guard = self.root_level_latch.lock();

        let inner = self.inner.get() ;


        unsafe {
            let frame_id = (*inner).page_table.get(&page_id);

            // If page_id is not in the buffer pool, return false
            if frame_id.is_none() {
                return false;
            }

            let frame_id = frame_id.unwrap();
            let frame_id_ref = *frame_id;

            let page: Option<&mut Page> = (*inner).pages.get_mut(frame_id_ref as usize);

            if page.is_none() {
                warn!("Could not find requested page to unpin, it shouldn't be possible");
                // TODO - log warning or something as this mean we have corruption
                return false;
            }

            let page = page.unwrap();

            // Also, set the dirty flag on the page to indicate if the page was modified.
            if is_dirty {
                page.with_write(|u| u.set_is_dirty(true));
            }

            let pin_count_before_unpin = page.get_pin_count();

            // If page's pin count is already 0, return false
            if pin_count_before_unpin == 0 {
                return false;
            }

            page.unpin();

            // If pin count reaches 0, mark as evictable
            if pin_count_before_unpin == 1 {
                (*inner).replacer.set_evictable(frame_id_ref, true);
            }

            true
        }
    }

    /**
     * TODO(P1): Add implementation
     *
     * @brief Flush the target page to disk.
     *
     * Use the DiskManager::WritePage() method to flush a page to disk, REGARDLESS of the dirty flag.
     * Unset the dirty flag of the page after flushing.
     *
     * @param page_id id of page to be flushed, cannot be INVALID_PAGE_ID
     * @return false if the page could not be found in the page table, true otherwise
     */
    pub fn flush_page(&mut self, page_id: PageId) -> bool {
        assert_ne!(page_id, INVALID_PAGE_ID);

        let root_latch_guard = self.root_level_latch.lock();

        let inner = self.inner.get_mut();

        let frame_id = inner.page_table.get(&page_id);

        if frame_id.is_none() {
            return false;
        }

        let frame_id = *frame_id.unwrap();

        let page = &inner.pages[frame_id as usize];

        page.with_write(|u| {
            // #############################################################################
            //                              Root Lock Release
            // #############################################################################
            // 1. Once we hold the lock we can release the root lock
            // Until we finish flushing we don't want to allow fetching the old page id,
            // and it will not as the disk scheduler is behind a Mutex
            drop(root_latch_guard);

            Self::flush_specific_page_unchecked(inner, u)
        });

        true
    }


    fn flush_specific_page_unchecked(inner: &InnerBufferPoolManager, page: &mut UnderlyingPage) {
        let data = Arc::new(*page.get_data());
        let promise = Promise::new();
        let future = promise.get_future();
        let req = WriteDiskRequest::new(page.get_page_id(), data, promise);

        let mut scheduler = inner.disk_scheduler.lock();
        scheduler.schedule(req.into());

        // TODO - should wait for X ms and then timeout?
        assert_eq!(future.wait(), true, "Should be able to write");

        page.set_is_dirty(false);
    }

    /**
     * TODO(P1): Add implementation
     *
     * @brief Flush all the pages in the buffer pool to disk.
     */
    pub fn flush_all_pages(&mut self) {
        // TODO - should acquire lock?
        // let mut inner = unsafe { self.inner.get_mut() };
        //
        // (*inner).page_table.keys().for_each(|page_id| {
        //     Self::flush_page(self, page_id.clone());
        // });
        unimplemented!()
    }

    /**
     * TODO(P1): Add implementation
     *
     * @brief Delete a page from the buffer pool. If page_id is not in the buffer pool, do nothing and return true. If the
     * page is pinned and cannot be deleted, return false immediately.
     *
     * After deleting the page from the page table, stop tracking the frame in the replacer and add the frame
     * back to the free list. Also, reset the page's memory and metadata. Finally, you should call deallocate_page() to
     * imitate freeing the page on the disk.
     *
     * @param page_id id of page to be deleted
     * @return false if the page exists but could not be deleted, true if the page didn't exist or deletion succeeded
     */
    pub fn delete_page(&mut self, page_id: PageId) -> bool {

        let _root_latch_guard = self.root_level_latch.lock();

        let inner = self.inner.get_mut();

        let frame_id_value: FrameId;
        {
            // TODO - should lock
            let frame_id = inner.page_table.get(&page_id);

            if frame_id.is_none() {
                return true;
            }
            frame_id_value = frame_id.cloned().unwrap();
        }

        let page: &mut Page = inner.pages.get_mut(frame_id_value as usize).expect("page must exists as it is in the page table");

        // If not evictable
        if page.get_pin_count() > 0 {
            return false;
        }

        // TODO - what about if page is dirty?

        inner.page_table.remove(&page_id);
        inner.free_list.push_front(frame_id_value);

        inner.replacer.remove(frame_id_value);
        inner.pages.remove(frame_id_value as usize);


        Self::deallocate_page(page_id);

        true
    }

    fn find_replacement_frame(inner: &mut InnerBufferPoolManager) -> Option<FrameId> {
        // Pick the replacement frame from the free list first
        if !inner.free_list.is_empty() {
            // Can't be empty
            inner.free_list.pop_front()
        } else {
            // pick replacement from the replacer, can't be empty
            inner.replacer.evict()
        }
    }


    /// Replace page with another page id, if `old_page`, `page_id` is the same as `new_page_id` don't do anything and return false
    fn replace_page<F: FnOnce(&mut InnerBufferPoolManager, PageId, &mut UnderlyingPage)>(inner: &mut InnerBufferPoolManager, old_page: &Page, new_page_id: PageId, new_page_reset_fn: F) -> bool {
        old_page.with_write(|mut underlying| {
            let old_page_id = underlying.get_page_id();

            if new_page_id == old_page_id {
                return false;
            }

            // Clear old reference
            inner.page_table.remove(&old_page_id);

            if underlying.is_dirty() {
                // If the replacement frame has a dirty page,
                //      * you should write it back to the disk first. You also need to reset the memory and metadata for the new page.
                Self::flush_specific_page_unchecked(inner, &mut underlying);
            }

            new_page_reset_fn(inner, new_page_id, underlying);

            true
        })
    }

    /**
     * @brief Allocate a page on disk. Caller should acquire the latch before calling this function.
     * @return the id of the allocated page
     */
    fn allocate_page(inner: &mut InnerBufferPoolManager) -> PageId {
        inner.next_page_id.fetch_add(1, Ordering::SeqCst)
    }

    /**
     * @brief Deallocate a page on disk. Caller should acquire the latch before calling this function.
     * @param page_id id of the page to deallocate
     */
    fn deallocate_page(_page_id: PageId) {
        // This is a no-nop right now without a more complex data structure to track deallocated pages
    }
}
