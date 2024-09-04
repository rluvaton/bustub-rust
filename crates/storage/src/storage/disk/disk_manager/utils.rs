use std::os::unix::fs::MetadataExt;
use std::path::Path;

// TODO - should return option instead
pub(crate) fn get_file_size(file_name: &Path) -> i32 {
    let m = file_name.metadata();

    if m.is_err() {
        return -1;
    }

    let m = m.unwrap();

    m.size() as i32
}
