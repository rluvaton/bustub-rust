use std::sync::Arc;
use parking_lot::Mutex;
use pages::{PageId, INVALID_PAGE_ID};
use buffer_pool_manager::{BufferPoolManager, PageReadGuard, PageWriteGuard};
use common::config::TableOID;
use lock_manager::LockManager;
use rid::RID;
use transaction::Transaction;
use tuple::{Tuple, TupleMeta};
use crate::storage::TablePage;

/// TODO - implement more from src/include/storage/table/table_heap.h
pub struct TableHeap {
    #[allow(unused)]
    bpm: Option<Arc<BufferPoolManager>>,

    /// Default: `INVALID_PAGE_ID`
    #[allow(unused)]
    first_page_id: PageId,

    /// Default: `INVALID_PAGE_ID`
    #[allow(unused)]
    last_page_id: Mutex<PageId>
}

impl TableHeap {

    /// Create a table heap without a transaction. (open table)
    pub fn new(buffer_pool_manager: Arc<BufferPoolManager>) -> Self {
        Self {
            bpm: Some(buffer_pool_manager),
            first_page_id: INVALID_PAGE_ID,
            last_page_id: Mutex::new(INVALID_PAGE_ID)
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
    pub fn insert_tuple(&self, meta: &TupleMeta, tuple: &Tuple, lock_mgr: Option<LockManager>, txn: Option<Arc<Transaction>>, oid: Option<TableOID>) -> Option<RID> {
        let oid = oid.unwrap_or(0);

        todo!()
    }

    /// Update the meta of a tuple.
    ///
    /// # Arguments
    ///
    /// * `meta`:  meta new tuple meta
    /// * `rid`: rid the rid of the inserted tuple
    ///
    pub fn update_tuple_meta(&self, meta: &TupleMeta, rid: &RID) {
        todo!()
    }

    /**
     * Read a tuple from the table.
     * @param rid rid of the tuple to read
     * @return the meta and tuple
     */
    pub fn get_tuple(&self, rid: RID) -> (TupleMeta, Tuple) {
        todo!()
    }

    /**
     * Read a tuple meta from the table. Note: if you want to get tuple and meta together, use `get_tuple` instead
     * to ensure atomicity.
     * @param rid rid of the tuple to read
     * @return the meta
     */
    pub fn get_tuple_meta(&self, rid: RID) -> TupleMeta {
        todo!()
    }

    /** @return the iterator of this table. When this iterator is created, it will record the current last tuple in the
      * table heap, and the iterator will stop at that point, in order to avoid halloween problem. You usually will need to
      * use this function for project 3. Given that you have already implemented your project 4 update executor as a
      * pipeline breaker, you may use `MakeEagerIterator` to test whether the update executor is implemented correctly.
      * There should be no difference between this function and `MakeEagerIterator` in project 4 if everything is
      * implemented correctly. */
    fn iter(&self) -> TableIterator {
        todo!();
    }

    /** @return the iterator of this table. The iterator will stop at the last tuple at the time of iterating. */
    fn eager_iter(&self) -> TableIterator {
        todo!()
    }

    /** @return the id of the first page of this table */
    fn get_first_page_id(&self) -> PageId { self.first_page_id }

    /**
     * Update a tuple in place. Should NOT be used in project 3. Implement your project 3 update executor as delete and
     * insert. You will need to use this function in project 4.
     * @param meta new tuple meta
     * @param tuple  new tuple
     * @param rid the rid of the tuple to be updated
     * @param check the check to run before actually update.
     */
    fn update_tuple_in_place<CheckFn: Fn(&TupleMeta, &Tuple, RID) -> bool>(&self, meta: &TupleMeta, tuple: &Tuple, rid: RID, check: Option<CheckFn>) -> bool {
        todo!()
    }

    /** For binder tests */
    fn create_empty_heap(create_table_heap: bool) -> Arc<TableHeap> {
        // The input parameter should be false in order to generate a empty heap
        assert_eq!(create_table_heap, false);

        Arc::new(TableHeap::default())
    }

    // The below functions are useful only when you want to implement abort in a way that removes an undo log from the
    // version chain. DO NOT USE THEM if you are unsure what they are supposed to do.
    //
    // And if you decide to use the below functions, DO NOT use the normal ones like `get_tuple`. Having two read locks
    // on the same thing in one thread might cause deadlocks.

    fn acquire_table_page_read_lock(&self, rid: RID) -> PageReadGuard {
        todo!()
    }

    fn acquire_table_page_write_lock(&self, rid: RID) -> PageWriteGuard {
        todo!()
    }

    fn update_tuple_in_place_with_lock_acquired(&self, meta: &TupleMeta, tuple: &Tuple, rid: RID, page: TablePage) {
        todo!()
    }

    fn get_tuple_with_lock_acquired(&self, rid: RID, page: TablePage) -> (TupleMeta, Tuple) {
        todo!()
    }

    fn get_tuple_meta_with_lock_acquired(&self, rid: RID, page: TablePage) -> TupleMeta {
        todo!()
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
