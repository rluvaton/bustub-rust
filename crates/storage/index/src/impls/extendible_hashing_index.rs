use std::fs::Metadata;
use crate::{GenericComparator, GenericKey, Index, IndexMetadata};
use buffer_pool_manager::BufferPoolManager;
use common::{Comparator, PageKey};
use error_utils::ToAnyhow;
use extendible_hash_table::{DiskExtendibleHashTable, bucket_array_size};
use hashing_common::KeyHasher;
use rid::RID;
use std::sync::Arc;
use transaction::Transaction;
use tuple::Tuple;

struct ExtendibleHashingIndex<const BUCKET_MAX_SIZE: usize, Key: PageKey, KeyComparator: Comparator<Key>, KeyHasherImpl: KeyHasher> (
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

pub type $helper_type<KeyHasherImpl: KeyHasher> = ExtendibleHashingIndex<{ bucket_array_size::<GenericKey<$key_size>, RID>() }, GenericKey<$key_size>, GenericComparator<$key_size>, KeyHasherImpl>;

impl<KeyHasherImpl: KeyHasher> ExtendibleHashingIndex<{ bucket_array_size::<GenericKey<$key_size>, RID>() }, GenericKey<$key_size>, GenericComparator<$key_size>, KeyHasherImpl> {
    pub fn new(metadata: Arc<IndexMetadata>, bpm: Arc<BufferPoolManager>) -> Result<Self, extendible_hash_table::errors::InitError> {
        Ok(Self(DiskExtendibleHashTable::new(
            metadata.get_name().to_string(),
            bpm,
            GenericComparator::<$key_size>::from(metadata.get_key_schema()),
            None,
            None,
            None
        )?))
    }
}

impl<KeyHasherImpl: KeyHasher> Index for ExtendibleHashingIndex<{ bucket_array_size::<GenericKey<$key_size>, RID>() }, GenericKey<$key_size>, GenericComparator<$key_size>, KeyHasherImpl> {
    fn insert_entry(&mut self, key: &Tuple, rid: RID, transaction: Option<Arc<Transaction>>) -> error_utils::anyhow::Result<()> {
        self.0.insert(&GenericKey::from(key), &rid, transaction).map_err(|err| err.to_anyhow())
    }

    fn delete_entry(&mut self, key: &Tuple, _rid: RID, transaction: Option<Arc<Transaction>>) -> error_utils::anyhow::Result<()> {
        self.0.remove(&GenericKey::from(key), transaction)
            .map(|_| ())
            .map_err(|err| err.to_anyhow())
    }

    fn scan_key(&self, key: &Tuple, transaction: Option<Arc<Transaction>>) -> error_utils::anyhow::Result<Vec<RID>> {
        self.0.get_value(&GenericKey::from(key), transaction)
            .map_err(|err| err.to_anyhow())
    }
}

    )+)
}

impl_extendible_hashing_index_for_generic_key! {
    ExtendibleHashingIndex4, 4
    ExtendibleHashingIndex8, 8
    ExtendibleHashingIndex16, 16
    ExtendibleHashingIndex32, 32
    ExtendibleHashingIndex64, 64
}

pub fn create_index_based_on_key_size(key_size: usize, metadata: Arc<IndexMetadata>, bpm: Arc<BufferPoolManager>) -> Arc<dyn Index> {
    match key_size {
        4 => Arc::new(ExtendibleHashingIndex4::new(metadata, bpm)),
        8 => Arc::new(ExtendibleHashingIndex8::new(metadata, bpm)),
        16 => Arc::new(ExtendibleHashingIndex16::new(metadata, bpm)),
        32 => Arc::new(ExtendibleHashingIndex32::new(metadata, bpm)),
        64 => Arc::new(ExtendibleHashingIndex64::new(metadata, bpm)),
        _ => panic!("Unimplemented extendible hash index for key size {}", key_size)
    }
}
