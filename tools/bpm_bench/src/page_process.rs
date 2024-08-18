use std::process::abort;
use common::config::PageData;

// #[repr(C)]
struct BustubBenchPageHeader {
    seed: u64,
    page_id: u64,
    // No `data` here, we will handle it with raw pointers.
}

pub(crate) unsafe fn modify_page(page: &mut PageData, page_idx: usize, seed: u64) {
    // Cast the data pointer to a BustubBenchPageHeader pointer
    let pg = page.as_mut_ptr() as *mut BustubBenchPageHeader;

    // Modify the header fields
    (*pg).seed = seed;
    (*pg).page_id = page_idx as u64;

    // Access the flexible data array
    let data_array = page.as_mut_ptr().add(std::mem::size_of::<BustubBenchPageHeader>());
    *data_array.add((*pg).seed as usize % 4000) = ((*pg).seed % 256) as u8;
}


/// Check the page and verify the data inside
pub unsafe fn check_page_consistent_no_seed(data: &PageData, page_idx: usize) {
    // Cast the data pointer to a BustubBenchPageHeader pointer
    let pg = data.as_ptr() as *mut BustubBenchPageHeader;

    if (*pg).page_id != page_idx as u64 {
        eprintln!("Page header not consistent: page_id={} page_idx={}", (*pg).page_id, page_idx);
        abort();
    }


    let left = *data.as_ptr().add(std::mem::size_of::<BustubBenchPageHeader>() + ((*pg).seed % 4000) as usize) as u8;
    let right = ((*pg).seed % 256) as u8;

    // Check if the data content is consistent
    if left != right {
        eprintln!(
            "page content not consistent: data_[{}]={} seed_ % 256={}",
            (*pg).seed % 4000,
            left,
            right
        );
        abort();
    }
}

pub unsafe fn check_page_consistent(data: &PageData, page_idx: usize, seed: u64) {
    let pg = data.as_ptr() as *const BustubBenchPageHeader;

    // Check if the seed matches the expected seed
    if (*pg).seed != seed {
        eprintln!(
            "page seed not consistent: page.seed={} seed={}",
            (*pg).seed, seed
        );
        abort();
    }

    check_page_consistent_no_seed(data, page_idx);
}

//
// /// Check the page and verify the data inside
// auto CheckPageConsistent(const char *data, size_t page_idx, uint64_t seed) -> void {
// const auto *pg = reinterpret_cast<const BustubBenchPageHeader *>(data);
// if (pg->seed_ != seed) {
// fmt::println(stderr, "page seed not consistent: seed_={} seed={}", pg->seed_, seed);
// std::terminate();
// }
// CheckPageConsistentNoSeed(data, page_idx);
// }


#[cfg(test)]
mod test {
    #[test]
    fn test_modify_page() {
        let page = storage::Page::default();
        let seed = 1;
        let page_id = 2;

        page.with_write(|u| unsafe {
            super::modify_page(u.get_data_mut(), page_id, seed);
        });

        let data = page.with_read(|u| *u.get_data());

        let actual_seed = u64::from_ne_bytes(data[0..8].try_into().unwrap());
        assert_eq!(actual_seed, seed);

        let actual_page_id = u64::from_ne_bytes(data[8..16].try_into().unwrap());
        assert_eq!(actual_page_id, page_id as u64);

        let data_array = &data[16..];
        let actual_data = data_array[seed as usize % 4000];
        assert_eq!(actual_data, seed as u8);
    }
}


// /// Modify the page and save some data inside
// auto ModifyPage(char *data, size_t page_idx, uint64_t seed) -> void {
// auto *pg = reinterpret_cast<BustubBenchPageHeader *>(data);
// pg->seed_ = seed;
// pg->page_id_ = page_idx;
// pg->data_[pg->seed_ % 4000] = pg->seed_ % 256;
// }
