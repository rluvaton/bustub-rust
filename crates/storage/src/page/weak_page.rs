use crate::page::underlying_page::UnderlyingPage;
use common::config::{PageData, PageId, BUSTUB_PAGE_SIZE, INVALID_PAGE_ID};
use common::ReaderWriterLatch;
use std::sync::{Arc, Weak};
use crate::page::strong_page::StrongPage;
// static_assert(sizeof(page_id_t) == 4);
// static_assert(sizeof(lsn_t) == 4);


#[derive(Debug)]
pub struct WeakPage {
    // Globally accessible
    pub underlying_page: Weak<ReaderWriterLatch<UnderlyingPage>>,
}

impl WeakPage {
    pub fn get_page_id(&self) -> Option<PageId> {
        Some(self.underlying_page.upgrade()?.read().get_page_id())
    }

    /** @return the pin count of this page */
    pub fn get_pin_count(&self) -> usize {
        Weak::strong_count(&self.underlying_page)
    }

    pub fn mark_as_dirty(&self) -> Option<()> {
        let underlying = self.underlying_page.upgrade()?;
        let mut underlying = underlying.write();

        underlying.set_is_dirty(true);

        Some(())
    }
}

impl Clone for WeakPage {
    fn clone(&self) -> Self {
        WeakPage {
            underlying_page: self.underlying_page.clone()
        }
    }
}

impl PartialEq for WeakPage {
    fn eq(&self, other: &Self) -> bool {
        let self_strong = self.underlying_page.upgrade();
        let other_strong = other.underlying_page.upgrade();

        if self_strong.is_some() && other_strong.is_some() {
            let self_strong = self_strong.unwrap();
            let other_strong = other_strong.unwrap();
            let self_underlying = self_strong.read();
            let other_underlying = other_strong.read();

            return self_underlying.eq(&other_underlying)
        }

        self_strong.is_none() == other_strong.is_none()
    }
}

impl From<StrongPage> for WeakPage {
    fn from(value: StrongPage) -> Self {
        WeakPage {
            underlying_page: Arc::downgrade(&Arc::clone(&value.underlying_page))
        }
    }
}

impl From<&StrongPage> for WeakPage {
    fn from(value: &StrongPage) -> Self {
        WeakPage {
            underlying_page: Arc::downgrade(&Arc::clone(&value.underlying_page))
        }
    }
}
