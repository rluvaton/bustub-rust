use std::hash::Hash;
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use common::config::{PageId, BUSTUB_PAGE_SIZE, INVALID_PAGE_ID};
use std::mem::size_of;
use prettytable::{row, Table};
use binary_utils::GetNBits;
use generics::GetOr;

const _HASH_TABLE_DIRECTORY_PAGE_METADATA_SIZE: usize = size_of::<u32>() * 2;

/// `HASH_TABLE_DIRECTORY_ARRAY_SIZE` is the number of page_ids that can fit in the directory page of an extendible hash index.
/// This is 512 because the directory array must grow in powers of 2, and 1024 page_ids leaves zero room for
/// storage of the other member variables.
///
///
const HASH_TABLE_DIRECTORY_MAX_DEPTH: u32 = 9;
const HASH_TABLE_DIRECTORY_ARRAY_SIZE: usize = 1 << HASH_TABLE_DIRECTORY_MAX_DEPTH;


//noinspection RsAssertEqual
const _: () = assert!(size_of::<PageId>() == 4);
//noinspection RsAssertEqual
const _: () = assert!(size_of::<DirectoryPage>() == _HASH_TABLE_DIRECTORY_PAGE_METADATA_SIZE + HASH_TABLE_DIRECTORY_ARRAY_SIZE + size_of::<PageId>() * HASH_TABLE_DIRECTORY_ARRAY_SIZE);
const _: () = assert!(size_of::<DirectoryPage>() <= BUSTUB_PAGE_SIZE);


///
/// Directory pages sit at the second level of our disk-based extendible hash table.
/// Each of them stores the logical child pointers to the bucket pages (as page ids), as well as metadata for handling bucket mapping and dynamic directory growing and shrinking.
///
#[repr(C)]
pub struct DirectoryPage {
    /// The maximum depth the header page could handle
    max_depth: u32,

    /// The current directory global depth
    global_depth: u32,

    /// An array of bucket page local depths
    local_depths: [u8; HASH_TABLE_DIRECTORY_ARRAY_SIZE],

    /// An array of bucket page ids
    bucket_page_ids: [PageId; HASH_TABLE_DIRECTORY_ARRAY_SIZE],
}


impl DirectoryPage {
    // Delete all constructor / destructor to ensure memory safety
    // TODO - delete destructor?

    /// After creating a new directory page from buffer pool, must call initialize
    /// method to set default values
    ///
    /// # Arguments
    ///
    /// * `max_depth`: Max depth in the directory page, set to None for default value `HASH_TABLE_DIRECTORY_MAX_DEPTH`
    ///
    /// returns: ()
    ///
    pub fn init(&mut self, max_depth: Option<u32>) {
        self.max_depth = max_depth.unwrap_or(HASH_TABLE_DIRECTORY_MAX_DEPTH);
        self.local_depths.fill(0);
        self.global_depth = 0;
        self.bucket_page_ids.fill(INVALID_PAGE_ID);
    }


    /// Get the bucket index that the key is hashed to
    ///
    /// # Arguments
    ///
    /// * `hash`: the hash of the key
    ///
    /// returns: u32 bucket index current key is hashed to
    ///
    pub fn hash_to_bucket_index(&self, hash: u32) -> u32 {
        // hash.get_n_msb_bits(self.global_depth as u8)
        // TODO - is this correct? also size is slow
        hash % self.size()
    }

    /// Lookup a bucket page using a directory index
    ///
    /// # Arguments
    ///
    /// * `bucket_idx`: the index in the directory to lookup
    ///
    /// returns: PageId bucket page_id corresponding to bucket_idx
    ///
    pub fn get_bucket_page_id(&self, bucket_idx: u32) -> PageId {
        self.bucket_page_ids[bucket_idx as usize]
    }


    /// Updates the directory index using a bucket index and page_id
    ///
    /// # Arguments
    ///
    /// * `bucket_idx`: directory index at which to insert page_id
    /// * `bucket_page_id`: page_id to insert
    ///
    ///
    pub fn set_bucket_page_id(&mut self, bucket_idx: u32, bucket_page_id: PageId) {
        self.bucket_page_ids[bucket_idx as usize] = bucket_page_id
    }

