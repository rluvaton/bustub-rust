use pages::{PageId, INVALID_PAGE_ID};
use crate::header_page::HeaderPage;

/// This iterator iterate over directory page ids
pub(crate) struct HeaderIter<'a> {
    page: &'a HeaderPage,
    index: usize
}

impl<'a> HeaderIter<'a> {
    pub(crate) fn new(page: &'a HeaderPage) -> HeaderIter<'a> {
        Self {
            page,
            index: 0,
        }
    }
}

impl<'a> Iterator for HeaderIter<'a> {
    type Item = PageId;

    fn next(&mut self) -> Option<Self::Item> {
        let dir = self.page.directory_page_ids;
        
        // Continue while there is more items and we did not reach a valid page id
        while self.index < dir.len() && dir[self.index] == INVALID_PAGE_ID {
            self.index += 1;
        }

        // If reached the end
        if self.index >= dir.len() {
            return None;
        }
        
        let res = Some(dir[self.index]);
        self.index += 1;
        
        res
    }
}