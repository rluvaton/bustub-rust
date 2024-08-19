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
    Default(DefaultDiskManagerOptions),
    UnlimitedMemory(UnlimitedMemoryDiskManagerOptions),
}

impl DiskManagerImplementationOptions {
    pub(crate) fn get_default() -> DiskManagerImplementationOptions {
        DiskManagerImplementationOptions::Default(
            DefaultDiskManagerOptions {
                file_path: None
            }
        )
    }
    pub(crate) fn get_unlimited_memory() -> DiskManagerImplementationOptions {
        DiskManagerImplementationOptions::UnlimitedMemory(
            UnlimitedMemoryDiskManagerOptions {
                enable_latency: false
            }
        )
    }
}

impl Default for DiskManagerImplementationOptions {
    fn default() -> Self {
        Self::get_default()
    }
}

#[derive(Debug, Clone, derive_builder::Builder)]
pub(crate) struct Options {
    /// run bpm bench for n milliseconds
    #[builder(default = "2000")]
    pub(crate) duration_ms: u64,

    /// Number of scan threads
    #[builder(default = "8")]
    pub(crate) scan_thread_n: usize,

    /// Number of lookup threads
    #[builder(default = "8")]
    pub(crate) get_thread_n: usize,

    /// Buffer pool size
    #[builder(default = "64")]
    pub(crate) bpm_size: usize,

    /// Number of pages
    #[builder(default = "6400")]
    pub(crate) db_size: usize,

    /// LRU-K size
    #[builder(default = "16")]
    pub(crate) lru_k_size: usize,

    /// Disk manager that will be in use
    #[builder(default = "DiskManagerImplementationOptions::default()")]
    pub(crate) disk_manager_specific: DiskManagerImplementationOptions,
}
