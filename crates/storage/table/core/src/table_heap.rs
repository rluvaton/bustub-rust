use std::sync::Arc;
use parking_lot::Mutex;
use buffer_common::AccessType;
use pages::{PageId, INVALID_PAGE_ID};
use buffer_pool_manager::{errors, BufferPool, BufferPoolManager, PageReadGuard, PageWriteGuard};
use common::config::TableOID;
use lock_manager::{LockManager};
use rid::RID;
use transaction::Transaction;
use tuple::{Tuple, TupleMeta};
use crate::table_iterator::TableIterator;
use crate::table_page::LARGEST_TUPLE_SIZE_WITHOUT_OVERFLOW;
use crate::TablePage;

/// TableHeap represents a physical table on disk.
/// This is just a doubly-linked list of pages.
pub struct TableHeap {
    #[allow(unused)]
    pub(super) bpm: Option<Arc<BufferPoolManager>>,

    /// Default: `INVALID_PAGE_ID`
    #[allow(unused)]
    pub(super) first_page_id: PageId,

    /// Default: `INVALID_PAGE_ID`
    #[allow(unused)]
    pub(super) last_page_id: Mutex<PageId>
}

impl TableHeap {

    /// Create a table heap without a transaction. (open table)
    pub fn new(bpm: Arc<BufferPoolManager>) -> Self {
        // Initialize the first table page.
        // TODO - return result here
        let mut guard = bpm.new_page(AccessType::default()).expect("Couldn't create a page for the table heap. Have you completed the buffer pool manager project?");
        let first_page_id = guard.get_page_id();
        
        let first_page = guard.cast_mut::<TablePage>();
        first_page.init();

        Self {
            bpm: Some(bpm),
            first_page_id,
            last_page_id: Mutex::new(first_page_id)
        }
    }


    /// Insert a tuple into the table. If the tuple is too large (>= page_size), return std::nullopt.
    ///
    /// # Arguments
    ///
    /// * `meta`: tuple meta
    /// * `tuple`: tuple to insert
    /// * `lock_mgr`:
    /// * `txn`:
    /// * `oid`:
    ///
    /// returns: Option<RID> the rid of the inserted tuple
    pub fn insert_tuple(&self, meta: &TupleMeta, tuple: &Tuple, lock_mgr: &Option<Arc<LockManager>>, txn: &Arc<Transaction>, oid: Option<TableOID>) -> Option<RID> {
        // Tuple size is too big
        if tuple.get_length() as usize >= LARGEST_TUPLE_SIZE_WITHOUT_OVERFLOW {
            return None;
        }
        
        let bpm = self.bpm.as_ref().unwrap();
        let oid = oid.unwrap_or(0);

        let mut last_page_id_guard = self.last_page_id.lock();
        let mut page_guard = bpm.fetch_page_write(*last_page_id_guard, AccessType::Unknown).expect("Should fetch page");
        loop {
            let page = page_guard.cast_mut::<TablePage>();
            if let Some(_) = page.get_next_tuple_offset(meta, tuple) {
                break
            }

            // if there's no tuple in the page, and we can't insert the tuple, then this tuple is too large.
            assert_ne!(page.get_num_tuples(), 0, "tuple is too large, cannot insert");

            let mut npg = bpm.new_page(AccessType::Unknown).expect("cannot allocate page");
            page.set_next_page_id(npg.get_page_id());

            let next_page = npg.cast_mut::<TablePage>();
            next_page.init();

            drop(page_guard);

            // acquire latch here as TSAN complains. Given we only have one insertion thread, this is fine.
            // TODO - here was new page write guard acquired

            *last_page_id_guard = npg.get_page_id();
            page_guard = npg;
        }

        let page = page_guard.cast_mut::<TablePage>();
        let slot_id = page.insert_tuple(meta, tuple)?;

        // only allow one insertion at a time, otherwise it will deadlock.
        let last_page_id = *last_page_id_guard;
        drop(last_page_id_guard);

        let rid = RID::new(last_page_id, slot_id as u32);

        #[cfg(feature = "lock_manager")]
        if let Some(lock_manager) = lock_mgr {
            assert!(
                lock_manager.lock_row(txn, lock_manager::LockMode::Exclusive, &oid, &rid),
                "failed to lock when inserting new tuple"
            )
        }

        Some(rid)
    }

    /// Update the meta of a tuple.
    ///
    /// # Arguments
    ///
    /// * `meta`:  meta new tuple meta
    /// * `rid`: rid the rid of the inserted tuple
    ///
    pub fn update_tuple_meta(&self, meta: &TupleMeta, rid: &RID) {
        // TODO - return result
        let mut page_guard = self.bpm.as_ref().expect("must have bpm").fetch_page_write(rid.get_page_id(), AccessType::Unknown).expect("should fetch page");
        page_guard.cast_mut::<TablePage>().update_tuple_meta(meta, rid);
    }

    /**
     * Read a tuple from the table.
     * @param rid rid of the tuple to read
     * @return the meta and tuple
     */
    pub fn get_tuple(&self, rid: &RID) -> (TupleMeta, Tuple) {
        // TODO - return result

        let page_guard = self.bpm.as_ref().expect("must have bpm").fetch_page_read(rid.get_page_id(), AccessType::Unknown).expect("should fetch page");
        let page = page_guard.cast::<TablePage>();

        let (meta, mut tuple) = page.get_tuple(rid);

        tuple.set_rid(*rid);

        (meta, tuple)

    }

