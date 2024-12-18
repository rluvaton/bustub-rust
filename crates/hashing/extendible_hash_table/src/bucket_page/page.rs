use super::errors::InsertionErrors;
use comfy_table::Table;
use common::{Comparator, OrdComparator, PageKey, PageValue};
use pages::PAGE_SIZE;
use std::cmp::Ordering;
use std::fmt::{Debug, Formatter};
use std::marker::PhantomData;
use std::mem::size_of;
use crate::bucket_page::{BucketIter, BucketPageIterState};

pub type MappingType<KeyType, ValueType> = (KeyType, ValueType);

//noinspection RsAssertEqual
const _: () = {
    type Key = u64;
    type Value = u16;
    // Assert that the comparator phantom data does not affecting size
    assert!(
        size_of::<BucketPage<{ bucket_array_size::<Key, Value>() }, Key, Value, OrdComparator<Key>>>() ==
            0 +
                // size
                size_of::<u32>() +

                // max size
                size_of::<u32>() +

                // array
                size_of::<MappingType<Key, Value>>() * bucket_array_size::<Key, Value>()
    );
};

const PAGE_METADATA_SIZE: usize = size_of::<u32>() * 2;

pub const fn bucket_array_size<Key, Value>() -> usize {
    (PAGE_SIZE - PAGE_METADATA_SIZE) / size_of::<MappingType<Key, Value>>()
}

