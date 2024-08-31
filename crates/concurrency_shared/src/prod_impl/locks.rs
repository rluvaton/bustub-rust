pub use std::sync::{
    Mutex as StdMutex,
    MutexGuard as StdMutexGuard,
    RwLock as StdRwLock,
    RwLockReadGuard as StdRwLockReadGuard,
    RwLockWriteGuard as StdRwLockWriteGuard,
    Condvar as StdCondvar,
    WaitTimeoutResult as StdWaitTimeoutResult,
};

pub use parking_lot::{
    Mutex,
    MutexGuard,
    RwLock,
    RwLockReadGuard,
    RwLockWriteGuard,
    RwLockUpgradableReadGuard
};

