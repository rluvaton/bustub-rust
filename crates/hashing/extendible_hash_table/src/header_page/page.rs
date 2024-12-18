use crate::header_page::HeaderIter;
use binary_utils::GetNBits;
use comfy_table::Table;
use pages::{PageId, INVALID_PAGE_ID, PAGE_SIZE};
use std::fmt::{Debug, Formatter};
use std::mem::size_of;
use crate::header_page::iterator::HeaderIterState;

#[allow(dead_code)]
const PAGE_METADATA_SIZE: usize = size_of::<u32>();

//noinspection RsAssertEqual
const _: () = assert!(size_of::<PageId>() == 4);
//noinspection RsAssertEqual
const _: () = assert!(size_of::<HeaderPage>() == size_of::<PageId>() * HeaderPage::ARRAY_SIZE + PAGE_METADATA_SIZE);
const _: () = assert!(size_of::<HeaderPage>() <= PAGE_SIZE);

///
/// Header page format:
///  ---------------------------------------------------
/// | DirectoryPageIds(2048) | MaxDepth (4) | Free(2044)
///  ---------------------------------------------------
///
///
///
/// The header page sits the at the first level of our disk-based extendible hash table,
/// and there is only one header page for a hash table.
///
/// It stores the logical child pointers to the directory pages (as page ids).
/// You can think about it as a static first-level directory page. The header page has the following fields:
#[repr(C)]
pub(crate) struct HeaderPage {
    /// An array of directory page ids
    pub(super) directory_page_ids: [PageId; HeaderPage::ARRAY_SIZE],

    /// The maximum depth the header page could handle
    max_depth: u32,
}


impl HeaderPage {
    pub(crate) const MAX_DEPTH: u32 = 9;
    const ARRAY_SIZE: usize = 1 << Self::MAX_DEPTH;
    // Delete all constructor / destructor to ensure memory safety
    // TODO - delete destructor?

    /// After creating a new header page from buffer pool, must call initialize
    /// method to set default values
    ///
    /// # Arguments
    ///
    /// * `max_depth`: Max depth in the header page, set to None for default value `HASH_TABLE_HEADER_MAX_DEPTH`
    ///
    /// returns: ()
    ///
    pub(crate) fn init(&mut self, max_depth: Option<u32>) {
        self.max_depth = max_depth.unwrap_or(Self::MAX_DEPTH);
        self.directory_page_ids.fill(INVALID_PAGE_ID);
    }


    /// Get the directory index that the key is hashed to
    ///
    /// # Arguments
    ///
    /// * `hash`: the hash of the key
    ///
    /// returns: u32 directory index the key is hashed to
    ///
    pub(crate) fn hash_to_directory_index(&self, hash: u32) -> u32 {
        // When max depth is 0 than all goes to the same directory
        if self.max_depth == 0 {
            return 0;
        }

        // You will want to use the most-significant bits for indexing into the header page's directory_page_ids array.
        // This involves taking the hash of your key and perform bit operations with the depth of the header page.
        // The header page depth will not change.

        let index = hash.get_n_msb_bits(self.max_depth as u8);

        assert!(index <= self.max_size(), "directory index {} must not be above header max size {}", index, self.max_size());

        index

        // TODO - should not expose this and the `get_directory_page_id` and instead have a function that get a hash value and return the page id?
    }

    /// Get the directory page id at an index
    ///
    /// # Arguments
    ///
    /// * `directory_idx`: index in the directory page id array
    ///
    /// returns: u32 directory page_id at index
    ///
    /// TODO - should return Option in case of invalid?
    ///        Should add unsafe unchecked function?
    pub(crate) fn get_directory_page_id(&self, directory_idx: u32) -> PageId {
        if directory_idx >= Self::ARRAY_SIZE as u32 {
            return INVALID_PAGE_ID;
        }
        self.directory_page_ids[directory_idx as usize]
    }


    /// Set the directory page id at an index
    ///
    /// # Arguments
    ///
    /// * `directory_idx`: index in the directory page id array
    /// * `directory_page_id`: page id of the directory
    ///
    /// returns: ()
    ///
    pub(crate) fn set_directory_page_id(&mut self, directory_idx: u32, directory_page_id: PageId) {
        assert!(directory_idx < Self::ARRAY_SIZE as u32, "Directory index ({}) is larger than the size directory size: {}", directory_idx, Self::ARRAY_SIZE as u32);

        self.directory_page_ids[directory_idx as usize] = directory_page_id;
    }

    /// Get the maximum number of directory page ids the header page could handle
    pub fn max_size(&self) -> u32 {
        2u32.pow(self.max_depth)
    }

    /// Prints the header's occupancy information
    #[allow(unused)]
    pub fn print_header(&self) {
        println!("{:?}", self)
    }
    
    /// Verify that the header page does not have any directory
    /// It will return Ok(()) when all directory page ids are missing or Err(Vec<PageId>) with all the existing directory page ids
    #[cfg(test)]
    pub fn verify_empty(&self) {
        let non_empty_page_id = (0..self.max_size()).into_iter()
            .map(|index| self.get_directory_page_id(index))
            .find(|page_id| page_id != &INVALID_PAGE_ID);
        
        assert_eq!(non_empty_page_id, None, "There must be no page id in the header page");
    }

    // Get iterator on valid page ids
    pub(crate) fn iter(&self) -> HeaderIter {
        HeaderIter::new(self)
    }
    
    pub(crate) fn resume_iter(&self, state: HeaderIterState) -> HeaderIter {
        HeaderIter::with_state(self, state)
    }
}

impl Debug for HeaderPage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("======== HEADER (max_depth: {}) ========\n", self.max_depth).as_str())?;

        let mut table = Table::new();

        table.set_header(vec!["directory_idx", "page_id"]);

        let max_id: PageId = 1 << self.max_depth;
        for idx in 0..max_id {
            table.add_row(vec![idx, self.directory_page_ids[idx as usize]]);
        }

        f.write_str(table.to_string().as_str())?;

        f.write_str("\n======== END HEADER ========\n")
    }
}
