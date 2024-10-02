use common::config::PageId;
use crate::buffer::AccessType;
use crate::buffer::buffer_pool_manager_2::buffer_pool_errors::{DeletePageError, FetchPageError, FlushAllPagesError, FlushPageError, NewPageError};
use crate::buffer::buffer_pool_manager_2::page_guards::{ReadPageGuard, WritePageGuard};

pub trait BufferPool {
    /// Returns the number of frames that this buffer pool manages.
    fn size(&self) -> usize;

    /// Allocates a new page on disk.
    ///
    /// ### Implementation
    ///
    /// You will maintain a thread-safe, monotonically increasing counter in the form of a `AtomicPageId`.
    ///
    /// Also, make sure to read the documentation for `DeletePage`! You can't assume that you will never run out of disk
    /// space (via `DiskScheduler::IncreaseDiskSpace`).
    ///
    /// Once you have allocated the new page via the counter, make sure to call `DiskScheduler::IncreaseDiskSpace` so you
    /// have enough space on disk!
    ///
    /// TODO(P1): Add implementation.
    ///
    /// @return The page ID of the newly allocated page.
    fn new_page(&self) -> Result<PageId, NewPageError>;

    /// Removes a page from the database, both on disk and in memory.
    ///
    /// If the page is pinned in the buffer pool, this function does nothing and returns `false`. Otherwise, this function
    /// removes the page from both disk and memory (if it is still in the buffer pool), returning `true`.
    ///
    /// ### Implementation
    ///
    /// Think about all of the places a page or a page's metadata could be, and use that to guide you on implementing this
    /// function. You will probably want to implement this function _after_ you have implemented `CheckedReadPage` and
    /// `CheckedWritePage`.
    ///
    /// Ideally, we would want to ensure that all space on disk is used efficiently. That would mean the space that deleted
    /// pages on disk used to occupy should somehow be made available to new pages allocated by `NewPage`.
    ///
    /// If you would like to attempt this, you are free to do so. However, for this implementation, you are allowed to
    /// assume you will not run out of disk space and simply keep allocating disk space upwards in `NewPage`.
    ///
    /// For (nonexistent) style points, you can still call `DeallocatePage` in case you want to implement something slightly
    /// more space-efficient in the future.
    ///
    /// TODO(P1): Add implementation.
    ///
    /// @param page_id The page ID of the page we want to delete.
    /// @return `false` if the page exists but could not be deleted, `true` if the page didn't exist or deletion succeeded.
    fn delete_page(&self, page_id: PageId) -> Result<bool, DeletePageError>;

    /// @brief Acquires an optional write-locked guard over a page of data. The user can specify an `AccessType` if needed.
    ///
    /// If it is not possible to bring the page of data into memory, this function will return an error.
    ///
    /// Page data can _only_ be accessed via page guards. Users of this `BufferPoolManager` are expected to acquire either a
    /// `ReadPageGuard` or a `WritePageGuard` depending on the mode in which they would like to access the data, which
    /// ensures that any access of data is thread-safe.
    ///
    /// There can only be 1 `WritePageGuard` reading/writing a page at a time. This allows data access to be both immutable
    /// and mutable, meaning the thread that owns the `WritePageGuard` is allowed to manipulate the page's data however they
    /// want. If a user wants to have multiple threads reading the page at the same time, they must acquire a `ReadPageGuard`
    /// with `CheckedReadPage` instead.
    ///
    /// ### Implementation
    ///
    /// There are 3 main cases that you will have to implement. The first two are relatively simple: one is when there is
    /// plenty of available memory, and the other is when we don't actually need to perform any additional I/O. Think about
    /// what exactly these two cases entail.
    ///
    /// The third case is the trickiest, and it is when we do not have any _easily_ available memory at our disposal. The
    /// buffer pool is tasked with finding memory that it can use to bring in a page of memory, using the replacement
    /// algorithm you implemented previously to find candidate frames for eviction.
    ///
    /// Once the buffer pool has identified a frame for eviction, several I/O operations may be necessary to bring in the
    /// page of data we want into the frame.
    ///
    /// There is likely going to be a lot of shared code with `CheckedReadPage`, so you may find creating helper functions
    /// useful.
    ///
    /// These two functions are the crux of this project, so we won't give you more hints than this. Good luck!
    ///
    /// TODO(P1): Add implementation.
    ///
    /// @param page_id The ID of the page we want to write to.
    /// @param access_type The type of page access.
    /// @return std::optional<WritePageGuard> An optional latch guard where if there are no more free frames (out of memory)
    /// returns `std::nullopt`, otherwise returns a `WritePageGuard` ensuring exclusive and mutable access to a page's data.
    fn fetch_write_page(&self, page_id: PageId, access_type: AccessType) -> Result<WritePageGuard, FetchPageError>;


