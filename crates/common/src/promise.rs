use std::sync::{Arc, Mutex, Condvar};
use std::time::Duration;

pub struct Promise<T> {
    result: Arc<(Mutex<(bool, Option<T>)>, Condvar)>,
}

impl<T> Promise<T> {
    pub fn new() -> Self {
        Self {
            result: Arc::new((Mutex::new((false, None)), Condvar::new())),
        }
    }

    pub fn set_value(&self, value: T) {
        let (lock, cvar) = &*self.result;
        let mut result = lock.lock().unwrap();
        *result = (true, Some(value));
        cvar.notify_one();
    }

    pub fn get_future(&self) -> Future<T> {
        Future {
            result: Arc::clone(&self.result),
        }
    }
}

pub struct Future<T> {
    result: Arc<(Mutex<(bool, Option<T>)>, Condvar)>,
}

impl<T> Future<T> {
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

#[cfg(test)]
mod tests {
    use std::thread;
    use std::time::Duration;
    use crate::Future;
    use crate::promise::Promise;

    #[test]
    fn should_work_with_boolean() {
        let promise = Promise::new();
        let future = promise.get_future();

        thread::spawn(move || {
            // Simulate some work
            promise.set_value(true);
        });

        let result = future.wait();

        assert_eq!(result, true)
    }

    #[test]
    fn should_work_with_string() {
        let promise = Promise::new();
        let future = promise.get_future();

        thread::spawn(move || {
            // Simulate some work
            promise.set_value("hello");
        });

        let result = future.wait();

        assert_eq!(result, "hello")
    }

    #[test]
    fn support_wait_for() {
        let promise = Promise::new();
        let future = promise.get_future();

        thread::spawn(move || {
            thread::sleep(Duration::from_millis(2_000));

            // Simulate some work
            promise.set_value("hello");
        });

        let is_ready = future.wait_for(Duration::from_millis(1000));
        assert_eq!(is_ready, false);

        let is_ready = future.wait_for(Duration::from_millis(500));
        assert_eq!(is_ready, false);

        let is_ready = future.wait_for(Duration::from_millis(600));
        assert_eq!(is_ready, true);

        // Should still work
        let result = future.wait();
        assert_eq!(result, "hello");

        // Should still work even after finished
        let is_ready = future.wait_for(Duration::from_millis(200));
        assert_eq!(is_ready, true);

    }
}

