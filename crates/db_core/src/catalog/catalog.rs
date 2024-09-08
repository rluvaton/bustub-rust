use std::collections::HashMap;
use std::sync::Arc;
use common::config::{AtomicIndexOID, AtomicTableOID, IndexOID, TableOID};
use crate::buffer::BufferPoolManager;
use crate::catalog::IndexInfo;
use crate::catalog::table_info::TableInfo;
use crate::concurrency::LockManager;
use crate::recovery::LogManager;

pub struct Catalog {
    bpm: Arc<BufferPoolManager>,
    lock_manager: Arc<LockManager>,
    log_manager: Arc<LogManager>,

    ///
    /// Map table identifier -> table metadata.
    ///
    /// NOTE: `tables` owns all table metadata.
    ///
    tables: HashMap<TableOID, Arc<TableInfo>>,

    /// Map table name -> table identifiers.
    table_names: HashMap<String, TableOID>,

    /// The next table identifier to be used.
    ///
    /// Default: 0
    next_table_oid: AtomicTableOID,

    /// Map index identifier -> index metadata.
    ///
    /// NOTE: that `indexes` owns all index metadata.
    indexes: HashMap<IndexOID, Arc<IndexInfo>>,

    /// Map table name -> index names -> index identifiers.
    index_names: HashMap<String, HashMap<String, IndexOID>>,

    /// The next index identifier to be used.
    /// Default: `0`
    next_index_oid: AtomicIndexOID,
}


impl Catalog {
    pub fn new(bpm: Arc<BufferPoolManager>, lock_manager: Arc<LockManager>, log_manager: Arc<LogManager>) -> Self {
        Self {
            bpm,
            lock_manager,
            log_manager,
            tables: HashMap::new(),
            table_names: HashMap::new(),
            next_index_oid: AtomicTableOID::new(0),
            indexes: HashMap::new(),
            index_names: HashMap::new(),
            next_table_oid: AtomicIndexOID::new(0)
        }
    }
}
