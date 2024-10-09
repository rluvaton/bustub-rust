use std::sync::atomic::AtomicI32;

// Reserved id for invalid page id
pub const INVALID_PAGE_ID: PageId = -1;

/// Size of a data page in byte
pub const PAGE_SIZE: usize = 4096;


/// The type of the page id (in cpp it was `page_id_t`)
pub type PageId = i32;

/// Atomic page id (in cpp it was `std::atomic<page_id_t>`)
pub type AtomicPageId = AtomicI32;

/// The data of the page
pub type PageData = [u8; PAGE_SIZE];

