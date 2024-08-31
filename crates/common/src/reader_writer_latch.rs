use concurrency_shared::locks::RwLock;

pub type ReaderWriterLatch<T> = RwLock<T>;
