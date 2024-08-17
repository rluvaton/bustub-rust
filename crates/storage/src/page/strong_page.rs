use crate::page::underlying_page::UnderlyingPage;
use common::config::{PageData, PageId, BUSTUB_PAGE_SIZE, INVALID_PAGE_ID};
use common::ReaderWriterLatch;
use std::sync::{Arc, Weak};
use anyhow::anyhow;
use crate::WeakPage;
// static_assert(sizeof(page_id_t) == 4);
// static_assert(sizeof(lsn_t) == 4);


/**
 * Page is the basic unit of storage within the database system. Page provides a wrapper for actual data pages being
 * held in main memory. Page also contains book-keeping information that is used by the buffer pool manager, e.g.
 * pin count, dirty flag, page id, etc.
 */
#[derive(Debug)]
pub struct StrongPage {
    // Globally accessible
    pub underlying_page: Arc<ReaderWriterLatch<UnderlyingPage>>,
}

impl StrongPage {
    pub fn new(page_id: PageId) -> Self {
        StrongPage {
            underlying_page: Arc::new(ReaderWriterLatch::new(UnderlyingPage::new(page_id, [0u8; BUSTUB_PAGE_SIZE])))
        }
    }

    pub fn create_with_data(page_id: PageId, data: PageData) -> Self {
        StrongPage {
            underlying_page: Arc::new(ReaderWriterLatch::new(UnderlyingPage::new(page_id, data)))
        }
    }

    pub fn get_page_id(&self) -> PageId {
        self.underlying_page.read().get_page_id()
    }

    /** @return the pin count of this page */
    pub fn get_pin_count(&self) -> usize {
        Arc::strong_count(&self.underlying_page)
    }

    pub fn mark_as_dirty(&self) {
        let mut underlying = self.underlying_page.write();

        underlying.set_is_dirty(true);
    }
}

impl Default for StrongPage {
    fn default() -> Self {
        StrongPage::new(INVALID_PAGE_ID)
    }
}

impl Clone for StrongPage {
    fn clone(&self) -> Self {
        StrongPage {
            underlying_page: self.underlying_page.clone()
        }
    }
}

impl PartialEq for StrongPage {
    fn eq(&self, other: &Self) -> bool {
        let self_underlying = self.underlying_page.read();
        let other_underlying = other.underlying_page.read();

        self_underlying.eq(&other_underlying)
    }
}

impl PartialEq<WeakPage> for StrongPage {
    fn eq(&self, other: &WeakPage) -> bool {
        let self_underlying = self.underlying_page.read();

        let other_strong: Result<StrongPage, anyhow::Error> = other.try_into();

        if other_strong.is_err() {
            return false
        }

        let other_strong = other_strong.unwrap();

        let other_underlying = other_strong.underlying_page.read();

        self_underlying.eq(&other_underlying)
    }
}

impl TryFrom<WeakPage> for StrongPage {
    type Error = anyhow::Error;

    fn try_from(value: WeakPage) -> Result<Self, Self::Error> {
        let upgraded = Weak::upgrade(&value.underlying_page);

        if let Some(under) = upgraded {
            return Ok(
                StrongPage {
                    underlying_page: under
                }
            );
        }

        Err(anyhow!("Unable to upgrade"))
    }
}

impl TryFrom<&WeakPage> for StrongPage {
    type Error = anyhow::Error;

    fn try_from(value: &WeakPage) -> Result<Self, Self::Error> {
        let upgraded = Weak::upgrade(&value.underlying_page);

        if let Some(under) = upgraded {
            return Ok(
                StrongPage {
                    underlying_page: under
                }
            );
        }

        Err(anyhow!("Unable to upgrade"))
    }
}
