use crate::{Page, PageAndWriteGuard, UnderlyingPage};
use std::ops::Deref;

pub trait PageAndGuard<'a>: Deref<Target = UnderlyingPage> + From<Page> + From<PageAndWriteGuard<'a>> {


    fn page(&self) -> &Page;
}

