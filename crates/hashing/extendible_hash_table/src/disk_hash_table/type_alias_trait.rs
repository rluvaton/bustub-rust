use common::{Comparator, PageKey, PageValue};
use hashing_common::KeyHasher;
use crate::{DiskHashTable, HeaderPage, DirectoryPage, BucketPage};

/// Type aliases to be used for easy access to types with generic or other
///
/// Use `<Self as TypeAliases>::HeaderPage` for example
///
/// ```
///
/// ```
pub(super) trait TypeAliases {

    /// The type of the header page
    ///
    /// Will be equal to `ExtendibleHashTableHeaderPage`
    type HeaderPage;

    /// The type of the directory page
    ///
    /// Will be equal to `ExtendibleHashTableDirectoryPage`
    type DirectoryPage;

    /// The type of the bucket page
    ///
    /// Will be equal to `ExtendibleHashTableBucketPage`
    type BucketPage;
}

impl<const BUCKET_MAX_SIZE: usize, Key, Value, KeyComparator, KeyHasherImpl> TypeAliases for DiskHashTable<BUCKET_MAX_SIZE, Key, Value, KeyComparator, KeyHasherImpl>
where
    Key: PageKey,
    Value: PageValue,
    KeyComparator: Comparator<Key>,
    KeyHasherImpl: KeyHasher
{
    type HeaderPage = HeaderPage;
    type DirectoryPage = DirectoryPage;
    type BucketPage = BucketPage<BUCKET_MAX_SIZE, Key, Value, KeyComparator>;
}
