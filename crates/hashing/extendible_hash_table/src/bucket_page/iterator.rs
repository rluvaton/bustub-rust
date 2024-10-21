use crate::bucket_page::BucketPage;
use buffer_pool_manager::PageReadGuard;
use common::{Comparator, PageKey, PageValue};
use pages::{PageId, INVALID_PAGE_ID};
use std::marker::PhantomData;

pub(crate) struct BucketIter<'a, const ARRAY_SIZE: usize, Key, Value, KeyComparator>
where
Key: PageKey,
Value: PageValue,
KeyComparator: Comparator<Key> {
    guard: PageReadGuard<'a>,
    index: usize,
    
    phantom_data: PhantomData<(KeyComparator, Value, KeyComparator)>
}

impl<'a, const ARRAY_SIZE: usize, Key, Value, KeyComparator> BucketIter<'a, ARRAY_SIZE, Key, Value, KeyComparator> where
    Key: PageKey,
    Value: PageValue,
    KeyComparator: Comparator<Key> {
    pub(crate) fn new(guard: PageReadGuard<'a>) ->Self {
        Self {
            guard,
            index: 0,
            phantom_data: PhantomData::default()
        }
    }
}

impl<'a, const ARRAY_SIZE: usize, Key, Value, KeyComparator> Iterator for BucketIter<'a, ARRAY_SIZE, Key, Value, KeyComparator>
where
    Key: PageKey,
    Value: PageValue,
    KeyComparator: Comparator<Key>{
    type Item = PageId;

    fn next(&mut self) -> Option<Self::Item> {
        let bucket = self.guard.cast::<BucketPage<ARRAY_SIZE, Key, Value, KeyComparator>>();

        // Continue while there is more items and we did not reach a valid page id
        while self.index < bucket.size() as usize &&
            // Continue while the bucket page id is invalid or the current index is a pointer to already seen bucket page id
            (bucket.bucket_page_ids[self.index] == INVALID_PAGE_ID || !bucket.is_the_original_bucket_index(self.index as u32)) {
            
            
            self.index += 1;
        }

        // If reached the end
        if self.index >= bucket.size() as usize {
            return None;
        }

        let res = Some(bucket[self.index]);
        self.index += 1;

        res
    }
}

#[cfg(test)]
mod tests {
    use crate::bucket_array_size;
    use crate::bucket_page::{test_utils, BucketPage};
    use buffer_common::AccessType;
    use buffer_pool_manager::{BufferPool, BufferPoolManager};
    use common::OrdComparator;

    // #[test]
    // fn should_go_over_all_items() {
    //     type Key = u64;
    //     type Value = u64;
    //     type Entry = (Key, Value);
    //     
    //     let bpm = BufferPoolManager::builder().build_arc();
    //     let mut new_page = bpm.new_page(AccessType::Unknown).expect("Create new page");
    //     let bucket_page = new_page.cast_mut::<BucketPage<{bucket_array_size::<Key, Value>()}, Key, Value, OrdComparator<Key>>>();
    //     
    //     let entries = test_utils::insert_until_full(bucket_page);
    //     
    //     bucket_page.insert()
    //     
    //     
    // }
}
