use common::config::{TableOID, TxnId};
use common::RID;
use crate::concurrency::LockMode;

/// Structure to hold a lock request.
/// This could be a lock request on a table OR a row.
/// For table lock requests, the rid_ attribute would be unused.
pub struct LockRequest {

    /// Txn_id of the txn requesting the lock
    txn_id: TxnId,

    /// Locking mode of the requested lock
    lock_mode: LockMode,

    /// Oid of the table for a table lock; oid of the table the row belong to for a row lock
    oid: TableOID,

    /// Rid of the row for a row lock; unused for table locks
    rid: RID,

    /// Whether the lock has been granted or not
    /// Default: `false`
    granted: bool
}

impl LockRequest {
    pub fn create_table_lock_request(txn_id: TxnId, lock_mode: LockMode, table_oid: TableOID) -> Self {
        Self {
            txn_id,
            lock_mode,
            oid: table_oid,
            rid: RID::default(),
            granted: false,
        }
    }

    pub fn create_row_lock_request(txn_id: TxnId, lock_mode: LockMode, table_oid: TableOID, rid: RID) -> Self {
        Self {
            txn_id,
            lock_mode,
            oid: table_oid,
            rid,
            granted: false,
        }
    }
}
