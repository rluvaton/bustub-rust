use super::errors::{DeletePageError, FetchPageError, NewPageError};
use super::page_guards::{PageReadGuard, PageWriteGuard};
use crate::buffer::{AccessType};
use common::config::PageId;

/// Buffer pool
/// 1. The buffer pool should never have deadlock
pub trait BufferPool {
    /// Get the size of the buffer pool
    fn get_pool_size(&self) -> usize;

    /// Try to create new page and return write guard to it
    /// # Arguments
    ///
    /// * `access_type`: For leaderboard
    fn new_page<'a>(&self, access_type: AccessType) -> Result<PageWriteGuard<'a>, NewPageError>;

    /// Fetch page with read guard
    ///
    /// # Arguments
    ///
    /// * `page_id`: The page id to fetch
    /// * `access_type`: For leaderboard
    ///
    /// returns: Result<PinReadPageGuard, FetchPageError>
    ///
    fn fetch_page_read(&self, page_id: PageId, access_type: AccessType) -> Result<PageReadGuard, FetchPageError>;


    /// Fetch page with write guard
    ///
    /// # Arguments
    ///
    /// * `page_id`: The page id to fetch
    /// * `access_type`: For leaderboard
    ///
    /// returns: Result<PinWritePageGuard, FetchPageError>
    ///
    fn fetch_page_write(&self, page_id: PageId, access_type: AccessType) -> Result<PageWriteGuard, FetchPageError>;


    /// Flush page to disk REGARDLESS of the dirty flag.
    ///
    /// # Arguments
    ///
    /// * `page_id`: page id to flush
    ///
    /// returns: bool whether the flush was successful
    ///
    fn flush_page(&self, page_id: PageId) -> bool;

    /// Flush all pages to disk
    fn flush_all_pages(&self);


    /// Delete page from the buffer pool
    ///
    /// # Arguments
    ///
    /// * `page_id`: page id to delete
    ///
    /// returns: Result<bool, DeletePageError> false if the page is missing
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// ```
    fn delete_page(&self, page_id: PageId) -> Result<bool, DeletePageError>;


    /// Get pin count of the requested page used for tests
    ///
    /// # Arguments
    ///
    /// * `page_id`: page id to get the pin count for
    ///
    /// returns: Option<usize> None if the page is missing, Some(usize) with the pin count if exists
    ///
    fn get_pin_count(&self, page_id: PageId) -> Option<usize>;
}