     /// @brief Acquires an optional read-locked guard over a page of data. The user can specify an `AccessType` if needed.
     ///
     /// If it is not possible to bring the page of data into memory, this function will return a `std::nullopt`.
     ///
     /// Page data can _only_ be accessed via page guards. Users of this `BufferPoolManager` are expected to acquire either a
     /// `ReadPageGuard` or a `WritePageGuard` depending on the mode in which they would like to access the data, which
     /// ensures that any access of data is thread-safe.
     ///
     /// There can be any number of `ReadPageGuard`s reading the same page of data at a time across different threads.
     /// However, all data access must be immutable. If a user wants to mutate the page's data, they must acquire a
     /// `WritePageGuard` with `CheckedWritePage` instead.
     ///
     /// ### Implementation
     ///
     /// See the implementation details of `CheckedWritePage`.
     ///
     /// TODO(P1): Add implementation.
     ///
     /// @param page_id The ID of the page we want to read.
     /// @param access_type The type of page access.
     /// @return std::optional<ReadPageGuard> An optional latch guard where if there are no more free frames (out of memory)
     /// returns `std::nullopt`, otherwise returns a `ReadPageGuard` ensuring shared and read-only access to a page's data.
    fn fetch_read_page(&self, page_id: PageId, access_type: AccessType) -> Result<ReadPageGuard, FetchPageError>;


     /// @brief Flushes a page's data out to disk.
     ///
     /// This function will write out a page's data to disk if it has been modified. If the given page is not in memory, this
     /// function will return `false`.
     ///
     /// ### Implementation
     ///
     /// You should probably leave implementing this function until after you have completed `CheckedReadPage` and
     /// `CheckedWritePage`, as it will likely be much easier to understand what to do.
     ///
     /// TODO(P1): Add implementation
     ///
     /// @param page_id The page ID of the page to be flushed.
     /// @return `false` if the page could not be found in the page table, otherwise `true`.
    fn flush_page(&self, page_id: PageId) -> Result<bool, FlushPageError>;


     /// @brief Flushes all page data that is in memory to disk.
     ///
     /// ### Implementation
     ///
     /// You should probably leave implementing this function until after you have completed `CheckedReadPage`,
     /// `CheckedWritePage`, and `FlushPage`, as it will likely be much easier to understand what to do.
     ///
     /// TODO(P1): Add implementation
    fn flush_all_pages(&self) -> Result<(), FlushAllPagesError>;



     /// @brief Retrieves the pin count of a page. If the page does not exist in memory, return `std::nullopt`.
     ///
     /// This function is thread safe. Callers may invoke this function in a multi-threaded environment where multiple threads
     /// access the same page.
     ///
     /// This function is intended for testing purposes. If this function is implemented incorrectly, it will definitely cause
     /// problems with the test suite and autograder.
     ///
     /// # Implementation
     ///
     /// We will use this function to test if your buffer pool manager is managing pin counts correctly. Since the
     /// `pin_count_` field in `FrameHeader` is an atomic type, you do not need to take the latch on the frame that holds the
     /// page we want to look at. Instead, you can simply use an atomic `load` to safely load the value stored. You will still
     /// need to take the buffer pool latch, however.
     ///
     /// TODO(P1): Add implementation
     ///
     /// @param page_id The page ID of the page we want to get the pin count of.
     /// @return std::optional<size_t> The pin count if the page exists, otherwise `std::nullopt`.
    fn get_pin_count(&self, page_id: PageId) -> Option<usize>;
}
