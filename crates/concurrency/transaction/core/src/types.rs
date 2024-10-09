use bytemuck::NoUninit;

/// Transaction State.
#[derive(Copy, Debug, Clone, PartialEq)]
pub enum TransactionState {
    Running = 0,
    Tainted,
    Committed = 100,
    Aborted,
}

/// Transaction isolation level. ReadUncommitted will NOT be used in project 3/4 as of Fall 2023.
#[derive(Copy, Debug, Clone, PartialEq)]
pub enum IsolationLevel {
    ReadUncommitted,
    SnapshotIsolation,
    Serializable
}

unsafe impl NoUninit for TransactionState {

}


unsafe impl NoUninit for IsolationLevel {

}

