use pages::{PageId, INVALID_PAGE_ID};
use crate::header_page::HeaderPage;

#[derive(Copy, Clone)]
pub(crate) struct HeaderIterState {
    pub(crate) index: usize,
}

/// This iterator iterate over directory page ids
pub(crate) struct HeaderIter<'a> {
    page: &'a HeaderPage,
    state: HeaderIterState
}

impl<'a> HeaderIter<'a> {
    pub(crate) fn new(page: &'a HeaderPage) -> HeaderIter<'a> {
        Self {
            page,
            state: HeaderIterState {
                index: 0,
            },
        }
    }

    pub(crate) fn with_state(page: &'a HeaderPage, state: HeaderIterState) -> HeaderIter<'a> {
        Self {
            page,
            state,
        }
    }

    pub fn get_state(&self) -> HeaderIterState {
        self.state
    }
}

impl<'a> Iterator for HeaderIter<'a> {
    type Item = PageId;

    fn next(&mut self) -> Option<Self::Item> {
        let dir = self.page.directory_page_ids;
        
        // Continue while there is more items and we did not reach a valid page id
        while self.state.index < dir.len() && dir[self.state.index] == INVALID_PAGE_ID {
            self.state.index += 1;
        }

        // If reached the end
        if self.state.index >= dir.len() {
            return None;
        }
        
        let res = Some(dir[self.state.index]);
        self.state.index += 1;
        
        res
    }
}
