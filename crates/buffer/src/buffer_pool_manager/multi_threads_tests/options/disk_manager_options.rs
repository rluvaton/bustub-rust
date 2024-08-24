use std::path::PathBuf;

#[derive(Debug, Clone)]
pub(crate) struct DefaultDiskManagerOptions {
    /// reuse db file
    pub file_path: Option<PathBuf>,
}

#[derive(Debug, Clone)]
pub(crate) struct UnlimitedMemoryDiskManagerOptions {
    /// enable disk latency
    pub(crate) enable_latency: bool,
}

#[derive(Debug, Clone)]
pub(crate) enum DiskManagerImplementationOptions {
    Default(crate::buffer_pool_manager::multi_threads_tests::options::DefaultDiskManagerOptions),
    UnlimitedMemory(crate::buffer_pool_manager::multi_threads_tests::options::UnlimitedMemoryDiskManagerOptions),
}

impl crate::buffer_pool_manager::multi_threads_tests::options::DiskManagerImplementationOptions {
    pub(crate) fn get_default() -> crate::buffer_pool_manager::multi_threads_tests::options::DiskManagerImplementationOptions {
        crate::buffer_pool_manager::multi_threads_tests::options::DiskManagerImplementationOptions::Default(
            crate::buffer_pool_manager::multi_threads_tests::options::DefaultDiskManagerOptions {
                file_path: None
            }
        )
    }
    pub(crate) fn get_unlimited_memory() -> crate::buffer_pool_manager::multi_threads_tests::options::DiskManagerImplementationOptions {
        crate::buffer_pool_manager::multi_threads_tests::options::DiskManagerImplementationOptions::UnlimitedMemory(
            crate::buffer_pool_manager::multi_threads_tests::options::UnlimitedMemoryDiskManagerOptions {
                enable_latency: false
            }
        )
    }
}

impl Default for crate::buffer_pool_manager::multi_threads_tests::options::DiskManagerImplementationOptions {
    fn default() -> Self {
        Self::get_default()
    }
}
