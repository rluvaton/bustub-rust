use crate::{GenericComparator, GenericKey, Index, IndexMetadata};
use buffer_pool_manager::BufferPoolManager;
use common::{Comparator, PageKey};
use error_utils::{ToAnyhow, ToAnyhowResult};
use extendible_hash_table::{DiskExtendibleHashTable, bucket_array_size};
use hashing_common::{DefaultKeyHasher, KeyHasher};
use rid::RID;
use std::sync::Arc;
use table::TableHeap;
use transaction::Transaction;
use tuple::Tuple;

pub struct ExtendibleHashingIndex<const BUCKET_MAX_SIZE: usize, Key: PageKey, KeyComparator: Comparator<Key>, KeyHasherImpl: KeyHasher> (
    DiskExtendibleHashTable<
        BUCKET_MAX_SIZE,
        Key,
        RID,
        KeyComparator,
        KeyHasherImpl
    >
);


macro_rules! impl_extendible_hashing_index_for_generic_key {
    ($($helper_type:ident, $key_size:literal)+) => ($(

pub type $helper_type = ExtendibleHashingIndex<{ bucket_array_size::<GenericKey<$key_size>, RID>() }, GenericKey<$key_size>, GenericComparator<$key_size>, DefaultKeyHasher>;

impl ExtendibleHashingIndex<{ bucket_array_size::<GenericKey<$key_size>, RID>() }, GenericKey<$key_size>, GenericComparator<$key_size>, DefaultKeyHasher> {
    pub fn new(metadata: Arc<IndexMetadata>, bpm: Arc<BufferPoolManager>) -> Result<Box<dyn Index>, extendible_hash_table::errors::InitError> {
        DiskExtendibleHashTable::new(
            metadata.get_name().to_string(),
            bpm,
            GenericComparator::<$key_size>::from(metadata.get_key_schema()),
            None,
            None,
            None
        ).map(|h| Self(h).to_dyn_box())
    }
}

impl Index for ExtendibleHashingIndex<{ bucket_array_size::<GenericKey<$key_size>, RID>() }, GenericKey<$key_size>, GenericComparator<$key_size>, DefaultKeyHasher> {
    fn insert_entry(&self, key: &Tuple, rid: RID, transaction: &Transaction) -> error_utils::anyhow::Result<()> {
        self.0.insert(&GenericKey::from(key), &rid, transaction).map_err(|err| err.to_anyhow())
    }

    fn delete_entry(&self, key: &Tuple, _rid: RID, transaction: &Transaction) -> error_utils::anyhow::Result<()> {
        self.0.remove(&GenericKey::from(key), transaction)
            .map(|_| ())
            .map_err(|err| err.to_anyhow())
    }

    fn scan_key(&self, key: &Tuple, transaction: &Transaction) -> error_utils::anyhow::Result<Vec<RID>> {
        self.0.get_value(&GenericKey::from(key), transaction)
            .map_err(|err| err.to_anyhow())
    }
    
    fn verify_integrity(&self, index_metadata: &IndexMetadata, table_heap: &TableHeap, transaction: &Transaction) {
        self.0.verify_integrity(false);
        
        if index_metadata.is_primary_key() {
            let table_heap_count = table_heap.iter().count();

            let all_table_heap_values_are_in_the_index = table_heap
                .iter()
                .all(|(_, tuple)| {
                    self.0.get_value(&GenericKey::from(&tuple), transaction) == Ok(vec![*tuple.get_rid()])
                });

            assert!(all_table_heap_values_are_in_the_index, "There should be for every table heap an entry in the index for primary key index");

            let index_count = self.0.iter().count();
            
            assert_eq!(table_heap_count, index_count, "Table heap count should have the same amount of items as the primary key index items");
        }
    }
    
    fn delete_completely(self: Box<Self>, transaction: &Transaction) -> error_utils::anyhow::Result<()> {
        self.0.delete_completely(transaction).to_anyhow()
    }
}

    )+)
}


impl_extendible_hashing_index_for_generic_key! {
    ExtendibleHashingIndex8, 1
    ExtendibleHashingIndex16, 2
    ExtendibleHashingIndex32, 4
    ExtendibleHashingIndex64, 8
}

pub fn create_extendible_hashing_index(key_size: usize, metadata: Arc<IndexMetadata>, bpm: Arc<BufferPoolManager>) -> Result<Box<dyn Index>, extendible_hash_table::errors::InitError> {
    match key_size {
        1 => ExtendibleHashingIndex8::new(metadata, bpm),
        2 => ExtendibleHashingIndex16::new(metadata, bpm),
        4 => ExtendibleHashingIndex32::new(metadata, bpm),
        8 => ExtendibleHashingIndex64::new(metadata, bpm),
        _ => panic!("Unimplemented extendible hash index for key size {}", key_size)
    }
}


// this match i64/u64
pub const TWO_INTEGER_SIZE: usize = 8;
pub type IntegerKeyType = GenericKey<TWO_INTEGER_SIZE>;
pub type IntegerValueType = RID;
pub type IntegerComparatorType = GenericComparator<TWO_INTEGER_SIZE>;
pub type HashTableIndexForTwoIntegerColumn = ExtendibleHashingIndex64;
