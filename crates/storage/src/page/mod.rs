mod strong_page;
mod page_guards;
mod underlying_page;
mod weak_page;

use common::config::{PageData, BUSTUB_PAGE_SIZE};
pub use strong_page::{StrongPage as Page};
pub use weak_page::WeakPage;
pub use underlying_page::UnderlyingPage;
pub use page_guards::{BasicPageGuard, ReadPageGuard, WritePageGuard};

pub trait AlignToPageData {
    fn align_to_page_data(&self) -> PageData;
}

impl AlignToPageData for &[u8] {
    fn align_to_page_data(&self) -> PageData {
        let mut data = [0; BUSTUB_PAGE_SIZE];

        data[..self.len()].copy_from_slice(self);

        data
    }
}
