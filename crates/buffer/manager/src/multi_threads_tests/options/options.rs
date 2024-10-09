use super::disk_manager_options::DiskManagerImplementationOptions;
use super::duration_type::DurationType;
use super::get_thread_page_id_getter::GetThreadPageId;


#[derive(Debug, Clone, derive_builder::Builder)]
pub(in super::super) struct Options {
    /// run bpm bench for n milliseconds
    // #[builder(default = "2000")]
    // pub(crate) duration_ms: u64,

    /// run get thread for specific duration (can be time or iteration)
    #[builder(default = "DurationType::default()")]
    pub(in super::super) get_thread_duration_type: DurationType,

    /// run scan threads for specific duration (can be time or iteration)
    #[builder(default = "DurationType::default()")]
    pub(in super::super) scan_thread_duration_type: DurationType,

    /// Number of scan threads
    #[builder(default = "8")]
    pub(in super::super) scan_thread_n: usize,

    /// Number of lookup threads
    #[builder(default = "8")]
    pub(in super::super) get_thread_n: usize,

    /// Buffer pool size
    #[builder(default = "64")]
    pub(in super::super) bpm_size: usize,

    /// Number of pages
    #[builder(default = "6400")]
    pub(in super::super) db_size: usize,

    /// LRU-K size
    #[builder(default = "16")]
    pub(in super::super) lru_k_size: usize,

    /// Disk manager that will be in use
    #[builder(default = "DiskManagerImplementationOptions::default()")]
    pub(in super::super) disk_manager_specific: DiskManagerImplementationOptions,

    /// Disk manager that will be in use
    #[builder(default = "GetThreadPageId::default()")]
    pub(in super::super) get_thread_page_id_type: GetThreadPageId,
}
