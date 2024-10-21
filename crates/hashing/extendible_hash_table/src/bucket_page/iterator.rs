use crate::bucket_page::{BucketPage, MappingType};
use common::{Comparator, PageKey, PageValue};

pub(crate) struct BucketIter<'a, const ARRAY_SIZE: usize, Key, Value, KeyComparator>
where
    Key: PageKey,
    Value: PageValue,
    KeyComparator: Comparator<Key>,
{
    page: &'a BucketPage<ARRAY_SIZE, Key, Value, KeyComparator>,
    index: usize,
}

impl<'a, const ARRAY_SIZE: usize, Key, Value, KeyComparator> BucketIter<'a, ARRAY_SIZE, Key, Value, KeyComparator>
where
    Key: PageKey,
    Value: PageValue,
    KeyComparator: Comparator<Key>,
{
    pub(crate) fn new(page: &'a BucketPage<ARRAY_SIZE, Key, Value, KeyComparator>) -> Self {
        Self {
            page,
            index: 0,
        }
    }
}

impl<'a, const ARRAY_SIZE: usize, Key, Value, KeyComparator> Iterator for BucketIter<'a, ARRAY_SIZE, Key, Value, KeyComparator>
where
    Key: PageKey,
    Value: PageValue,
    KeyComparator: Comparator<Key>,
{
    type Item = &'a MappingType<Key, Value>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.page.size() as usize {
            return None;
        }

        let res = Some(&self.page.array[self.index]);
        self.index += 1;

        res
    }
}

#[cfg(test)]
mod tests {
    use crate::bucket_page::test_utils::insert_until_full;
use std::sync::Arc;
    use buffer_common::AccessType;
    use buffer_pool_manager::{BufferPool, BufferPoolManager};
    use common::OrdComparator;
    use crate::{bucket_array_size, bucket_page_type};
    use crate::bucket_page::BucketPage;

    #[test]
    fn should_go_over_all_entries() {
        type Key = u64;
        type Value = u64;
        
        let bpm = Arc::new(BufferPoolManager::default());
        let mut new_page = bpm.new_page(AccessType::Unknown).expect("Create new page");
        let bucket_page = new_page.cast_mut::<bucket_page_type!(Key, Value, OrdComparator<Key>)>();
        
        let mut entries = insert_until_full(bucket_page);
        let mut found_entries = bucket_page.iter().cloned().collect::<Vec<(Key, Value)>>();
        
        // Sort both entries so we can compare
        entries.sort();
        found_entries.sort();
        
        assert_eq!(entries, found_entries);
    }
}
