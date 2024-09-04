use std::cmp::Ordering;
use crate::storage::page::b_plus_tree::MappingType;
use common::config::{PageId, BUSTUB_PAGE_SIZE};
use std::fmt::{Debug, Display, Formatter};
use std::marker::PhantomData;
use std::mem::size_of;

pub trait KeyComparatorFn<Key: Sized> {
    fn cmp(a: &Key, b: &Key) -> Ordering;
}


// Test assertion helper type
const _ASSERTION_TEST_TYPE: usize = hash_table_bucket_array_size::<u8, u8>();

struct AssertionTest {}

impl KeyComparatorFn<u8> for AssertionTest {
    fn cmp(_a: &u8, _b: &u8) -> Ordering {
        unreachable!()
    }
}
//noinspection RsAssertEqual
const _: () = {
    // Assert that the comparator phantom data does not affecting size
    assert!(
        size_of::<BucketPage<_ASSERTION_TEST_TYPE, u8, u8, AssertionTest>>() ==
            0 +
            // size
                size_of::<u32>() +

                // max size
                size_of::<u32>() +

                // array
                size_of::<MappingType<u8, u8>>() * _ASSERTION_TEST_TYPE
    );
};

const HASH_TABLE_BUCKET_PAGE_METADATA_SIZE: usize = size_of::<u32>() * 2;

pub const fn hash_table_bucket_array_size<Key, Value>() -> usize {
    (BUSTUB_PAGE_SIZE - HASH_TABLE_BUCKET_PAGE_METADATA_SIZE) / size_of::<MappingType<Key, Value>>()
}


/// Bucket pages sit at the third level of our disk-based extendible hash table. They are the ones that are actually storing the key-value pairs.
///
/// Bucket page format:
///  ----------------------------------------------------------------------------
/// | METADATA | KEY(1) + VALUE(1) | KEY(2) + VALUE(2) | ... | KEY(n) + VALUE(n)
///  ----------------------------------------------------------------------------
///
/// Metadata format (size in byte, 8 bytes in total):
///  --------------------------------
/// | CurrentSize (4) | MaxSize (4)
///  --------------------------------
///
/// The `ArraySize` generic must be the value of `hash_table_bucket_array_size`
/// This can be removed once [`feature(generic_const_exprs)`](https://github.com/rust-lang/rust/issues/76560) is stable
///
/// # Examples
/// ```rust
/// use std::cmp::Ordering;
/// use storage::storage::{ExtendibleHashTableBucketPage, hash_table_bucket_array_size, ExtendibleHashTableBucketKeyComparatorFn};
///
/// struct AssertionTest {}
///
/// impl ExtendibleHashTableBucketKeyComparatorFn<u8> for AssertionTest {
///     fn cmp(_a: &u8, _b: &u8) -> Ordering {
///         unreachable!()
///     }
/// }
///
/// const a: usize = hash_table_bucket_array_size::<u8, u8>();
///
/// type B = ExtendibleHashTableBucketPage::<a, u8, u8, AssertionTest>;
///
/// let _ = B::ARRAY_SIZE_OK;
/// ```
///
/// When the bucket page size is not the hash_table_bucket_array_size size
/// ```compile_fail
/// use std::cmp::Ordering;
/// use storage::{ExtendibleHashTableBucketPage, hash_table_bucket_array_size, ExtendibleHashTableBucketKeyComparatorFn};
///
/// struct AssertionTest {}
///
/// impl ExtendibleHashTableBucketKeyComparatorFn<u8> for AssertionTest {
///     fn cmp(_a: &u8, _b: &u8) -> Ordering {
///         unreachable!()
///     }
/// }
///
/// const a: usize = hash_table_bucket_array_size::<u8, u8>() - 1;
///
/// type B = ExtendibleHashTableBucketPage::<a, u8, u8, AssertionTest>;
///
/// let _ = B::ARRAY_SIZE_OK;
/// ```
///
#[repr(packed)]
pub struct BucketPage<const ARRAY_SIZE: usize, Key, Value, KeyComparator>
where
    Key: Sized + Display,
    Value: Sized + Display + PartialEq,
    KeyComparator: KeyComparatorFn<Key>,
{
    /// The number of key-value pairs the bucket is holding
    size: u32,

    /// The maximum number of key-value pairs the bucket can handle
    max_size: u32,

    /// The array that holds the key-value pairs
    array: [MappingType<Key, Value>; ARRAY_SIZE],

    // This will not affect the struct size in memory
    _comparator: PhantomData<KeyComparator>,
}


