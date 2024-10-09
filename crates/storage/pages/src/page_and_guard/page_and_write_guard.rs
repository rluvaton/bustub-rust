use std::ops::{Deref, DerefMut};
use crate::{Page, PageAndReadGuard, PageWriteGuard, UnderlyingPage};
use crate::page_and_guard::PageAndGuard;

pub struct PageAndWriteGuard<'a>(
    // First drop the guard and then the page
    PageWriteGuard<'a>,
    Page,
);

impl<'a> PageAndGuard for PageAndWriteGuard<'a> {
    fn page(&self) -> &Page {
        &self.1
    }
}

impl<'a> PageAndWriteGuard<'a> {

    #[inline(always)]
    pub(crate) fn page_mut(&mut self) -> &mut Page {
        &mut self.1
    }

    #[inline(always)]
    pub(crate) fn write_guard(self) -> PageWriteGuard<'a> {
        self.0
    }

    #[allow(unused)]
    pub(crate) fn downgrade(self) -> PageAndReadGuard<'a> {
        PageAndReadGuard::from(self)
    }
}

impl<'a> Deref for PageAndWriteGuard<'a> {
    type Target = UnderlyingPage;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> DerefMut for PageAndWriteGuard<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'a> From<Page> for PageAndWriteGuard<'a> {
    fn from(page: Page) -> Self {
        let write_guard = unsafe { std::mem::transmute::<PageWriteGuard<'_>, PageWriteGuard<'static>>(page.write()) };

        PageAndWriteGuard(write_guard, page)
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn should_not_leak_write_lock() {
        let page = Page::new(1);

        let page_clone = page.clone();

        {
            let page_with_guard = PageAndWriteGuard::from(page_clone);

            let page_id = page_with_guard.get_page_id();

            assert_eq!(page_id, 1);

            let page_id = page_with_guard.get_page_id();

            assert_eq!(page_id, 1);
        }

        let write_guard = page.write();

        assert_eq!(write_guard.get_page_id(), 1);
    }
}
