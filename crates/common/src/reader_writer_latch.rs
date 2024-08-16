use parking_lot::RwLock;

pub type ReaderWriterLatch<T> = RwLock<T>;