    /**
     * Read a tuple meta from the table. Note: if you want to get tuple and meta together, use `get_tuple` instead
     * to ensure atomicity.
     * @param rid rid of the tuple to read
     * @return the meta
     */
    pub fn get_tuple_meta(&self, rid: &RID) -> TupleMeta {
        // TODO - return result

        let page_guard = self.bpm.as_ref().expect("must have bpm").fetch_page_read(rid.get_page_id(), AccessType::Unknown).expect("should fetch page");
        let page = page_guard.cast::<TablePage>();

        page.get_tuple_meta(rid)
    }

    /** @return the iterator of this table. When this iterator is created, it will record the current last tuple in the
      * table heap, and the iterator will stop at that point, in order to avoid halloween problem. You usually will need to
      * use this function for project 3. Given that you have already implemented your project 4 update executor as a
      * pipeline breaker, you may use `MakeEagerIterator` to test whether the update executor is implemented correctly.
      * There should be no difference between this function and `MakeEagerIterator` in project 4 if everything is
      * implemented correctly. */
    pub fn iter(self: Arc<Self>) -> TableIterator {
        // Lock get value and unlock
        let last_page_id = *self.last_page_id.lock();

        let num_tuples = {
            let page_guard = self.bpm.as_ref().expect("Must have bpm").fetch_page_read(last_page_id, AccessType::Unknown).expect("Must be able to fetch page");

            page_guard.cast::<TablePage>().get_num_tuples()
        };

        TableIterator::new(self.clone(), RID::new(self.first_page_id, 0), RID::new(last_page_id, num_tuples))
    }

    /** @return the iterator of this table. The iterator will stop at the last tuple at the time of iterating. */
    pub fn eager_iter(self: Arc<Self>) -> TableIterator {
        TableIterator::new(self.clone(), RID::new(self.first_page_id, 0), RID::new(INVALID_PAGE_ID, 0))
    }

    /** @return the id of the first page of this table */
    pub fn get_first_page_id(&self) -> PageId { self.first_page_id }

    /**
     * Update a tuple in place. Should NOT be used in project 3. Implement your project 3 update executor as delete and
     * insert. You will need to use this function in project 4.
     * @param meta new tuple meta
     * @param tuple  new tuple
     * @param rid the rid of the tuple to be updated
     * @param check the check to run before actually update.
     */
    pub unsafe fn update_tuple_in_place<CheckFn: Fn(&TupleMeta, &Tuple, &RID) -> bool>(&self, meta: &TupleMeta, tuple: &Tuple, rid: &RID, check: Option<CheckFn>) -> bool {

        // TODO - return result
        let mut page_guard = self.bpm.as_ref().expect("must have bpm").fetch_page_write(rid.get_page_id(), AccessType::Unknown).expect("should fetch page");
        let page = page_guard.cast_mut::<TablePage>();

        let (old_meta, old_tup) = page.get_tuple(rid);

        let valid = check.map(|f| f(&old_meta, &old_tup, rid)).unwrap_or(true);

        if valid {
            page.update_tuple_in_place(meta, tuple, rid);
        }

        valid
    }

    /** For binder tests */
    pub fn create_empty_heap(create_table_heap: bool) -> Arc<TableHeap> {
        // The input parameter should be false in order to generate a empty heap
        assert_eq!(create_table_heap, false);

        Arc::new(TableHeap::default())
    }

    // The below functions are useful only when you want to implement abort in a way that removes an undo log from the
    // version chain. DO NOT USE THEM if you are unsure what they are supposed to do.
    //
    // And if you decide to use the below functions, DO NOT use the normal ones like `get_tuple`. Having two read locks
    // on the same thing in one thread might cause deadlocks.

    pub fn acquire_table_page_read_lock(&self, rid: &RID) -> Result<PageReadGuard, errors::FetchPageError> {
        // TODO - avoid expect
        self.bpm.as_ref().expect("must have BPM").fetch_page_read(rid.get_page_id(), AccessType::Unknown)
    }

    pub fn acquire_table_page_write_lock(&self, rid: &RID) -> Result<PageWriteGuard, errors::FetchPageError> {
        self.bpm.as_ref().expect("must have BPM").fetch_page_write(rid.get_page_id(), AccessType::Unknown)
    }

    pub unsafe fn update_tuple_in_place_with_lock_acquired(&self, meta: &TupleMeta, tuple: &Tuple, rid: &RID, page: &mut TablePage) {
        page.update_tuple_in_place(meta, tuple, rid);
    }

    pub fn get_tuple_with_lock_acquired(&self, rid: &RID, page: &TablePage) -> (TupleMeta, Tuple) {
        let (meta, mut tuple) = page.get_tuple(rid);
        tuple.set_rid(*rid);

        (meta, tuple)
    }

    pub fn get_tuple_meta_with_lock_acquired(&self, rid: &RID, page: &TablePage) -> TupleMeta {
        page.get_tuple_meta(&rid)
    }
}

impl Default for TableHeap {
    fn default() -> Self {
        Self {
            bpm: None,
            first_page_id: INVALID_PAGE_ID,
            last_page_id: Mutex::new(INVALID_PAGE_ID)
        }
    }
}
