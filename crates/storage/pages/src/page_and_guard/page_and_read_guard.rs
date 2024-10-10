use std::ops::Deref;
use crate::{Page, PageReadGuard, PageWriteGuard, UnderlyingPage, PageAndWriteGuard, PageAndGuard};

pub struct PageAndReadGuard<'a>(
    // First drop the guard and then the page
    PageReadGuard<'a>,
    Page,
);

impl<'a> PageAndGuard<'a> for PageAndReadGuard<'a> {
    fn page(&self) -> &Page {
        &self.1
    }
}

impl<'a> PageAndReadGuard<'a> {
    #[inline(always)]
    #[allow(unused)]
    pub(crate) fn page_mut_ref(&mut self) -> &mut Page {
        &mut self.1
    }

    #[inline(always)]
    #[allow(unused)]
    pub(crate) fn read_guard(self) -> PageReadGuard<'a> {
        self.0
    }
}

impl<'a> Deref for PageAndReadGuard<'a> {
    type Target = UnderlyingPage;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> From<Page> for PageAndReadGuard<'a> {
    fn from(page: Page) -> Self {
        let read_guard = unsafe { std::mem::transmute::<PageReadGuard<'_>, PageReadGuard<'static>>(page.read()) };

        PageAndReadGuard(read_guard, page)
    }
}

impl<'a> From<PageAndWriteGuard<'a>> for PageAndReadGuard<'a> {
    fn from(page_and_guard: PageAndWriteGuard<'a>) -> Self {
        let page = page_and_guard.page().clone();
        let read_guard = unsafe { std::mem::transmute::<PageReadGuard<'_>, PageReadGuard<'static>>(PageWriteGuard::downgrade(page_and_guard.write_guard())) };

        PageAndReadGuard(read_guard, page)
    }
}
