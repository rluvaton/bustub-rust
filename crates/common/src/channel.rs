use std::collections::VecDeque;
// use std::sync::{Condvar, Mutex};
use parking_lot::{Mutex, Condvar};

/**
 * Channels allow for safe sharing of data between threads. This is a multi-producer multi-consumer channel.
 */
// TODO - should we do this?
//        I think RUST have a built in way
pub struct Channel<T> {
    mutex: Mutex<VecDeque<T>>,
    cv: Condvar,
    // queue: VecDeque<T>,
}

unsafe impl <T>Send for Channel<T> {
}

impl<T> Channel<T> {
    pub fn new() -> Self {
        Channel {
            mutex: Mutex::new(VecDeque::new()),
            // queue: VecDeque::new(),
            cv: Condvar::new()
        }
    }
    pub fn put(&self, el: T) {
        self.mutex.lock().push_back(el);
        self.cv.notify_all();
    }

    pub fn get(&self) -> T {
        let mut guard = self.mutex.lock();
        self.cv.wait_while(&mut guard, |q| q.is_empty());

        guard.pop_front().unwrap()
    }
}
