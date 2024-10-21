use crate::directory_page::DirectoryPage;
use pages::{PageId, INVALID_PAGE_ID};

#[derive(Copy, Clone)]
pub(crate) struct DirectoryIterState {
    pub(crate) index: usize,
}

/// This iterator iterate over bucket page ids
pub(crate) struct DirectoryIter<'a> {
    page: &'a DirectoryPage,
    state: DirectoryIterState,
}

impl<'a> DirectoryIter<'a> {
    pub(crate) fn new(page: &'a DirectoryPage) -> DirectoryIter<'a> {
        Self {
            page,
            state: DirectoryIterState {
                index: 0
            },
        }
    }
    
    pub(crate) fn with_state(page: &'a DirectoryPage, state: DirectoryIterState) -> DirectoryIter<'a> {
        Self {
            page,
            state
        }
    }

    pub fn get_state(&self) -> DirectoryIterState {
        self.state
    }
}

impl<'a> Iterator for DirectoryIter<'a> {
    type Item = PageId;

    fn next(&mut self) -> Option<Self::Item> {
        // Continue while there is more items and we did not reach a valid page id
        while self.state.index < self.page.size() as usize &&
            // Continue while the bucket page id is invalid or the current index is a pointer to already seen bucket page id
            (self.page.bucket_page_ids[self.state.index] == INVALID_PAGE_ID || !self.page.is_the_original_bucket_index(self.state.index as u32)) {
            
            
            self.state.index += 1;
        }

        // If reached the end
        if self.state.index >= self.page.size() as usize {
            return None;
        }

        let res = Some(self.page.bucket_page_ids[self.state.index]);
        self.state.index += 1;

        res
    }
}