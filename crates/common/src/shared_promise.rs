use std::sync::{Arc, Mutex, Condvar};
use std::time::Duration;

/// Promise that can be shared between multiple threads, for single thread, use `Promise`
pub struct SharedPromise<T: Clone> {
    result: Arc<(Mutex<(bool, Option<T>)>, Condvar)>,
}

impl<T: Clone> SharedPromise<T> {
    pub fn new() -> Self {
        Self {
            result: Arc::new((Mutex::new((false, None)), Condvar::new())),
        }
    }

    pub fn set_value(&self, value: T) {
        let (lock, cvar) = &*self.result;
        let mut result = lock.lock().unwrap();
        *result = (true, Some(value));
        cvar.notify_all();
    }

    pub fn get_future(&self) -> SharedFuture<T> {
        SharedFuture {
            result: Arc::clone(&self.result),
        }
    }
}


#[derive(Debug, Clone)]
pub struct SharedFuture<T: Clone> {
    result: Arc<(Mutex<(bool, Option<T>)>, Condvar)>,
}

impl<T: Clone> SharedFuture<T> {
    pub fn wait(&self) -> T {
        let (lock, cvar) = &*self.result;
        let mut result = lock.lock().unwrap();
        while result.1.is_none() {
            result = cvar.wait(result).unwrap();
        }

        result.1.clone().unwrap()
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
    use std::sync::Arc;
    use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
    use std::thread;
    use std::time::Duration;
    
    use crate::SharedPromise;

    #[test]
    fn should_work_with_boolean() {
        let promise = SharedPromise::new();
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
        let promise = SharedPromise::new();
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
        let promise = SharedPromise::new();
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

    #[test]
    fn should_work_waiting_multiple_times_some_before_finish_some_after() {
        let promise = SharedPromise::new();
        let future = promise.get_future();

        let number_of_threads_ready = Arc::new(AtomicUsize::new(0));
        let can_resume = Arc::new(AtomicBool::new(false));

        let mut threads = vec![];
        for i in 0..100 {
            let future = future.clone();
            let number_of_threads_ready = number_of_threads_ready.clone();
            let can_resume = can_resume.clone();
            threads.push(thread::spawn(move || {
                let prev = number_of_threads_ready.fetch_add(1, Ordering::SeqCst);

                // After 50 items wait for a signal to continue
                if prev >= 50 {
                    while !can_resume.load(Ordering::SeqCst) {

                    }
                }

                // Wait for 2s and then timeout
                future.wait();
            }))
        }


        thread::spawn(move || {
            while number_of_threads_ready.load(Ordering::SeqCst) < 50 {

            }

            thread::sleep(Duration::from_millis(100));

            // Simulate some work
            promise.set_value(true);

            can_resume.store(true, Ordering::SeqCst);
        }).join().unwrap();

        for t in threads {
            t.join().unwrap()
        }
    }
}

