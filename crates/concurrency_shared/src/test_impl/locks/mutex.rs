use parking_lot::RawMutex;
pub use loom::sync::{
    Mutex as UnderlyingMutex,
    MutexGuard as UnderlyingMutexGuard,
};
use std::fmt;
use std::fmt::{Debug, Display};
use std::ops::{Deref, DerefMut};

/// A mutual exclusion primitive useful for protecting shared data
///
/// This mutex will block threads waiting for the lock to become available. The
/// mutex can also be statically initialized or created via a `new`
/// constructor. Each mutex has a type parameter which represents the data that
/// it is protecting. The data can only be accessed through the RAII guards
/// returned from `lock` and `try_lock`, which guarantees that the data is only
/// ever accessed when the mutex is locked.
pub struct Mutex<T: ?Sized> {
    underlying: UnderlyingMutex<T>
}

unsafe impl<T: ?Sized + Send> Send for Mutex<T> {}
unsafe impl<T: ?Sized + Send> Sync for Mutex<T> {}

impl<T> Mutex<T> {

    /// Creates a new mutex in an unlocked state ready for use.
    pub fn new(val: T) -> Mutex<T> {
        Self {
            underlying: UnderlyingMutex::new(val)
        }
    }

    /// Consumes this mutex, returning the underlying data.
    pub fn into_inner(self) -> T {
        self.underlying.into_inner().expect("into_inner should not fail")
    }
}

impl<T: ?Sized> Mutex<T> {
    #[inline]
    pub unsafe fn make_guard_unchecked(&self) -> MutexGuard<'_, T> {
        MutexGuard {
            underlying: self.underlying.lock().expect("should be able to lock")
        }
    }

    /// Acquires a mutex, blocking the current thread until it is able to do so.
    ///
    /// This function will block the local thread until it is available to acquire
    /// the mutex. Upon returning, the thread is the only thread with the mutex
    /// held. An RAII guard is returned to allow scoped unlock of the lock. When
    /// the guard goes out of scope, the mutex will be unlocked.
    ///
    /// Attempts to lock a mutex in the thread which already holds the lock will
    /// result in a deadlock.
    #[inline]
    pub fn lock(&self) -> MutexGuard<'_, T> {
        MutexGuard {
            underlying: self.underlying.lock().expect("should be able to lock")
        }
    }

    /// Attempts to acquire this lock.
    ///
    /// If the lock could not be acquired at this time, then `None` is returned.
    /// Otherwise, an RAII guard is returned. The lock will be unlocked when the
    /// guard is dropped.
    ///
    /// This function does not block.
    #[inline]
    pub fn try_lock(&self) -> Option<MutexGuard<'_, T>> {
        self.underlying.try_lock().map(|o| {
            MutexGuard {
                underlying: o
            }
        }).ok()
    }

    /// Returns a mutable reference to the underlying data.
    ///
    /// Since this call borrows the `Mutex` mutably, no actual locking needs to
    /// take place---the mutable borrow statically guarantees no locks exist.
    #[inline]
    pub fn get_mut(&mut self) -> &mut T {
        todo!()
    }

    /// Checks whether the mutex is currently locked.
    #[inline]
    pub fn is_locked(&self) -> bool {
        todo!()
    }

    /// Forcibly unlocks the mutex.
    ///
    /// This is useful when combined with `mem::forget` to hold a lock without
    /// the need to maintain a `MutexGuard` object alive, for example when
    /// dealing with FFI.
    ///
    /// # Safety
    ///
    /// This method must only be called if the current thread logically owns a
    /// `MutexGuard` but that guard has been discarded using `mem::forget`.
    /// Behavior is undefined if a mutex is unlocked when not locked.
    #[inline]
    pub unsafe fn force_unlock(&self) {
        todo!()
    }

    /// Returns the underlying raw mutex object.
    ///
    /// Note that you will most likely need to import the `RawMutex` trait from
    /// `lock_api` to be able to call functions on the raw mutex.
    ///
    /// # Safety
    ///
    /// This method is unsafe because it allows unlocking a mutex while
    /// still holding a reference to a `MutexGuard`.
    #[inline]
    pub unsafe fn raw(&self) -> &RawMutex {
        todo!()
    }

    /// Returns a raw pointer to the underlying data.
    ///
    /// This is useful when combined with `mem::forget` to hold a lock without
    /// the need to maintain a `MutexGuard` object alive, for example when
    /// dealing with FFI.
    ///
    /// # Safety
    ///
    /// You must ensure that there are no data races when dereferencing the
    /// returned pointer, for example if the current thread logically owns
    /// a `MutexGuard` but that guard has been discarded using `mem::forget`.
    #[inline]
    pub fn data_ptr(&self) -> *mut T {
        todo!()
    }
}
//
// impl<T: ?Sized> Mutex<T> {
//     /// Attempts to acquire this lock until a timeout is reached.
//     ///
//     /// If the lock could not be acquired before the timeout expired, then
//     /// `None` is returned. Otherwise, an RAII guard is returned. The lock will
//     /// be unlocked when the guard is dropped.
//     #[inline]
//     pub fn try_lock_for(&self, timeout: parking_lot::RawMutex::Duration) -> Option<MutexGuard<'_, T>> {
//         todo!()
//     }
//
//     /// Attempts to acquire this lock until a timeout is reached.
//     ///
//     /// If the lock could not be acquired before the timeout expired, then
//     /// `None` is returned. Otherwise, an RAII guard is returned. The lock will
//     /// be unlocked when the guard is dropped.
//     #[inline]
//     pub fn try_lock_until(&self, timeout: RawMutex::Instant) -> Option<MutexGuard<'_, T>> {
//         todo!()
//     }
//
// }