// TODO - maybe instead of comparator use the Partial Eq function
impl<const ARRAY_SIZE: usize, Key, Value, KeyComparator> BucketPage<ARRAY_SIZE, Key, Value, KeyComparator>
where
    Key: Sized + Display,
    Value: Sized + Display + PartialEq,
    KeyComparator: KeyComparatorFn<Key>,
{
    /// Assert that the array size generic is ok
    /// this will not show pretty stack trace and will not be evaluated unless called explicitly,
    /// but it's the best we can do for know
    /// Won't be needed once [`feature(generic_const_exprs)`](https://github.com/rust-lang/rust/issues/76560) is stable
    // noinspection RsAssertEqual
    pub const ARRAY_SIZE_OK: () = assert!(ARRAY_SIZE == hash_table_bucket_array_size::<Key, Value>(), "ArraySize generic was not hash_table_bucket_array_size::<Key, ValueType>()");

    // Delete all constructor / destructor to ensure memory safety
    // TODO - delete destructor?


    /**
     * After creating a new bucket page from buffer pool, must call initialize
     * method to set default values
     * @param max_size Max size of the bucket array
     */
    pub fn init(&mut self, max_size: Option<u32>) {
        // Validate correct generic at compile time
        let _ = Self::ARRAY_SIZE_OK;
        let max_size = max_size.unwrap_or(hash_table_bucket_array_size::<Key, Value>() as u32);

        self.size = 0;
        self.max_size = max_size;
        // self.array = [None; ARRAY_SIZE];
    }

    /**
     * Lookup a key
     *
     * @param key key to lookup
     * @param[out] value value to set
     * @param cmp the comparator
     * @return true if the key and value are present, false if not found.
     */
    // TODO - use the comparor
    pub fn lookup(&self, key: &Key, value: &Value, cmp: &KeyComparator) -> bool {
        unimplemented!();
        // for i in 0..self.size as usize {
        //     let current_pair = &self.array[i];
        //
        //     // TODO - should do binary search? the values are not ordered
        //     if KeyComparator::cmp(key, &current_pair.0) == Ordering::Equal && *value == current_pair.1 {
        //         return true;
        //     }
        // }
        //
        // false
    }

    /**
     * Attempts to insert a key and value in the bucket.
     *
     * @param key key to insert
     * @param value value to insert
     * @param cmp the comparator to use
     * @return true if inserted, false if bucket is full or the same key is already present
     */
    pub fn insert(&mut self, key: &Key, value: &Value, cmp: &KeyComparator) -> bool {
        unimplemented!();
        //
        // if self.is_full() {
        //     return false;
        // }
        //
        // for i in 0..self.size as usize {
        //     let current_pair = &self.array[i];
        //
        //     // TODO - should do binary search? the values are not ordered
        //     if KeyComparator::cmp(key, &current_pair.0) == Ordering::Equal && *value == current_pair.1 {
        //         return true;
        //     }
        // }
        //
        // false
    }

    /**
     * Removes a key and value.
     *
     * @return true if removed, false if not found
     */
    pub fn remove(&mut self, key: &Key, cmp: &KeyComparator) -> bool {
        if self.is_empty() {
            return false;
        }

        // TODO - can do binary search?
        let bucket_index = self.get_bucket_idx_by_key(key, cmp);
        if let Some(bucket_index) = bucket_index {
            self.remove_at(bucket_index as u32);

            return true;
        }

        false
    }

    pub fn remove_at(&mut self, bucket_idx: u32) {
        unimplemented!();

        //
        // if bucket_idx >= self.size {
        //     return;
        // }
        //
        // // If not the last item replace it with the last item and decrease size
        // if bucket_idx != self.size - 1 {
        //     self.array.swap(bucket_idx as usize, self.size as usize - 1);
        // }
        //
        // self.size -= 1;
        //
        // // TODO - MARK DELETED?
        // // TODO - CAN CHANGE LOCATION? - SWAP THE ABOUT TO DELETE WITH THE LAST AND DECREASE SIZE
        // // unimplemented!();
    }

    /**
     * @brief Gets the key at an index in the bucket.
     *
     * @param bucket_idx the index in the bucket to get the key at
     * @return key at index bucket_idx of the bucket
     */
    pub fn key_at(&self, bucket_idx: u32) -> Key {
        // &self.entry_at(bucket_idx).0;
        unimplemented!()
    }

    /**
     * Gets the value at an index in the bucket.
     *
     * @param bucket_idx the index in the bucket to get the value at
     * @return value at index bucket_idx of the bucket
     */
    pub fn value_at(&self, bucket_idx: u32) -> Value {
        // &self.entry_at(bucket_idx).1
        unimplemented!()
    }

    /**
     * Gets the entry at an index in the bucket.
     *
     * @param bucket_idx the index in the bucket to get the entry at
     * @return entry at index bucket_idx of the bucket
     */
    pub fn entry_at(&self, bucket_idx: u32) -> &MappingType<Key, Value> {
        // &self.array[bucket_idx as usize]
        unimplemented!()

    }

    /**
     * @return number of entries in the bucket
     */
    pub fn size(&self) -> u32 {
        self.size
    }

    /**
     * @return whether the bucket is full
     */
    pub fn is_full(&self) -> bool {
        self.size == self.max_size
    }

    /**
     * @return whether the bucket is empty
     */
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    /// Prints the bucket's occupancy information
    pub fn print_bucket(&self) {
        println!("{:?}", self)
    }

    fn get_bucket_idx_by_key(&self, key: &Key, cmp: &KeyComparator) -> Option<usize> {
        // self.array[..self.size() as usize].iter().position(|item| cmp(key, &item.0) == Ordering::Equal)
        unimplemented!()

    }

}

impl<const ARRAY_SIZE: usize, Key: Sized + Display, Value: Sized + Display + PartialEq, KeyComparator: KeyComparatorFn<Key>> Debug for BucketPage<ARRAY_SIZE, Key, Value, KeyComparator> {

    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        unimplemented!();

        // f.write_str(format!("======== BUCKET (size_: {} | max_size_: {}) ========\n", self.size, self.max_size).as_str())?;
        // f.write_str("| i | k | v |\n")?;
        //
        // for idx in 0..self.size {
        //     f.write_str(format!("| {} | {} | {} |\n", idx, self.key_at(idx), self.value_at(idx)).as_str())?;
        // }
        //
        // f.write_str("================ END BUCKET ================\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn assert_size() {
        const a: usize = hash_table_bucket_array_size::<u8, u8>();

        let b = BucketPage::<a, u8, u8, AssertionTest> {
            size: 0,
            max_size: 0,
            array: [(0, 0); a],
            _comparator: PhantomData
        };

        // let () = BucketPage::<a, u8, u8>::OK;

    }
}

