use rid::RID;
use tuple::Tuple;

type Key = Tuple;
type Value = RID;
//
// pub type ExtendibleHashingIndex<KeyHasherImpl>
// where
//     KeyHasherImpl: KeyHasher
// = DiskHashTable<{bucket_array_size::<Key, Value>() } , Key, Value, GenericComparator<>, KeyHasherImpl>
//
//
// impl<const BUCKET_MAX_SIZE: usize, Key, Value, KeyComparator, KeyHasherImpl> extendible_hash_table::DiskHashTable<BUCKET_MAX_SIZE, Key, Value, KeyComparator, KeyHasherImpl>
// where
//     Key: PageKey,
//     Value: PageValue,
//     KeyComparator: Comparator<Key>,
//     KeyHasherImpl: KeyHasher,
// {
//     pub const BUCKET_ARRAY_SIZE_OK: () = <Self as TypeAliases>::BucketPage::ARRAY_SIZE_OK;
//
//     /// @brief Creates a new DiskExtendibleHashTable.
//     ///
//     /// # Arguments
//     ///
//     /// - `name`:
//     /// - `bpm`: bpm buffer pool manager to be used
//     /// - `cmp`: comparator for keys
//     /// - `hash_fn`: the hash function
//     /// - `header_max_depth`: the max depth allowed for the header page
//     /// - `directory_max_depth`: the max depth allowed for the directory page
//     /// - `bucket_max_size`: the max size allowed for the bucket page array
//     ///
//     pub fn new(name: String, bpm: Arc<BufferPoolManager>, cmp: KeyComparator, header_max_depth: Option<u32>, directory_max_depth: Option<u32>, bucket_max_size: Option<u32>) -> Result<Self, errors::InitError> {
//     }
// }
//
//
//
// impl Index for ExtendibleHashingIndex<BUCKET_MAX_SIZE, Key, Value, KeyComparator, KeyHasherImpl> {
//
// }
