use binary_utils::GetNBits;
use pages::{PageId, PAGE_SIZE, INVALID_PAGE_ID};
use generics::GetOr;
use prettytable::{row, Table};
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::mem::size_of;

const PAGE_METADATA_SIZE: usize = size_of::<u32>() * 2;



//noinspection RsAssertEqual
const _: () = assert!(size_of::<PageId>() == 4);
//noinspection RsAssertEqual
const _: () = assert!(size_of::<DirectoryPage>() == PAGE_METADATA_SIZE + DirectoryPage::ARRAY_SIZE + size_of::<PageId>() * DirectoryPage::ARRAY_SIZE);
const _: () = assert!(size_of::<DirectoryPage>() <= PAGE_SIZE);

///
/// Directory pages sit at the second level of our disk-based extendible hash table.
/// Each of them stores the logical child pointers to the bucket pages (as page ids), as well as metadata for handling bucket mapping and dynamic directory growing and shrinking.
///
#[repr(C)]
pub(crate) struct DirectoryPage {
    /// The maximum depth the header page could handle
    max_depth: u32,

    /// The current directory global depth
    global_depth: u32,

    /// An array of bucket page local depths
    local_depths: [u8; DirectoryPage::ARRAY_SIZE],

    /// An array of bucket page ids
    bucket_page_ids: [PageId; DirectoryPage::ARRAY_SIZE],
}


impl DirectoryPage {

    pub(crate) const MAX_DEPTH: u32 = 9;

    /// `ARRAY_SIZE` is the number of page_ids that can fit in the directory page of an extendible hash index.
    /// This is 512 because the directory array must grow in powers of 2, and 1024 page_ids leaves zero room for
    /// storage of the other member variables.
    const ARRAY_SIZE: usize = 1 << Self::MAX_DEPTH;

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
    pub(crate)  fn init(&mut self, max_depth: Option<u32>) {
        self.max_depth = max_depth.unwrap_or(Self::MAX_DEPTH);
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
    pub(crate)  fn hash_to_bucket_index(&self, hash: u32) -> u32 {
        // When global depth is 0 than all goes to the same bucket
        if self.global_depth == 0 {
            return 0;
        }
        // You will want to use the least-significant bits for indexing into the directory page's bucket_page_ids array.
        // This involves taking the hash of your key and perform bit operations with the current depth of the directory page.
        let index = hash.get_n_lsb_bits(self.global_depth as u8);

        assert!(index <= self.max_size(), "bucket index {} must not be above directory max size {}", index, self.max_size());

        index
    }

    /// Lookup a bucket page using a directory index
    ///
    /// # Arguments
    ///
    /// * `bucket_idx`: the index in the directory to lookup
    ///
    /// returns: PageId bucket page_id corresponding to bucket_idx
    ///
    pub(crate)  fn get_bucket_page_id(&self, bucket_idx: u32) -> PageId {
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
    pub(crate)  fn set_bucket_page_id(&mut self, bucket_idx: u32, bucket_page_id: PageId) {
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
    pub(crate)  fn get_split_image_index(&self, _bucket_idx: u32) -> u32 {
        unimplemented!()
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
    pub(crate)  fn get_global_depth_mask(&self) -> u32 {
        u32::MAX.get_n_lsb_bits(self.global_depth as u8)
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
    pub(crate)  fn get_local_depth_mask(&self, bucket_idx: u32) -> u32 {
        let local_depth = self.get_local_depth(bucket_idx);

        if local_depth == 0 {
            0u32
        } else {
            0xFFFFFFFFu32.get_n_lsb_bits(local_depth as u8)
        }
    }

    /// Get the global depth of the hash table directory
    ///
    /// # Arguments
    ///
    /// returns: u32 the global depth of the directory
    ///
    pub(crate)  fn get_global_depth(&self) -> u32 {
        self.global_depth
    }

    pub(crate)  fn get_max_depth(&self) -> u32 {
        self.max_depth
    }

    /// Increment the global depth of the directory
    pub(crate)  fn incr_global_depth(&mut self) -> bool {
        // If reached the max depth
        if self.global_depth + 1 > self.max_depth {
            return false;
        }

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

        true
    }

    /// Decrement the global depth of the directory
    pub(crate)  fn decr_global_depth(&mut self) -> bool {
        if self.global_depth == 0 {
            return false;
        }


        let size_before = self.size() as usize;
        self.global_depth -= 1;
        let size_after = self.size() as usize;

        // Reset page id
        self.bucket_page_ids[size_after..size_before].fill(INVALID_PAGE_ID);
        
        // Reset depth
        self.local_depths[size_after..size_before].fill(0);

        true
    }

    ///
    /// returns: bool true if the directory can be shrunk
    ///
    pub(crate)  fn can_shrink(&self) -> bool {
        self.local_depths.iter().all(|&d| (d as u32) < self.global_depth)
    }

    ///
    /// returns: u32 the current directory size
    ///
    pub(crate)  fn size(&self) -> u32 {
        2u32.pow(self.global_depth)
    }

    ///
    /// returns: u32 the max directory size
    ///
    pub(crate)  fn max_size(&self) -> u32 {
        2u32.pow(self.max_depth)
    }

    /// Gets the local depth of the bucket at bucket_idx
    ///
    /// # Arguments
    ///
    /// * `bucket_idx`: the bucket index to lookup
    ///
    /// returns: u32 the local depth of the bucket at bucket_idx
    ///
    pub(crate)  fn get_local_depth(&self, bucket_idx: u32) -> u32 {
        self.local_depths[bucket_idx as usize] as u32
    }


    /// Set the local depth of the bucket at bucket_idx to local_depth
    ///
    /// # Arguments
    ///
    /// * `bucket_idx`: the bucket index to update
    /// * `local_depth`: new local depth
    ///
    pub(crate)  fn set_local_depth(&mut self, bucket_idx: u32, local_depth: u8) {
        // assert!((local_depth as u32 )<= self.global_depth, "Local depth cant be larger than global depth");
        self.local_depths[bucket_idx as usize] = local_depth;
    }

    /// Increment the local depth of the bucket at bucket_idx
    ///
    /// # Arguments
    ///
    /// * `bucket_idx`: bucket index to increment
    ///
    pub(crate)  fn incr_local_depth(&mut self, bucket_idx: u32) {
        assert!((self.local_depths[bucket_idx as usize] as u32) < self.global_depth, "can't increment more than the global depth {}", self.global_depth);

        self.local_depths[bucket_idx as usize] += 1;
    }

    /// Decrement the local depth of the bucket at bucket_idx
    ///
    /// # Arguments
    ///
    /// * `bucket_idx`: bucket index to decrement
    ///
    pub(crate)  fn decr_local_depth(&mut self, bucket_idx: u32) {
        assert!(self.local_depths[bucket_idx as usize] + 1 > 0, "local depth is already 0");

        self.local_depths[bucket_idx as usize] -= 1;
    }
    //
    // pub fn get_addition_status(&self, bucket_idx: u32) -> AdditionRequirement {
    //
    // }

    /// Verify the following invariants:
    /// (1) All LD <= GD.
    /// (2) Each bucket has precisely 2^(GD - LD) pointers pointing to it.
    /// (3) The LD is the same at each index with the same bucket_page_id
    ///
    pub(crate)  fn verify_integrity(&self, print_directory_on_failure: bool) {
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
                if print_directory_on_failure {
                    self.print_directory();
                }
                assert_eq!(curr_ld, *page_id_to_ld.get_or(&curr_page_id, &0), "local depth is not the same at each index with same bucket page id for page id {}", curr_page_id);
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
                if print_directory_on_failure {
                    self.print_directory();
                }
                assert_eq!(curr_count, required_count, "a bucket does not have precisely 2^(GD - LD) pointers to it")
            }
        }
    }

    /// Prints the current directory
    pub(crate)  fn print_directory(&self) {
        println!("{:?}", self)
    }

    pub(crate)  fn extended_format<GetKeysForBucketFn: Fn(PageId) -> Vec<String>>(&self, f: &mut Formatter<'_>, get_keys_for_bucket: GetKeysForBucketFn) -> std::fmt::Result {
        f.write_str(format!("======== DIRECTORY (global_depth: {}) ========\n", self.global_depth).as_str())?;

        let mut table = Table::new();

        table.add_row(row!["prefix", "page_id", "local_depth", "keys"]);

        let max_id = (0x1 << self.global_depth) as u32;
        for idx in 0..(max_id as usize) {
            let page_id = self.bucket_page_ids[idx];
            let local_depth = self.local_depths[idx];

            let keys_for_bucket = get_keys_for_bucket(page_id);

            table.add_row(row![
                format_number_in_bits(idx as u64, self.global_depth),
                page_id,
                local_depth,
                keys_for_bucket.join(", ")
            ]);
        }

        f.write_str(table.to_string().as_str())?;

        f.write_str("================ END DIRECTORY ================\n")
    }
}

impl Debug for DirectoryPage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("======== DIRECTORY (global_depth: {}) ========\n", self.global_depth).as_str())?;

        let mut table = Table::new();

        table.add_row(row!["prefix", "page_id", "local_depth"]);

        let max_id = (0x1 << self.global_depth) as u32;
        for idx in 0..(max_id as usize) {
            let page_id = self.bucket_page_ids[idx];
            let local_depth = self.local_depths[idx];

            table.add_row(row![format_number_in_bits(idx as u64, self.global_depth), page_id, local_depth]);
        }

        f.write_str(table.to_string().as_str())?;

        f.write_str("================ END DIRECTORY ================\n")
    }
}

