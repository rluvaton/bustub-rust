use std::collections::VecDeque;
use std::sync::{Condvar, Mutex};

/**
 * Channels allow for safe sharing of data between threads. This is a multi-producer multi-consumer channel.
 */
// TODO - should we do this?
//        I think RUST have a built in way
pub struct Channel<T> {
    mutex: Mutex<()>,
    cv: Condvar,
    queue: VecDeque<T>,
}

impl<T> Channel<T> {
    pub fn new() -> Self {
        Channel {
            mutex: Mutex::new(()),
            queue: VecDeque::new(),
            cv: Condvar::new()
        }
    }
    pub fn put(&mut self, el: T) {
        {
            let _guard = self.mutex.lock().unwrap();
            self.queue.push_back(el);
        }

        self.cv.notify_all()
    }

    pub fn get(&mut self) -> T {
        let _guard = self.cv.wait_while(self.mutex.lock().unwrap(), |_| !self.queue.is_empty()).unwrap();

        return self.queue.pop_front().unwrap();
    }
}
