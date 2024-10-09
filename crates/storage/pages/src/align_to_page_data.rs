use crate::{PageData, PAGE_SIZE};

pub trait AlignToPageData {
    fn align_to_page_data(&self) -> PageData;
}

impl AlignToPageData for &[u8] {
    fn align_to_page_data(&self) -> PageData {
        let mut data = [0; PAGE_SIZE];

        data[..self.len()].copy_from_slice(self);

        data
    }
}

impl AlignToPageData for &str {
    fn align_to_page_data(&self) -> PageData {
        self.as_bytes().align_to_page_data()
    }
}