fn format_number_in_bits(n: u64, number_of_bits: u32) -> String {
    format!("{n:#064b}")[64 - number_of_bits as usize..].to_string()
}


#[cfg(test)]
mod tests {
    use buffer_pool_manager::{BufferPool, BufferPoolManager};
    use buffer_common::AccessType;

    use parking_lot::Mutex;
    use std::sync::Arc;
    use disk_storage::DiskManagerUnlimitedMemory;
    use crate::directory_page::DirectoryPage;

    #[test]
    fn global_and_local_depth_mash() {
        let disk_mgr = Arc::new(Mutex::new(DiskManagerUnlimitedMemory::new()));
        let bpm = BufferPoolManager::new(5, disk_mgr, None, None);


        // Create directory
        let mut directory_guard = bpm.new_page(AccessType::Unknown).expect("Should be able to create new page");

        let directory_page = directory_guard.cast_mut::<DirectoryPage>();
        directory_page.init(Some(3));

        assert_eq!(directory_page.global_depth, 0);
        assert_eq!(directory_page.get_global_depth_mask(), 0);

        for i in 0..directory_page.local_depths.len() {
            assert_eq!(directory_page.local_depths[i], 0);
            assert_eq!(directory_page.get_local_depth_mask(i as u32), 0);
        }

        // grow the directory, local depths should change!
        directory_page.set_local_depth(0, 1);
        directory_page.incr_global_depth();
        directory_page.set_local_depth(1, 1);

        assert_eq!(directory_page.global_depth, 1);
        assert_eq!(directory_page.get_global_depth_mask(), 0x00000001u32);

        // TODO - add more tests for the mask
    }
}
