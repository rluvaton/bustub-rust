mod mutex;

pub use mutex::*;

// TODO - use shuttle for RW

pub use parking_lot::{
    RwLock,
    RwLockReadGuard,
    RwLockWriteGuard,
    RwLockUpgradableReadGuard
};


// Have prefix Std as it has different behavior
pub use loom::sync::{
    Mutex as StdMutex,
    MutexGuard as StdMutexGuard,
    RwLock as StdRwLock,
    RwLockReadGuard as StdRwLockReadGuard,
    RwLockWriteGuard as StdRwLockWriteGuard,
    Condvar as StdCondvar,
    WaitTimeoutResult as StdWaitTimeoutResult,
};