impl<T: ?Sized + Default> Default for Mutex<T> {
    #[inline]
    fn default() -> Mutex<T> {
        Mutex::new(Default::default())
    }
}

impl<T> From<T> for Mutex<T> {
    #[inline]
    fn from(t: T) -> Mutex<T> {
        Mutex::new(t)
    }
}

impl<T: ?Sized + fmt::Debug> fmt::Debug for Mutex<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.try_lock() {
            Some(guard) => f.debug_struct("Mutex").field("data", &&*guard).finish(),
            None => {
                struct LockedPlaceholder;
                impl fmt::Debug for LockedPlaceholder {
                    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                        f.write_str("<locked>")
                    }
                }

                f.debug_struct("Mutex")
                    .field("data", &LockedPlaceholder)
                    .finish()
            }
        }
    }
}

#[clippy::has_significant_drop]
#[must_use = "if unused the Mutex will immediately unlock"]
pub struct MutexGuard<'a, T: ?Sized> {
    underlying: UnderlyingMutexGuard<'a, T>,
}

unsafe impl<'a, T: ?Sized + Sync + 'a> Sync for MutexGuard<'a, T> {}

impl<'a, T: ?Sized + 'a> MutexGuard<'a, T> {
    /// Returns a reference to the original `Mutex` object.
    pub fn mutex(s: &Self) -> &'a Mutex< T> {
        todo!()
    }

    pub fn unlocked<F, U>(s: &mut Self, f: F) -> U
    where
        F: FnOnce() -> U,
    {
        todo!()
    }

    pub fn leak(s: Self) -> &'a mut T {
        todo!()
    }
}

impl<'a, T: ?Sized + 'a> Deref for MutexGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &T {
        self.underlying.deref()
    }
}

impl<'a, T: ?Sized + 'a> DerefMut for MutexGuard<'a, T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut T {
        self.underlying.deref_mut()
    }
}

impl<'a, T: Debug + ?Sized + 'a> Debug for MutexGuard<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.underlying.fmt(f)
    }
}

impl<'a, T: Display + ?Sized + 'a> Display for MutexGuard<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.underlying.fmt(f)
    }
}
