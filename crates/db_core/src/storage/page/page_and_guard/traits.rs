use crate::storage::{Page, UnderlyingPage};
use std::ops::Deref;

pub(crate) trait PageAndGuard: Deref<Target = UnderlyingPage> + From<Page> {

    fn page(self) -> Page;

    fn page_ref(&self) -> &Page;
}

