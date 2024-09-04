use tempdir::TempDir;

pub(crate) fn get_tmp_dir() -> TempDir {
    TempDir::new("buffer_pool_manager_multi_threads_tests").expect("Should create tmp directory")
}
