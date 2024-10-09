use crate::{Page, UnderlyingPage};
use std::ops::Deref;

pub trait PageAndGuard: Deref<Target = UnderlyingPage> + From<Page> {


    fn page(&self) -> &Page;
}

