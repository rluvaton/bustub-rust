use super::type_alias_trait::TypeAliases;

use crate::DiskHashTable;
use buffer_pool_manager::errors::BufferPoolError;
use buffer_pool_manager::PageReadGuard;
use common::{Comparator, PageKey, PageValue};
use hashing_common::KeyHasher;
use pages::PageId;

pub const HEADER_PAGE_ID: PageId = 0;                                             // the header page id


impl<const BUCKET_MAX_SIZE: usize, Key, Value, KeyComparator, KeyHasherImpl> DiskHashTable<BUCKET_MAX_SIZE, Key, Value, KeyComparator, KeyHasherImpl>
where
    Key: PageKey,
    Value: PageValue,
    KeyComparator: Comparator<Key>,
    KeyHasherImpl: KeyHasher,
{
    fn fetch_header_page_read<'a>(&'a self, header: &'a PageReadGuard<'a>) -> Result<(&'a PageReadGuard, &'a <Self as TypeAliases>::HeaderPage), BufferPoolError> {
        // let header = self.bpm.fetch_page_read(self.header_page_id, AccessType::Unknown)
        //     .map_err_to_buffer_pool_err()
        //     .context("Failed to fetch header")?;

        let header_page = header.cast::<<Self as TypeAliases>::HeaderPage>();

        Ok((header, header_page))
    }
}


