use common::{PageKey, PageValue};
use crate::storage::{Comparator, ExtendibleHashTableBucketPage, ExtendibleHashTableDirectoryPage, ExtendibleHashTableHeaderPage};
use super::DiskExtendibleHashTable;

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

impl<const BUCKET_MAX_SIZE: usize, Key, Value, KeyComparator> TypeAliases for DiskExtendibleHashTable<BUCKET_MAX_SIZE, Key, Value, KeyComparator>
where
    Key: PageKey,
    Value: PageValue,
    KeyComparator: Comparator<Key>,
{
    type HeaderPage = ExtendibleHashTableHeaderPage;
    type DirectoryPage = ExtendibleHashTableDirectoryPage;
    type BucketPage = ExtendibleHashTableBucketPage<BUCKET_MAX_SIZE, Key, Value, KeyComparator>;
}
