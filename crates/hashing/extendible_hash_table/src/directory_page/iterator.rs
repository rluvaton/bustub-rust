use buffer_pool_manager::PageReadGuard;
use pages::{PageId, INVALID_PAGE_ID};
use crate::directory_page::DirectoryPage;

/// This iterator iterate over bucket page ids
pub(crate) struct DirectoryIter<'a> {
    guard: PageReadGuard<'a>,
    index: usize,
}

impl<'a> DirectoryIter<'a> {
    pub(crate) fn new(guard: PageReadGuard<'a>) -> DirectoryIter<'a> {
        Self {
            guard,
            index: 0,
        }
    }
}

impl<'a> Iterator for DirectoryIter<'a> {
    type Item = PageId;

    fn next(&mut self) -> Option<Self::Item> {
        let dir = self.guard.cast::<DirectoryPage>();

        // Continue while there is more items and we did not reach a valid page id
        while self.index < dir.size() as usize &&
            // Continue while the bucket page id is invalid or the current index is a pointer to already seen bucket page id
            (dir.bucket_page_ids[self.index] == INVALID_PAGE_ID || !dir.is_the_original_bucket_index(self.index as u32)) {
            
            
            self.index += 1;
        }

        // If reached the end
        if self.index >= dir.size() as usize {
            return None;
        }

        let res = Some(dir.bucket_page_ids[self.index]);
        self.index += 1;

        res
    }
}