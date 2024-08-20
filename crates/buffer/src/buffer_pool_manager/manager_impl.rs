use std::collections::{HashMap, HashSet, LinkedList};
use std::ops::DerefMut;
use std::sync::Arc;
use std::sync::atomic::Ordering;
use parking_lot::Mutex;
use common::config::{AtomicPageId, FrameId, PageId, INVALID_PAGE_ID, LRUK_REPLACER_K};
use common::Promise;
use log::warn;
use recovery::LogManager;
use storage::{BasicPageGuard, DiskManager, DiskScheduler, Page, ReadPageGuard, WritePageGuard, UnderlyingPage, WriteDiskRequest, ReadDiskRequest};
use crate::buffer_pool_manager::manager::{BufferPoolManager, InnerBufferPoolManager};
use crate::lru_k_replacer::{AccessType, LRUKReplacer};

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
            latch: Mutex::new(InnerBufferPoolManager {
                next_page_id: AtomicPageId::new(0),
                log_manager,

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
    pub fn new_page(&mut self) -> Option<Page> {
        let mut inner = self.latch.lock();

        // Check if there is a frame available
        let frame_id = Self::find_replacement_frame(inner.deref_mut())?;

        inner.replacer.with_lock(|replacer| {
            // Record access so the frame will be inserted and the LRU-k algorithm will work
            replacer.record_access(frame_id, AccessType::Unknown);

            // Then set evictable state after frame inserted
            // "Pin" the frame so that the replacer wouldn't evict the frame before the buffer pool manager "Unpin"s it.
            replacer.set_evictable(frame_id, false);
        });

        // TODO - avoid having the same page twice when asked simultaneously
        // TODO - should acquire lock
        let new_page_id = Self::allocate_page(&mut inner);

        inner.page_table.insert(new_page_id, frame_id);

        let old_page: Option<&mut Page> = inner.pages.get_mut(frame_id as usize);

        // If no page was in the pages already
        if old_page.is_none() {
            // Create it
            let mut new_page = Page::new(new_page_id);

            inner.pages.insert(frame_id as usize, new_page.clone());

            // Pin before returning to the caller to avoid page to be evicted in the meantime
            new_page.pin();

            return Some(new_page);
        }

        // If already had old page, need to flush to disk
        let old_page = old_page.unwrap();
        let mut cloned = old_page.clone();

        Self::replace_page(inner.deref_mut(), &cloned, new_page_id, |_, new_page_id, underlying| {
            underlying.reset(new_page_id);
        });

        // Pin before returning to the caller to avoid page to be evicted in the meantime
        cloned.pin();

        Some(cloned)
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
    pub fn new_page_guarded(&mut self, page_id: PageId) -> BasicPageGuard {
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
    pub fn fetch_page(&mut self, page_id: PageId, access_type: AccessType) -> Option<Page> {
        if page_id == INVALID_PAGE_ID {
            return None;
        }

        unsafe { self.fetch_page_unchecked(page_id, access_type) }
    }

    unsafe fn fetch_page_unchecked(&mut self, page_id: PageId, access_type: AccessType) -> Option<Page> {
        let mut inner = self.latch.lock();

        assert_ne!(page_id, INVALID_PAGE_ID);

        // TODO - should lock?
        // First search for page_id in the buffer pool
        if let Some(&frame_id) = inner.page_table.get(&page_id) {
            // Page exists in the table

            // Prevent the frame to be evictable
            inner.replacer.with_lock(|replacer| {
                replacer.set_evictable(frame_id, false);
                replacer.record_access(frame_id, access_type);
            });

            let mut p = inner.pages[frame_id as usize].clone();

            // Pin before returning to the caller to avoid page to be evicted in the meantime
            p.pin();

            return Some(p);
        }

        // Need to fetch from disk
        let frame_id = Self::find_replacement_frame(inner.deref_mut())?;

        inner.replacer.with_lock(|replacer| {
            // Record access so the frame will be inserted and the LRU-k algorithm will work
            replacer.record_access(frame_id, AccessType::Unknown);

            // Then set evictable state after frame inserted
            // "Pin" the frame so that the replacer wouldn't evict the frame before the buffer pool manager "Unpin"s it.
            replacer.set_evictable(frame_id, false);
        });


        // TODO - cleanup and extract to helper functions

        // Different page exists on the same frame, meaning that we need to flush that page if needed
        if let Some(page) = inner.pages.get_mut(frame_id as usize) {
            let pinned_page_clone = page.clone();

            // Pin before returning to the caller to avoid page to be evicted in the meantime
            page.pin();

            // TODO - should already have a lock on the pages and page table here to avoid page removed in the meantime
            inner.page_table.insert(page_id, frame_id);

            Self::replace_page(&mut inner, &pinned_page_clone, page_id, |inner, new_page_id, underlying| {
                // Update page id if we're replacing an existing page
                underlying.replace_page_id_without_content_update(new_page_id);

                Self::fetch_specific_page_unchecked(inner, underlying);
            });

            return Some(pinned_page_clone);
        }

        // No page exists on that frame
        let mut page: Page = Page::new(page_id);

        // Pin before returning to the caller to avoid page to be evicted in the meantime
        page.pin();
        inner.page_table.insert(page_id, frame_id);
        // Insert page to pages list
        let page_to_insert = page.clone();
        inner.pages.insert(frame_id as usize, page_to_insert);

        page.with_write(|mut underlying| {
            // TODO - should already have a lock on the pages and page table here to avoid page removed in the meantime
            Self::fetch_specific_page_unchecked(&inner, &mut underlying);
        });


        Some(page)
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
    pub fn fetch_page_basic(&mut self, page_id: PageId) -> BasicPageGuard {
        unimplemented!()
    }
    pub fn fetch_page_read(&mut self, page_id: PageId) -> ReadPageGuard {
        unimplemented!()
    }
    pub fn fetch_page_write(&mut self, page_id: PageId) -> WritePageGuard {
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
    pub fn unpin_page(&mut self, page_id: PageId, is_dirty: bool, access_type: AccessType) -> bool {
        // TODO - should acquire lock?
        let mut inner = self.latch.lock();

        let frame_id = inner.page_table.get(&page_id);

        // If page_id is not in the buffer pool, return false
        if frame_id.is_none() {
            return false;
        }

        let frame_id = frame_id.unwrap();
        let frame_id_ref = *frame_id;

        // TODO not need to record access
        // self.replacer.record_access(*frame_id, access_type);

        let page: Option<&mut Page> = inner.pages.get_mut(frame_id_ref as usize);

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
            // return inner.replacer.with_lock(|replacer| {
            //     let is_evictable = unsafe { replacer.is_evictable_unchecked(frame_id_ref) };
            //
            //     replacer.set_evictable(frame_id_ref, true);
            //
            //     // If was not evictable return true as we fixed the pin count,
            //     // if it was evictable, then we did nothing and return false
            //     return !is_evictable;
            // });
            return false;
        }

        page.unpin();

        // If pin count reaches 0, mark as evictable
        if pin_count_before_unpin == 1 {
            inner.replacer.set_evictable(frame_id_ref, true);
        }

        true
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
        let inner = self.latch.lock();

        if !inner.page_table.contains_key(&page_id) {
            return false;
        }

        Self::flush_page_unchecked(&inner, page_id);

        true
    }

    fn flush_page_unchecked(inner: &InnerBufferPoolManager, page_id: PageId) {
        assert_ne!(page_id, INVALID_PAGE_ID);

        let page = &inner.pages[page_id as usize];

        page.with_write(|u| Self::flush_specific_page_unchecked(inner, u))
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
        let inner = self.latch.lock();

        inner.page_table.keys().for_each(|page_id| {
            Self::flush_page_unchecked(&inner, *page_id);
        });

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
        let mut inner = self.latch.lock();

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

        inner.page_table.remove(&frame_id_value);
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

    // TODO - for debugging
    pub fn lock(&mut self) -> &mut Self {
        return self;
    }
}
