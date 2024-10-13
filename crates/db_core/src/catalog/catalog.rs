use std::collections::HashMap;
use std::sync::Arc;
use std::sync::atomic::Ordering;
use common::config::{AtomicIndexOID, AtomicTableOID, IndexOID, TableOID};
use buffer_pool_manager::BufferPoolManager;
use crate::catalog::{IndexInfo, Schema};
use crate::catalog::table_info::TableInfo;
use crate::concurrency::LockManager;
use recovery_log_manager::LogManager;
use transaction::Transaction;
use crate::storage::TableHeap;

pub struct Catalog {
    #[allow(unused)]
    bpm: Arc<BufferPoolManager>,
    #[allow(unused)]
    lock_manager: Option<Arc<LockManager>>,
    #[allow(unused)]
    log_manager: Option<Arc<LogManager>>,

    ///
    /// Map table identifier -> table metadata.
    ///
    /// NOTE: `tables` owns all table metadata.
    ///
    #[allow(unused)]
    tables: HashMap<TableOID, Arc<TableInfo>>,

    /// Map table name -> table identifiers.
    #[allow(unused)]
    table_names: HashMap<String, TableOID>,

    /// The next table identifier to be used.
    ///
    /// Default: 0
    #[allow(unused)]
    next_table_oid: AtomicTableOID,

    /// Map index identifier -> index metadata.
    ///
    /// NOTE: that `indexes` owns all index metadata.
    #[allow(unused)]
    indexes: HashMap<IndexOID, Arc<IndexInfo>>,

    /// Map table name -> index names -> index identifiers.
    #[allow(unused)]
    index_names: HashMap<String, HashMap<String, IndexOID>>,

    /// The next index identifier to be used.
    /// Default: `0`
    #[allow(unused)]
    next_index_oid: AtomicIndexOID,
}


impl Catalog {
    pub fn new(bpm: Arc<BufferPoolManager>, lock_manager: Option<Arc<LockManager>>, log_manager: Option<Arc<LogManager>>) -> Self {
        Self {
            bpm,
            lock_manager,
            log_manager,
            tables: HashMap::new(),
            table_names: HashMap::new(),
            next_index_oid: AtomicTableOID::new(0),
            indexes: HashMap::new(),
            index_names: HashMap::new(),
            next_table_oid: AtomicIndexOID::new(0),
        }
    }


    /**
     * Create a new table and return its metadata.
     * @param txn The transaction in which the table is being created
     * @param table_name The name of the new table, note that all tables beginning with `__` are reserved for the system.
     * @param schema The schema of the new table
     * @param create_table_heap whether to create a table heap for the new table
     * @return A (non-owning) pointer to the metadata for the table
     */
    pub fn create_table(&mut self, txn: Arc<Transaction>, table_name: String, schema: Arc<Schema>, create_table_heap: Option<bool>) -> Option<Arc<TableInfo>> {
        if !self.table_names.contains_key(&table_name) {
            return None
        }

        let create_table_heap = create_table_heap.unwrap_or(true);

        // Construct the table heap
        // When create_table_heap == false, it means that we're running binder tests (where no txn will be provided) or
        // we are running shell without buffer pool. We don't need to create TableHeap in this case.
        let table: Arc<TableHeap> = if create_table_heap {
            Arc::new(TableHeap::new(self.bpm.clone()))
        } else {
            // Otherwise, create an empty heap only for binder tests
            Arc::new(TableHeap::default())
        };

        // Fetch the table OID for the new table
        let table_oid = self.next_table_oid.fetch_add(1, Ordering::SeqCst);

        // Construct the table information
        let meta = Arc::new(TableInfo::new(schema, table_name.clone(), table, table_oid));

        // Update the internal tracking mechanisms
        self.tables.insert(table_oid, meta.clone());
        self.table_names.insert(table_name.clone(), table_oid);
        self.index_names.insert(table_name, HashMap::new());

        Some(meta)
    }
}
