use super::manager::{BufferPoolManager, InnerBufferPoolManager};
use crate::buffer::{PinPageGuard, BufferPoolManagerStats, PinReadPageGuard, PinWritePageGuard, ThreadSafeReplacer, Replacer};
use crate::buffer::{AccessType, LRUKReplacer};
use crate::storage::{DiskManager, DiskScheduler, Page, PageAndWriteGuard, PageWriteGuard, ReadDiskRequest, UnderlyingPage, WriteDiskRequest};
use common::config::{AtomicPageId, FrameId, PageId, INVALID_PAGE_ID, LRUK_REPLACER_K};
use common::{Promise, UnsafeSingleRefData, UnsafeSingleRefMutData};
use parking_lot::Mutex;
use crate::recovery::LogManager;
use std::cell::{UnsafeCell};
use std::collections::{HashMap, LinkedList};
use std::ops::{Deref, DerefMut};
use std::sync::atomic::{Ordering};
use std::sync::Arc;
use std::time::Duration;
use dashmap::DashMap;
use dashmap::mapref::one::Ref;
use num_format::Locale::en;
use tracy_client::{non_continuous_frame, secondary_frame_mark, span};
use super::errors::{NewPageError, FetchPageError, NoAvailableFrameFound, DeletePageError, InvalidPageId};

// While waiting - red-ish (brighter than page lock)
const ROOT_LOCK_WAITING_COLOR: u32 = 0xEF0107;

// While holding the root lock - green-ish (brighter than page lock)
const ROOT_LOCK_HOLDING_COLOR: u32 = 0x32DE84;

// TODO - should return result rather than Option in fetch, new and more
impl BufferPoolManager {
    pub fn new(
        pool_size: usize,
        disk_manager: Arc<Mutex<(impl DiskManager + 'static)>>,
        replacer_k: Option<usize>,
        log_manager: Option<LogManager>,
    ) -> Self {
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
                log_manager,

                // we allocate a consecutive memory space for the buffer pool
                pages: Vec::with_capacity(pool_size),

                replacer: LRUKReplacer::new(
                    pool_size,
                    replacer_k.unwrap_or(LRUK_REPLACER_K),
                ),

                disk_scheduler: Arc::new(Mutex::new(DiskScheduler::new(disk_manager))),
                page_table: DashMap::with_capacity(pool_size),
                free_list,

            }),

            stats: BufferPoolManagerStats::default(),
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
    pub fn new_page(&self) -> Result<Page, NewPageError> {
        Ok(self.new_page_with_write_guard()?.page())
    }

    fn new_page_with_write_guard<'a>(&self) -> Result<PageAndWriteGuard<'a>, NewPageError> {
        let acquiring_root_latch = span!("Acquiring root latch");

        // Red color while waiting
        acquiring_root_latch.emit_color(ROOT_LOCK_WAITING_COLOR);

        // First acquire the lock for thread safety
        let waiting = self.stats.waiting_for_root_latch.create_single();
        let root_latch_guard = self.root_level_latch.lock();
        drop(waiting);
        drop(acquiring_root_latch);
        secondary_frame_mark!("new_page");

        let holding_root_latch = self.stats.holding_root_latch.create_single();
        let holding_root_latch_span = span!("Holding root latch");

        // Green color while holding
        holding_root_latch_span.emit_color(ROOT_LOCK_HOLDING_COLOR);

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
                //
                let mut old_page_with_guard: PageAndWriteGuard = PageAndWriteGuard::from(old_page.clone());



                // 1. Remove reference to the old page in the page table
                //    We can bypass the lock as no pin exists on the page
                let old_page_id = old_page.get_page_id_bypass_lock();
                (*inner).page_table.remove(&old_page_id);

                if old_page.is_dirty() {
                    let mut scheduler = (*inner).disk_scheduler.lock();

                    drop(holding_root_latch);
                    drop(holding_root_latch_span);

                    // #############################################################################
                    //                              Root Lock Release
                    // #############################################################################
                    // 2. Once we hold the lock for the page and the disk scheduler we can release the root lock
                    // Until we finish flushing we don't want to allow fetching the old page id,
                    // and it will not as the disk scheduler is behind a Mutex
                    drop(root_latch_guard);


                    // // 1. Hold writable lock on the page to make sure no one can access the page content before we finish
                    // //    replacing the old page content with the new one
                    // old_page_with_guard = PageAndWriteGuard::from(old_page.clone());

                    // 3. Flush the old page content if it's dirty
                    Self::flush_specific_page_unchecked(&mut *scheduler, old_page, old_page_with_guard.deref_mut(), old_page_id);
                } else {
                    // old_page_with_guard = PageAndWriteGuard::from(old_page);
                }

                // 3. Create new page
                Page::reset_with_write_guard(&mut old_page_with_guard, new_page_id);

                // 4. Pin the old page as it will be our new page
                old_page_with_guard.page_ref().pin();

                return Ok(old_page_with_guard);
            }