#[macro_export]
macro_rules! bucket_page_type {
    ($key:ty, $value:ty, $cmp:ty) => {
        BucketPage<{bucket_array_size::<$key, $value>()}, $key, $value, $cmp>
    };
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
#[repr(C)]
// TODO - replace the repr(C)
pub(crate) struct BucketPage<const ARRAY_SIZE: usize, Key, Value, KeyComparator>
where
    Key: PageKey,
    Value: PageValue,
    KeyComparator: Comparator<Key>,
{
    /// The number of key-value pairs the bucket is holding
    pub(super) size: u32,

    /// The maximum number of key-value pairs the bucket can handle
    pub(super) max_size: u32,

    /// The array that holds the key-value pairs
    pub(super) array: [MappingType<Key, Value>; ARRAY_SIZE],

    _key_comparator: PhantomData<KeyComparator>,
}


// TODO - maybe instead of comparator use the Partial Eq function
impl<const ARRAY_SIZE: usize, Key, Value, KeyComparator> BucketPage<ARRAY_SIZE, Key, Value, KeyComparator>
where
    Key: PageKey,
    Value: PageValue,
    KeyComparator: Comparator<Key>,
{
    /// Assert that the array size generic is ok
    /// this will not show pretty stack trace and will not be evaluated unless called explicitly,
    /// but it's the best we can do for know
    /// Won't be needed once [`feature(generic_const_exprs)`](https://github.com/rust-lang/rust/issues/76560) is stable
    // noinspection RsAssertEqual
    pub(crate) const ARRAY_SIZE_OK: () = assert!(ARRAY_SIZE == bucket_array_size::<Key, Value>(), "ArraySize generic was not hash_table_bucket_array_size::<Key, ValueType>()");

    // Delete all constructor / destructor to ensure memory safety
    // TODO - delete destructor?


    /**
     * After creating a new bucket page from buffer pool, must call initialize
     * method to set default values
     * @param max_size Max size of the bucket array
     */
    pub(crate) fn init(&mut self, max_size: Option<u32>) {
        // Validate correct generic at compile time
        let _ = Self::ARRAY_SIZE_OK;
        let max_size = max_size.unwrap_or(ARRAY_SIZE as u32);
        assert!(max_size <= ARRAY_SIZE as u32, "Max size must be smaller than ARRAY_SIZE");

        self.size = 0;
        self.max_size = max_size;
    }

    /// Lookup a key
    ///
    /// # Arguments
    ///
    /// * `key`: key to lookup
    /// * `comparator`: the comparator
    ///
    /// returns: Option<& Value> None if the key was missing, Some with reference to the found value if not
    ///
    pub(crate) fn lookup(&self, key: &Key, comparator: &KeyComparator) -> Option<&Value> {
        self
            .iter()
            .find(|(item_key, _)| comparator.cmp(key, &item_key) == Ordering::Equal)
            .map(|(_key, value)| value)
    }

    /**
     * Attempts to insert a key and value in the bucket.
     *
     * @param key key to insert
     * @param value value to insert
     * @param cmp the comparator to use
     * @return true if inserted, false if bucket is full or the same key is already present
     */
    pub(crate) fn insert(&mut self, key: &Key, value: &Value, comparator: &KeyComparator) -> Result<(), InsertionErrors> {
        if self.is_full() {
            return Err(InsertionErrors::BucketIsFull);
        }

        let missing = self.array[0..self.size as usize]
            .iter()
            .all(|(item_key, _)| comparator.cmp(key, &item_key) != Ordering::Equal);

        if !missing {
            return Err(InsertionErrors::KeyAlreadyExists);
        }

        // TODO - No need to clone as key and value support copy
        let entry: MappingType<Key, Value> = (key.clone(), value.clone());

        self.array[self.size as usize] = entry;
        self.size += 1;

        Ok(())
    }

    /// Replace a value in key
    ///
    /// If the value does not exist it will not set it to the new value
    ///
    /// Returns true if value changed and false if not (the key is missing)
    pub(crate) fn replace(&mut self, key: &Key, new_value: &Value, comparator: &KeyComparator) -> bool {
        if self.is_empty() {
            return false;
        }

        let bucket_index = self.get_bucket_idx_by_key(key, comparator);

        if let Some(bucket_index) = bucket_index {
            if bucket_index >= self.size as usize {
                return false;
            }

            self.array[bucket_index].1 = new_value.clone();


            return true;
        }

        false
    }

    /**
     * Removes a key and value.
     *
     * @return true if removed, false if not found
     */
    pub(crate) fn remove(&mut self, key: &Key, comparator: &KeyComparator) -> bool {
        if self.is_empty() {
            return false;
        }

        let bucket_index = self.get_bucket_idx_by_key(key, comparator);
        if let Some(bucket_index) = bucket_index {
            self.remove_at(bucket_index as u32);

            return true;
        }

        false
    }

    pub(crate) fn remove_at(&mut self, bucket_idx: u32) {
        if bucket_idx >= self.size {
            return;
        }

        // If not the last item replace it with the last item and decrease size
        if bucket_idx != self.size - 1 {
            self.array.swap(bucket_idx as usize, self.size as usize - 1);
        }

        self.size -= 1;
    }

    /**
     * @brief Gets the key at an index in the bucket.
     *
     * @param bucket_idx the index in the bucket to get the key at
     * @return key at index bucket_idx of the bucket
     */
    pub(crate) fn key_at(&self, bucket_idx: u32) -> &Key {
        &self.entry_at(bucket_idx).0
    }

    /**
     * Gets the value at an index in the bucket.
     *
     * @param bucket_idx the index in the bucket to get the value at
     * @return value at index bucket_idx of the bucket
     */
    pub(crate) fn value_at(&self, bucket_idx: u32) -> &Value {
        &self.entry_at(bucket_idx).1
    }

    /**
     * Gets the entry at an index in the bucket.
     *
     * @param bucket_idx the index in the bucket to get the entry at
     * @return entry at index bucket_idx of the bucket
     */
    pub(crate) fn entry_at(&self, bucket_idx: u32) -> &MappingType<Key, Value> {
        &self.array[bucket_idx as usize]
    }

    // Replace all entries with different one, this is useful for rehashing
    pub(crate) fn replace_all_entries(&mut self, new_items: &[MappingType<Key, Value>]) {
        assert!(self.max_size as usize >= new_items.len(), "can't insert more items than bucket can hold");

        // TODO - should do swap to the memory or something for better perf?
        self.array[0..new_items.len()].clone_from_slice(new_items);

        self.size = new_items.len() as u32;
    }

    /**
     * @return number of entries in the bucket
     */
    pub(crate) fn size(&self) -> u32 {
        self.size
    }

    /**
     * @return whether the bucket is full
     */
    pub(crate) fn is_full(&self) -> bool {
        self.size == self.max_size
    }

    /**
     * @return whether the bucket is empty
     */
    pub(crate) fn is_empty(&self) -> bool {
        self.size == 0
    }

    /// Prints the bucket's occupancy information
    #[allow(unused)]
    pub(crate) fn print_bucket(&self) {
        println!("{:?}", self)
    }

    fn get_bucket_idx_by_key(&self, key: &Key, comparator: &KeyComparator) -> Option<usize> {
        self.array[..self.size() as usize].iter().position(|item| comparator.cmp(key, &item.0) == Ordering::Equal)
    }

    pub(crate) fn iter(&self) -> BucketIter<ARRAY_SIZE, Key, Value, KeyComparator> {
        BucketIter::new(self)
    }

    pub(crate) fn resume_iter(&self, state: BucketPageIterState) -> BucketIter<ARRAY_SIZE, Key, Value, KeyComparator> {
        BucketIter::with_state(self, state)
    }
}

impl<const ARRAY_SIZE: usize, Key: PageKey, Value: PageValue, KeyComparator: Comparator<Key>> Debug for BucketPage<ARRAY_SIZE, Key, Value, KeyComparator> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("======== BUCKET (size: {} | max_size: {}) ========\n", self.size, self.max_size).as_str())?;

        let mut table = Table::new();

        table.set_header(vec!["index", "key", "value"]);

        for idx in 0..self.size {
            table.add_row(vec![idx.to_string(), format!("{:?}", self.key_at(idx)), self.value_at(idx).to_string()]);
        }

        f.write_str(table.to_string().as_str())?;
        f.write_str("\n================ END BUCKET ================\n")
    }
}


#[cfg(test)]
mod tests {
    use crate::bucket_array_size;
    use crate::bucket_page::{test_utils, BucketPage};
    use buffer_common::AccessType;
    use buffer_pool_manager::{BufferPool, BufferPoolManager};
    use common::OrdComparator;

    #[test]
    fn should_find_all_inserted_values() {
        type Key = u64;
        type Value = u64;
        type Entry = (Key, Value);

        let cmp = OrdComparator::default();

        let bpm = BufferPoolManager::builder().build_arc();
        let mut new_page = bpm.new_page(AccessType::Unknown).expect("Create new page");
        let bucket_page = new_page.cast_mut::<bucket_page_type!(Key, Value, OrdComparator<Key>)>();
        bucket_page.init(None);

        assert!(bucket_page.is_empty(), "bucket should be empty");

        let entries = test_utils::insert_until_full(bucket_page);

        // Bucket page should only have the inserted entries
        for (key, value) in entries {
            let actual_value = bucket_page.lookup(&key, &cmp);

            assert_eq!(actual_value, Some(&value), "Page value should be the inserted one");
        }
    }
}
