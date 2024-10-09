use std::process::abort;
use common::config::PageData;

// This is the structure of the page
#[allow(unused)]
struct BustubBenchPageHeader {
    seed: u64,
    page_id: u64,

    // Until end of page data
    data: &'static [u8]
}

pub(crate) fn modify_page(page: &mut PageData, page_idx: usize, seed: u64) {
    page[0..8].copy_from_slice(&u64::to_ne_bytes(seed));
    page[8..16].copy_from_slice(&u64::to_ne_bytes(page_idx as u64));
    page[16 + (seed as usize % 4000)] = (seed % 256) as u8;
}


/// Check the page and verify the data inside
pub fn check_page_consistent_no_seed(data: &PageData, page_idx: usize) {
    // Cast the data pointer to a BustubBenchPageHeader pointer
    let data_seed = u64::from_ne_bytes(data[0..8].try_into().unwrap());
    let data_page_id = u64::from_ne_bytes(data[8..16].try_into().unwrap());

    if data_page_id != page_idx as u64 {
        eprintln!("Page header not consistent: page_id={} page_idx={}", data_page_id, page_idx);
        abort();
    }

    let left = data[(16 + (data_seed % 4000)) as usize];
    let right = (data_seed % 256) as u8;

    // Check if the data content is consistent
    if left != right {
        eprintln!(
            "page content not consistent: data[{}]={} seed % 256={}",
            (16 + (data_seed % 4000)),
            left,
            right
        );
        abort();
    }
}

pub fn check_page_consistent(data: &PageData, page_idx: usize, seed: u64) {
    let data_seed = u64::from_ne_bytes(data[0..8].try_into().unwrap());

    // Check if the seed matches the expected seed
    if data_seed != seed {
        eprintln!(
            "{} page seed not consistent: page.seed={} seed={}",
            page_idx, data_seed, seed
        );
        abort();
    }

    check_page_consistent_no_seed(data, page_idx);
}

#[cfg(test)]
mod test {
    use pages::Page;

    #[test]
    fn test_modify_page() {
        let page = Page::default();
        let seed = 1;
        let page_id = 2;

        page.with_write(|u| {
            super::modify_page(u.get_data_mut(), page_id, seed);
        });

        let guard = page.read();
        let data = guard.get_data();

        let actual_seed = u64::from_ne_bytes(data[0..8].try_into().unwrap());
        assert_eq!(actual_seed, seed);

        let actual_page_id = u64::from_ne_bytes(data[8..16].try_into().unwrap());
        assert_eq!(actual_page_id, page_id as u64);

        let data_array = &data[16..];
        let actual_data = data_array[seed as usize % 4000];
        assert_eq!(actual_data, seed as u8);
    }
}