            // In new page we're holding the root lock until we finish creating the new page

            // If we have a new page, create it
            let mut new_page = Page::new(new_page_id);

            // Add to the frame table
            (*inner).pages.insert(frame_id as usize, new_page.clone());

            // Pin before returning to the caller to avoid page to be evicted in the meantime
            new_page.pin();

            let page_and_write_guard = PageAndWriteGuard::from(new_page);

            Ok(page_and_write_guard)
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
    pub fn new_page_guarded(self: &Arc<Self>) -> Result<PinPageGuard, NewPageError> {
        let page = self.new_page()?;

        Ok(PinPageGuard::new(self.clone(), page))
    }

    pub fn new_page_write_guarded<'a>(self: &Arc<Self>) -> Result<PinWritePageGuard<'a>, NewPageError> {
        let page_and_write_guard = self.new_page_with_write_guard()?;

        Ok(
            PinWritePageGuard::<'a>::from_write_guard(self.clone(), page_and_write_guard)
        )
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
    pub fn fetch_page(&self, page_id: PageId, access_type: AccessType) -> Result<Page, FetchPageError> {
        if page_id == INVALID_PAGE_ID {
            return Err(InvalidPageId.into());
        }

        Ok(self.fetch_page_unchecked(page_id, access_type)?.page())
    }

    /// This return a page with write lock being held
    fn fetch_page_unchecked(&self, page_id: PageId, access_type: AccessType) -> Result<PageAndWriteGuard, FetchPageError> {
        assert_ne!(page_id, INVALID_PAGE_ID);

        let acquiring_root_latch = span!("Acquiring root latch");

        // Red color while waiting
        acquiring_root_latch.emit_color(ROOT_LOCK_WAITING_COLOR);

        // First acquire the lock for thread safety
        let waiting = self.stats.waiting_for_root_latch.create_single();
        let mut root_latch_guard = self.root_level_latch.lock();
        drop(waiting);
        drop(acquiring_root_latch);

        let holding_root_latch = self.stats.holding_root_latch.create_single();

        // secondary_frame_mark!("fetch_page");
        let f = non_continuous_frame!("fetch_page");


        let holding_root_latch_span = span!("Holding root latch");

        // Green color while holding
        holding_root_latch_span.emit_color(ROOT_LOCK_HOLDING_COLOR);

        unsafe {
            // Get the inner data
            let inner = self.inner.get();


            // First search for page_id in the buffer pool
            if let Some(frame_id) = self.get_frame_id_by_page_id(&page_id) {
                // Page exists in the table

                // Prevent the frame to be evictable
                (*inner).replacer.with_lock(|replacer| {
                    replacer.set_evictable(frame_id, false);
                    replacer.record_access(frame_id, access_type);
                });


                let page = (*inner).pages[frame_id as usize].clone();
                // Pin before returning to the caller to avoid page to be evicted in the meantime
                page.pin();

                drop(holding_root_latch);
                drop(holding_root_latch_span);
                drop(f);

                // #############################################################################
                //                              Root Lock Release
                // #############################################################################
                // After pinned we can release the lock as nothing will change the page
                drop(root_latch_guard);

                return Ok(PageAndWriteGuard::from(page));
            }

            // Need to fetch from disk

            let frame_id = Self::find_replacement_frame(&mut (*inner))?;

            {
                let _span = span!("Update replacement policy");
                // Add to The Replacement Policy
                (*inner).replacer.with_lock(|replacer| {
                    // Record access so the frame will be inserted and the LRU-k algorithm will work
                    replacer.record_access(frame_id, AccessType::Unknown);

                    // Then set evictable state after frame inserted
                    // "Pin" the frame so that the replacer wouldn't evict the frame before the buffer pool manager "Unpin"s it.
                    replacer.set_evictable(frame_id, false);
                });
            }

            // Add it to the page id to the frame mapping

            (*inner).page_table.insert(page_id, frame_id);

            let old_page: Option<&mut Page> = (*inner).pages.get_mut(frame_id as usize);

            // If we evicted and we have old page we will release the root lock to avoid blocking other processes while we flush the old page to disk
            if let Some(old_page) = old_page {



                // 1. Hold writable lock on the page to make sure no one can access the page content before we finish
                //    replacing the old page content with the new one
                let mut old_page_with_guard = PageAndWriteGuard::from(old_page.clone());

                // 1. Remove reference to the old page in the page table
                let old_page_id = old_page.get_page_id_bypass_lock();
                (*inner).page_table.remove(&old_page_id);

                // 2. Acquire the scheduler lock
                let mut scheduler = (*inner).disk_scheduler.lock();

                drop(holding_root_latch_span);
                drop(f);
                drop(holding_root_latch);
                // #############################################################################
                //                              Root Lock Release
                // #############################################################################
                // 2. Once we hold the lock for the page and the disk scheduler we can release the root lock
                // Until we finish flushing we don't want to allow fetching the old page id,
                // and it will not as the disk scheduler is behind a Mutex
                drop(root_latch_guard);




                // 3. Flush the old page content if it's dirty
                if old_page.is_dirty() {
                    Self::flush_specific_page_unchecked(&mut *scheduler, old_page, old_page_with_guard.deref_mut(), old_page_id);
                }

                // 3. Create new page
                Page::partial_reset_with_write_guard(&mut old_page_with_guard, page_id);

                // 4. Pin the old page as it will be our new page
                old_page.pin();

                // 5. Fetch data from disk
                Self::fetch_specific_page_unchecked(&mut *scheduler, old_page_with_guard.deref_mut(), page_id);

                return Ok(old_page_with_guard);
            }

            // We create a new page

            let mut page: Page = Page::new(page_id);

            // 1. Pin the page

            // Pin before returning to the caller to avoid page to be evicted in the meantime
            page.pin();

            // 2. Add to the frame list
            (*inner).pages.insert(frame_id as usize, page.clone());

            let mut page_with_guard = PageAndWriteGuard::from(page);

            // 3. Acquire the scheduler lock
            let mut scheduler = (*inner).disk_scheduler.lock();

            drop(holding_root_latch_span);
            drop(f);
            drop(holding_root_latch);
            // #############################################################################
            //                              Root Lock Release
            // #############################################################################
            // 4. Once we hold the lock for the page and the disk scheduler we can release the root lock
            // Until we finish flushing we don't want to allow fetching the old page id,
            // and it will not as the disk scheduler is behind a Mutex
            drop(root_latch_guard);

            Self::fetch_specific_page_unchecked(&mut *scheduler, page_with_guard.deref_mut(), page_id);

            Ok(page_with_guard)
        }
    }

    #[inline(always)]
    fn fetch_specific_page_unchecked(disk_scheduler: &mut DiskScheduler, page: &mut UnderlyingPage, requested_page_id: PageId) {
        if requested_page_id == 31 {
            println!("Requested page 31");
        }
        let _fetch = span!("fetch page");

        // SAFETY:
        //
        // UnsafeSingleRefMutData: because this function hold the lock on the page we are certain that the page data reference won't
        // drop as we wait here
        //
        // get_data_mut_unchecked: we don't want to set dirty flag to true in here as we are fetching the data
        let data = unsafe { UnsafeSingleRefMutData::new(page.get_data_mut_unchecked()) };

        let promise = Promise::new();
        let future = promise.get_future();
        let req = ReadDiskRequest::new(requested_page_id, data, promise);

        disk_scheduler.schedule(req.into());

        // TODO - should wait for X ms and then timeout?
        assert_eq!(future.wait(), true, "Should be able to fetch");
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
    pub fn fetch_page_basic(self: &Arc<Self>, page_id: PageId) -> Result<PinPageGuard, FetchPageError> {
        let page = self.fetch_page(page_id, AccessType::Unknown)?;

        Ok(PinPageGuard::new(self.clone(), page))
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
    pub fn fetch_page_read<'a>(self: &'a Arc<Self>, page_id: PageId) -> Result<PinReadPageGuard<'a>, FetchPageError> {
        if page_id == INVALID_PAGE_ID {
            return Err(InvalidPageId.into());
        }

        let page_and_write_guard = self.fetch_page_unchecked(page_id, AccessType::Unknown)?;

        Ok(
            PinReadPageGuard::<'a>::from_write_guard(self.clone(), page_and_write_guard)
        )
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
    pub fn fetch_page_write<'a>(self: &'a Arc<Self>, page_id: PageId) -> Result<PinWritePageGuard<'a>, FetchPageError> {
        if page_id == INVALID_PAGE_ID {
            return Err(InvalidPageId.into());
        }

        let page_and_write_guard = self.fetch_page_unchecked(page_id, AccessType::Unknown)?;

        Ok(
            PinWritePageGuard::<'a>::from_write_guard(self.clone(), page_and_write_guard)
        )
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
        // TODO - performance improvement, Unpinning page can bypass some locks as if it was pinned nothing could evict it

        let acquiring_root_latch = span!("Acquiring root latch");

        // Red color while waiting
        acquiring_root_latch.emit_color(ROOT_LOCK_WAITING_COLOR);

        // First acquire the lock for thread safety
        let waiting = self.stats.waiting_for_root_latch.create_single();
        // TODO - Should unpinning page hold the root latch? correctness said it should as everything that touch the `inner`
        //        property should hold a latch
        let _root_latch_guard = self.root_level_latch.lock();
        drop(waiting);
        drop(acquiring_root_latch);
        let _f = non_continuous_frame!("unpin_page");

        let holding_root_latch_span = span!("Holding root latch");


        // Green color while holding
        holding_root_latch_span.emit_color(ROOT_LOCK_HOLDING_COLOR);

        // let _holding_root_latch = self.stats.holding_root_latch.create_single();


        let inner = self.inner.get();

        unsafe {
            let frame_id = self.get_frame_id_by_page_id(&page_id);

            // If page_id is not in the buffer pool, return false
            if frame_id.is_none() {
                return false;
            }

            let frame_id = frame_id.unwrap();


            let page: &mut Page = (*inner).pages.get_mut(frame_id as usize).expect("Page cannot be missing as it exists in the page table");

            // Also, set the dirty flag on the page to indicate if the page was modified.
            if is_dirty {
                page.set_is_dirty(is_dirty);
            }

            let pin_count_before_unpin = page.get_pin_count();

            // If page's pin count is already 0, return false
            if pin_count_before_unpin == 0 {
                return false;
            }

            {
                let _unpin = span!("Unpin");

                page.unpin();
            }

            // If pin count reaches 0, mark as evictable
            if pin_count_before_unpin == 1 {
                (*inner).replacer.set_evictable(frame_id, true);
            }

            true
        }
    }

