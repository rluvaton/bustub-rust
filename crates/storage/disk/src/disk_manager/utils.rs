use std::path::Path;

// TODO - should return option instead
pub(super) fn get_file_size(file_name: &Path) -> i32 {
    let m = file_name.metadata();

    if m.is_err() {
        return -1;
    }

    let m = m.unwrap();


    m.len() as i32
}