    /// Gets the split image of an index
    ///
    /// # Arguments
    ///
    /// * `bucket_idx`: the directory index for which to find the split image
    ///
    /// returns: u32 the directory index of the split image
    ///
    pub fn get_split_image_index(&self, bucket_idx: u32) -> u32 {
        0
    }

    /// returns a mask of global_depth 1's and the rest 0's.
    ///
    /// In Extendible Hashing we map a key to a directory index
    /// using the following hash + mask function.
    ///
    /// DirectoryIndex = Hash(key) & GLOBAL_DEPTH_MASK
    ///
    /// where GLOBAL_DEPTH_MASK is a mask with exactly GLOBAL_DEPTH 1's from LSB
    /// upwards.  For example, global depth 3 corresponds to 0x00000007 in a 32-bit
    /// representation.
    ///
    /// returns: u32 mask of global_depth 1's and the rest 0's (with 1's from LSB upwards)
    ///
    pub fn get_global_depth_mask(&self) -> u32 {
        unimplemented!()
    }

    /// same as global depth mask, except it
    /// uses the local depth of the bucket located at bucket_idx
    ///
    /// # Arguments
    ///
    /// * `bucket_idx`: the index to use for looking up local depth
    ///
    /// returns: u32 mask of local 1's and the rest 0's (with 1's from LSB upwards)
    ///
    pub fn get_local_depth_mask(&self, bucket_idx: u32) -> u32 {
        unimplemented!()
    }

    /// Get the global depth of the hash table directory
    ///
    /// # Arguments
    ///
    /// returns: u32 the global depth of the directory
    ///
    pub fn get_global_depth(&self) -> u32 {
        self.global_depth
    }

    pub fn get_max_depth(&self) -> u32 {
        self.max_depth
    }

    /// Increment the global depth of the directory
    pub fn incr_global_depth(&mut self) {
        let size_before = self.size() as usize;
        self.global_depth += 1;
        let size_after = self.size() as usize;

        // When increasing global depth, we now have double the size we had before
        // and because the new buckets don't point to anywhere, we must point them to the same bucket before the doubling
        // Example:
        // if we had hash value 0b10101 and local and global depth was 2 (so the size was 2 ^ (global depth) = 4)
        // when we increase the number of buckets (global depth + 1) we now have 8 buckets (2 ^ (global depth) = 8)
        // and hash value 0b10101 will lend on 0b101 instead of 0b01
        // but that bucket has no page and no local depth
        // so we need to point it to the old bucket (as local depth has not changed for that bucket)
        // and set the local depth of that bucket to be the same as the one before

        // Point to the same bucket page id
        {
            let (before, after) = self.bucket_page_ids[..size_after].split_at_mut(size_before);
            after.clone_from_slice(&before);
        }

        // Have the same local depth as the pointed bucket page
        {
            let (before, after) = self.local_depths[..size_after].split_at_mut(size_before);
            after.clone_from_slice(&before);
        }
    }

    /// Decrement the global depth of the directory
    pub fn decr_global_depth(&mut self) {
        self.global_depth -= 1;
    }

    ///
    /// returns: bool true if the directory can be shrunk
    ///
    pub fn can_shrink(&self) -> bool {
        self.local_depths.iter().all(|&d| (d as u32) < self.global_depth)
    }

    ///
    /// returns: u32 the current directory size
    ///
    pub fn size(&self) -> u32 {
        2u32.pow(self.global_depth)
    }

    ///
    /// returns: u32 the max directory size
    ///
    pub fn max_size(&self) -> u32 {
        // TODO - is this the right value?
        self.max_depth
    }

    /// Gets the local depth of the bucket at bucket_idx
    ///
    /// # Arguments
    ///
    /// * `bucket_idx`: the bucket index to lookup
    ///
    /// returns: u32 the local depth of the bucket at bucket_idx
    ///
    pub fn get_local_depth(&self, bucket_idx: u32) -> u32 {
        self.local_depths[bucket_idx as usize] as u32
    }