    // Same implementation as unpin page except we are coming with write guard so we have some assumption we can exploit
    pub fn unpin_page_from_pinned_page(&self, pinned_page: &Page, _access_type: AccessType) -> bool {
        // TODO - I really hope this is thread safe
        let _root_latch_guard = self.root_level_latch.lock();

        // No need to hold root lock as we only touch 4 things:
        // 1. page - and we have it
        // 2. page table - we have pinned page, so nothing should
        // 3. page map - same as page table
        // 4. replacer - replacer is thread safe
        let inner = self.inner.get();


        unsafe {
            //
            let frame_id = self.get_frame_id_by_page_id(&pinned_page.get_page_id_bypass_lock());

            // If page_id is not in the buffer pool, return false
            if frame_id.is_none() {
                eprintln!("Frame is was not found based on the page, this should not be possible for pages that are tracked by the buffer pool, this might mean that we got different page");
                panic!("No frame match the requested page");
            }

            let frame_id = frame_id.unwrap();

            // Making sure the page exists
            (*inner).pages.get_mut(frame_id as usize).expect("Page cannot be missing as it exists in the page table");

            let pin_count_before_unpin = pinned_page.get_pin_count();

            // If page's pin count is already 0, return false
            if pin_count_before_unpin == 0 {
                return false;
            }

            pinned_page.unpin();

            // If pin count reaches 0, mark as evictable
            if pin_count_before_unpin == 1 {
                (*inner).replacer.set_evictable(frame_id, true);
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
    #[inline(always)]
    pub fn flush_page(&self, page_id: PageId) -> bool {
        assert_ne!(page_id, INVALID_PAGE_ID);

        let acquiring_root_latch = span!("Acquiring root latch");

        // Red color while waiting
        acquiring_root_latch.emit_color(ROOT_LOCK_WAITING_COLOR);

        // First acquire the lock for thread safety
        let waiting = self.stats.waiting_for_root_latch.create_single();
        let root_latch_guard = self.root_level_latch.lock();
        drop(waiting);
        drop(acquiring_root_latch);
        let f = non_continuous_frame!("flush_page");

        // secondary_frame_mark!("flush_page");

        let holding_root_latch_span = span!("Holding root latch");

        // Green color while holding
        holding_root_latch_span.emit_color(ROOT_LOCK_HOLDING_COLOR);
        let holding_root_latch = self.stats.holding_root_latch.create_single();

        let inner = self.inner.get();

        unsafe {
            let frame_id = self.get_frame_id_by_page_id(&page_id);

            if frame_id.is_none() {
                return false;
            }

            let frame_id = frame_id.unwrap();

            let page = &(*inner).pages[frame_id as usize];

            let old_page_id = page.get_page_id_bypass_lock();

            // 1. Acquire the scheduler lock
            let mut scheduler = (*inner).disk_scheduler.lock();

            drop(holding_root_latch_span);
            drop(f);
            drop(holding_root_latch);
            // #############################################################################
            //                              Root Lock Release
            // #############################################################################
            // 2. Once we hold the lock for the page and the disk scheduler we can release the root lock
            // Until we finish flushing we don't want to allow fetching the old page id,
            // and it will not as the disk scheduler is behind a Mutex
            drop(root_latch_guard);

            let tmp_page = page.clone();
            let mut page_write = tmp_page.write();

            Self::flush_specific_page_unchecked(&mut scheduler, page, page_write.deref_mut(), old_page_id);

            true
        }
    }

    fn flush_specific_page_unchecked(disk_scheduler: &mut DiskScheduler, page: &Page, underlying_page: &mut UnderlyingPage, page_id_to_flush: PageId) {
        let _flush_page = span!("Flush page");

        // SAFETY: because this function hold the lock on the page we are certain that the page data reference won't
        //         drop as we wait here
        let data = unsafe { UnsafeSingleRefData::new(underlying_page.get_data()) };
        let promise = Promise::new();
        let future = promise.get_future();
        let req = WriteDiskRequest::new(page_id_to_flush, data, promise);

        disk_scheduler.schedule(req.into());

        // TODO - should wait for X ms and then timeout?
        assert_eq!(future.wait(), true, "Should be able to write");

        page.set_is_dirty(false);
    }

    /**
     * TODO(P1): Add implementation
     *
     * @brief Flush all the pages in the buffer pool to disk.
     */
    pub fn flush_all_pages(&self) {
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
    pub fn delete_page(&self, page_id: PageId) -> Result<bool, DeletePageError> {
        if page_id == INVALID_PAGE_ID {
            return Err(InvalidPageId.into());
        }

        let acquiring_root_latch = span!("Acquiring root latch");

        // Red color while waiting
        acquiring_root_latch.emit_color(ROOT_LOCK_WAITING_COLOR);

        // First acquire the lock for thread safety
        let waiting = self.stats.waiting_for_root_latch.create_single();
        let _root_latch_guard = self.root_level_latch.lock();
        drop(waiting);
        drop(acquiring_root_latch);
        let _f = non_continuous_frame!("delete_page");
        // secondary_frame_mark!("delete_page");
        let holding_root_latch_span = span!("Holding root latch");

        // Green color while holding
        holding_root_latch_span.emit_color(ROOT_LOCK_HOLDING_COLOR);
        let _holding_root_latch = self.stats.holding_root_latch.create_single();

        let inner = self.inner.get();

        unsafe {
            let frame_id = self.get_frame_id_by_page_id(&page_id);

            if frame_id.is_none() {
                return Ok(true);
            }
            let frame_id = frame_id.unwrap();

            let page: &mut Page = (*inner).pages.get_mut(frame_id as usize).expect("page must exists as it is in the page table");
            let mut page_write_guard = PageAndWriteGuard::from(page.clone());

            assert_eq!(page_write_guard.get_page_id(), page_id, "Page to remove must be the same as the requested page");

            // If not evictable
            if page.get_pin_count() > 0 {
                return Err(DeletePageError::PageIsNotEvictable(page_id));
            }

            // TODO - what about if page is dirty?
            (*inner).page_table.remove(&page_id);
            (*inner).free_list.push_front(frame_id);

            (*inner).replacer.remove(frame_id);

            // Do not remove the item, and instead change it to INVALID_PAGE_ID
            // so we won't change the frame location
            Page::reset_with_write_guard(&mut page_write_guard, INVALID_PAGE_ID);

            Self::deallocate_page(page_id);

            Ok(true)
        }
    }

    fn find_replacement_frame(inner: &mut InnerBufferPoolManager) -> Result<FrameId, NoAvailableFrameFound> {
        // Pick the replacement frame from the free list first
        if !inner.free_list.is_empty() {
            // Can't be empty
            inner.free_list.pop_front().ok_or(NoAvailableFrameFound)
        } else {
            // pick replacement from the replacer, can't be empty
            inner.replacer.evict().ok_or(NoAvailableFrameFound)
        }
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
        // TODO - call disk scheduler to deallocate the page
    }

    pub fn get_stats(&self) -> BufferPoolManagerStats {
        self.stats.clone()
    }

    /// Get frame ID by page ID
    ///
    /// Having this function to avoid holding the page table entry too long as it may lead to a deadlock, so we only get the value and drop the entry
    fn get_frame_id_by_page_id(&self, page_id: &PageId) -> Option<FrameId> {
        let inner = self.inner.get();

        unsafe {
            match (*inner).page_table.get(page_id) {
                Some(entry) => Some(*entry.value()),
                None => None
            }
        }
    }
}
