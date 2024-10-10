use std::marker::PhantomData;
use std::sync::{Arc, Mutex, Condvar};
use std::time::Duration;

/// Promise
///
/// In order to share the promise result, you should use `SharedPromise` instead
pub struct PromiseLifetime<'a, T> {
    result: Arc<(Mutex<(bool, Option<T>)>, Condvar)>,
    phantom_data: PhantomData<&'a T>
}

impl<'a, T> PromiseLifetime<'a, T> {
    pub fn new() -> Self {
        Self {
            result: Arc::new((Mutex::new((false, None)), Condvar::new())),
            phantom_data: PhantomData,
        }
    }

    pub fn set_value(&self, value: T) {
        let (lock, cvar) = &*self.result;
        let mut result = lock.lock().unwrap();
        *result = (true, Some(value));
        cvar.notify_one();
    }

    pub fn get_future(&self) -> FutureLifetime<'a, T> {
        FutureLifetime {
            result: Arc::clone(&self.result),
            phantom_data: PhantomData,
        }
    }
}

/// This cannot be shared, do not clone itm if you want use `SharedFuture` and `SharedPromise`
#[derive(Debug)]
pub struct FutureLifetime<'a, T> {
    result: Arc<(Mutex<(bool, Option<T>)>, Condvar)>,
    phantom_data: PhantomData<&'a T>
}

impl<'a, T> FutureLifetime<'a, T> {
    pub fn wait(&self) -> T {
        let (lock, cvar) = &*self.result;
        let mut result = lock.lock().unwrap();
        while result.1.is_none() {
            result = cvar.wait(result).unwrap();
        }
        result.1.take().unwrap()
    }

    pub fn wait_for(&self, duration: Duration) -> bool {
        let (lock, cvar) = &*self.result;
        let ready = lock.lock().unwrap();

        let result = cvar.wait_timeout_while(ready, duration, |ready| !ready.0).unwrap();

        result.0.0 // Returns true if the value is ready within the timeout
    }
}