    /// Set the local depth of the bucket at bucket_idx to local_depth
    ///
    /// # Arguments
    ///
    /// * `bucket_idx`: the bucket index to update
    /// * `local_depth`: new local depth
    ///
    pub fn set_local_depth(&mut self, bucket_idx: u32, local_depth: u8) {
        self.local_depths[bucket_idx as usize] = local_depth;
    }

    /// Increment the local depth of the bucket at bucket_idx
    ///
    /// # Arguments
    ///
    /// * `bucket_idx`: bucket index to increment
    ///
    pub fn incr_local_depth(&mut self, bucket_idx: u32) {
        unimplemented!()
    }

    /// Decrement the local depth of the bucket at bucket_idx
    ///
    /// # Arguments
    ///
    /// * `bucket_idx`: bucket index to decrement
    ///
    pub fn decr_local_depth(&mut self, bucket_idx: u32) {
        unimplemented!()
    }

    /// Verify the following invariants:
    /// (1) All LD <= GD.
    /// (2) Each bucket has precisely 2^(GD - LD) pointers pointing to it.
    /// (3) The LD is the same at each index with the same bucket_page_id
    ///
    pub fn verify_integrity(&self) {
        // build maps of {bucket_page_id : pointer_count} and {bucket_page_id : local_depth}
        let mut page_id_to_count: HashMap<PageId, u32> = HashMap::new();
        let mut page_id_to_ld: HashMap<PageId, u32> = HashMap::new();

        // Verify: (3) The LD is the same at each index with the same bucket_page_id
        for curr_idx in 0..self.size() {
            let curr_page_id = self.bucket_page_ids[curr_idx as usize];
            let curr_ld = self.local_depths[curr_idx as usize] as u32;

            // Verify: (1) All LD <= GD.
            assert!(curr_ld <= self.global_depth, "there exists a local depth greater than the global depth");

            page_id_to_count.insert(curr_page_id, page_id_to_count.get_or(&curr_page_id, &0) + 1);

            if page_id_to_ld.contains_key(&curr_page_id) && curr_ld != page_id_to_ld[&curr_page_id] {
                let old_ld = *page_id_to_ld.get_or(&curr_page_id, &0);
                println!("Verify Integrity: curr_local_depth: {}, old_local_depth {}, for page_id: {}", curr_ld, old_ld, curr_page_id);
                self.print_directory();
                assert_eq!(curr_ld, *page_id_to_ld.get_or(&curr_page_id, &0), "local depth is not the same at each index with same bucket page id");
            } else {
                page_id_to_ld.insert(curr_page_id, curr_ld);
            }
        }

        // Verify: (2) Each bucket has precisely 2^(GD - LD) pointers pointing to it.
        for (curr_page_id, curr_count) in page_id_to_count {
            let curr_ld = *page_id_to_ld.get_or(&curr_page_id, &0);
            let required_count: u32 = 0x1 << (self.global_depth - curr_ld);

            if curr_count != required_count {
                println!("Verify Integrity: curr_count: {}, required_count {}, for page_id: {}", curr_count, required_count, curr_page_id);
                self.print_directory();
                assert_eq!(curr_count, required_count, "a bucket does not have precisely 2^(GD - LD) pointers to it")
            }
        }
    }

    /// Prints the current directory
    pub fn print_directory(&self) {
        println!("{:?}", self)
    }
}

impl Debug for DirectoryPage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("======== DIRECTORY (global_depth: {}) ========\n", self.global_depth).as_str())?;

        let mut table = Table::new();

        table.add_row(row!["bucket_idx", "page_id", "local_depth"]);

        let max_id = (0x1 << self.global_depth) as u32;
        for idx in 0..(max_id as usize) {
            let page_id = self.bucket_page_ids[idx];
            let local_depth = self.local_depths[idx];

            table.add_row(row![idx, page_id, local_depth]);
        }

        f.write_str(table.to_string().as_str())?;

        f.write_str("================ END DIRECTORY ================")
    }
}
