use std::ops::{Deref, DerefMut};
use crate::storage::{Page, PageAndReadGuard, PageWriteGuard};

pub(crate) struct PageAndWriteGuard<'a>(
    // First drop the guard and then the page
    PageWriteGuard<'a>,
    Page,
);

impl<'a> PageAndWriteGuard<'a> {
    #[inline(always)]
    pub(crate) fn page(self) -> Page {
        self.1
    }

    #[inline(always)]
    pub(crate) fn page_ref(&self) -> &Page {
        &self.1
    }

    #[inline(always)]
    pub(crate) fn page_mut_ref(&mut self) -> &mut Page {
        &mut self.1
    }

    #[inline(always)]
    pub(crate) fn write_guard(self) -> PageWriteGuard<'a> {
        self.0
    }

    pub(crate) fn downgrade(self) -> PageAndReadGuard<'a> {
        PageAndReadGuard::from(self)
    }
}

impl<'a> Deref for PageAndWriteGuard<'a> {
    type Target = PageWriteGuard<'a>;

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

impl<'a> From<&mut Page> for PageAndWriteGuard<'a> {
    fn from(page: &mut Page) -> Self {
        let write_guard = unsafe { std::mem::transmute::<PageWriteGuard<'_>, PageWriteGuard<'static>>(page.write()) };

        PageAndWriteGuard(write_guard, page.clone())
    }
}

#[cfg(test)]
mod tests {
    use crate::storage::Page;
    use super::*;

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
