use std::path::PathBuf;

#[derive(Debug, Clone)]
pub(in super::super) struct DefaultDiskManagerOptions {
    /// reuse db file
    pub file_path: Option<PathBuf>,
}

#[derive(Debug, Clone)]
pub(in super::super) struct UnlimitedMemoryDiskManagerOptions {
    /// enable disk latency
    pub(in super::super) enable_latency: bool,
}

#[derive(Debug, Clone)]
pub(in super::super) enum DiskManagerImplementationOptions {
    Default(DefaultDiskManagerOptions),
    UnlimitedMemory(UnlimitedMemoryDiskManagerOptions),
}

impl DiskManagerImplementationOptions {
    pub(in super::super) fn get_default() -> DiskManagerImplementationOptions {
        DiskManagerImplementationOptions::Default(
            DefaultDiskManagerOptions {
                file_path: None
            }
        )
    }
    pub(in super::super) fn get_unlimited_memory() -> DiskManagerImplementationOptions {
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
