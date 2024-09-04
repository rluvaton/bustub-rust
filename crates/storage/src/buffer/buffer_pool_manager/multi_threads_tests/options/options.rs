use crate::buffer::buffer_pool_manager::multi_threads_tests::options::disk_manager_options::DiskManagerImplementationOptions;
use crate::buffer::buffer_pool_manager::multi_threads_tests::options::duration_type::DurationType;
use crate::buffer::buffer_pool_manager::multi_threads_tests::options::get_thread_page_id_getter::GetThreadPageId;


#[derive(Debug, Clone, derive_builder::Builder)]
pub(crate) struct Options {
    /// run bpm bench for n milliseconds
    // #[builder(default = "2000")]
    // pub(crate) duration_ms: u64,

    /// run get thread for specific duration (can be time or iteration)
    #[builder(default = "DurationType::default()")]
    pub(crate) get_thread_duration_type: DurationType,

    /// run scan threads for specific duration (can be time or iteration)
    #[builder(default = "DurationType::default()")]
    pub(crate) scan_thread_duration_type: DurationType,

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

    /// Disk manager that will be in use
    #[builder(default = "GetThreadPageId::default()")]
    pub(crate) get_thread_page_id_type: GetThreadPageId,
}
