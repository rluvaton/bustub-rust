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

#[derive(Clone)]
pub struct Catalog {
    #[allow(unused)]
    bpm: Option<Arc<BufferPoolManager>>,
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
    next_table_oid: Arc<AtomicTableOID>,

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
    next_index_oid: Arc<AtomicIndexOID>,
}


impl Catalog {
    pub fn new(bpm: Option<Arc<BufferPoolManager>>, lock_manager: Option<Arc<LockManager>>, log_manager: Option<Arc<LogManager>>) -> Self {
        Self {
            bpm,
            lock_manager,
            log_manager,
            tables: HashMap::new(),
            table_names: HashMap::new(),
            next_index_oid: Arc::new(AtomicTableOID::new(0)),
            indexes: HashMap::new(),
            index_names: HashMap::new(),
            next_table_oid: Arc::new(AtomicIndexOID::new(0)),
        }
    }


    /// Create a new table and return its metadata.
    ///
    /// @param txn The transaction in which the table is being created
    /// @param table_name The name of the new table, note that all tables beginning with `__` are reserved for the system.
    /// @param schema The schema of the new table
    /// @param create_table_heap whether to create a table heap for the new table
    /// @return A (non-owning) pointer to the metadata for the table
    ///
    pub fn create_table(&mut self, txn: Arc<Transaction>, table_name: String, schema: Arc<Schema>, create_table_heap: Option<bool>) -> Option<Arc<TableInfo>> {
        if !self.table_names.contains_key(&table_name) {
            return None
        }

        let create_table_heap = create_table_heap.unwrap_or(true);

        // Construct the table heap
        // When create_table_heap == false, it means that we're running binder tests (where no txn will be provided) or
        // we are running shell without buffer pool. We don't need to create TableHeap in this case.
        let table: Arc<TableHeap> = if create_table_heap {
            Arc::new(TableHeap::new(self.bpm.as_ref().unwrap().clone()))
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

    /// Query table metadata by name.
    /// @param table_name The name of the table
    /// @return A (non-owning) pointer to the metadata for the table
    pub fn get_table_by_name(&self, table_name: &str) -> Option<Arc<TableInfo>> {
        if let Some(table_oid) = self.table_names.get(table_name) {
            let meta = self.tables.get(table_oid).expect("Broken Invariant");

            return Some(meta.clone())
        }

        // Table not found
        None
    }


    /// Query table metadata by OID
    /// @param table_oid The OID of the table to query
    /// @return A (non-owning) pointer to the metadata for the table
    pub fn get_table_by_oid(&self, table_oid: TableOID) -> Option<Arc<TableInfo>> {
        self.tables.get(&table_oid).cloned()
    }

    pub fn get_table_names(&self) -> Vec<String> {
        self.table_names.keys().cloned().collect()
    }


    /// Get all of the indexes for the table identified by `table_name`.
    /// @param table_name The name of the table for which indexes should be retrieved
    /// @return A vector of IndexInfo* for each index on the given table, empty vector
    /// in the event that the table exists but no indexes have been created for it
    pub fn get_table_indexes_by_name(&self, table_name: &String) -> Vec<Arc<IndexInfo>> {
        /// Ensure the table exists
        if !self.table_names.contains_key(table_name) {
            return vec![];
        }

        let table_indexes = self.index_names.get(table_name).expect("Broken Invariant");

        let mut indexes = Vec::with_capacity(table_indexes.len());

        for index_meta in table_indexes {
            indexes.push(self.indexes.get(index_meta.1).cloned().expect("Broken Invariant"));
        }

        indexes
    }
}

impl Default for Catalog {
    fn default() -> Self {
        Self::new(None, None, None)
    }
}
