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
