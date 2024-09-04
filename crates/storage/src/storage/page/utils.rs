use common::config::{PageData, BUSTUB_PAGE_SIZE};

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
