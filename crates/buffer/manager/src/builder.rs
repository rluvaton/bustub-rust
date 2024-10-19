use crate::manager::InnerBufferPoolManager;
use crate::{BufferPoolManager};
use disk_storage::{DiskManager, DiskScheduler};
use eviction_policy::{EvictionPoliciesTypes, EvictionPolicy, EvictionPolicyCreator, LRUKEvictionPolicy, LRUKOptions};
use pages::AtomicPageId;
use parking_lot::Mutex;
use recovery_log_manager::LogManager;
use std::collections::{HashMap, LinkedList};
use std::sync::Arc;


#[cfg(feature = "statistics")]
use crate::BufferPoolManagerStats;

pub struct BufferPoolManagerBuilder {
    /// This is required
    pool_size: Option<usize>,

    /// This is required
    disk_scheduler: Option<DiskScheduler>,

    // Eviction policy creator, it get the pool size and return the eviction policy
    eviction_policy_creator: Box<dyn FnOnce(usize) -> Box<dyn EvictionPolicy>>,

    /// This is not required
    log_manager: Option<Arc<LogManager>>,
}

impl BufferPoolManagerBuilder {
    pub fn with_pool_size(mut self, pool_size: usize) -> Self {
        assert!(self.pool_size.is_none(), "Pool size is already set");

        self.pool_size.replace(pool_size);

        self
    }

    // ################# Disk Scheduler #####################

    pub fn with_disk_manager<D: DiskManager + 'static>(self, disk_manager: D) -> Self {
        self.with_arc_disk_manager(Arc::new(disk_manager))
    }

    pub fn with_arc_disk_manager<D: DiskManager>(mut self, disk_manager: Arc<D>) -> Self {
        assert!(self.disk_scheduler.is_none(), "Disk scheduler is already set");

        self.disk_scheduler.replace(DiskScheduler::new(disk_manager));

        self
    }

    pub fn with_disk_scheduler(mut self, disk_scheduler: DiskScheduler) -> Self {
        assert!(self.disk_scheduler.is_none(), "Disk scheduler is already set");

        self.disk_scheduler.replace(disk_scheduler);

        self
    }

    // ################# Eviction Policies #####################

    pub fn with_lru_k_eviction_policy(self, k: usize) -> Self {
        self.with_eviction_policy_creator(EvictionPoliciesTypes::LRU_K(LRUKOptions::new(k)).get_creator())
    }

    pub fn with_eviction_policy_creator<Creator: FnOnce(usize) -> Box<dyn EvictionPolicy> + 'static>(mut self, creator: Creator) -> Self {
        self.eviction_policy_creator = Box::new(creator);

        self
    }

    pub fn with_log_manager(mut self, log_manager: Option<Arc<LogManager>>) -> Self {
        self.log_manager = log_manager;

        self
    }

    pub fn build(self) -> BufferPoolManager {
        let pool_size = self.pool_size.expect("Must have pool size");
        // Initially, every page is in the free list.
        let mut free_list = LinkedList::new();

        for i in 0..pool_size {
            free_list.push_back(i as i32)
        }

        BufferPoolManager {
            next_page_id: AtomicPageId::new(0),
            pool_size,

            log_manager: self.log_manager,

            inner: Mutex::new(InnerBufferPoolManager {

                // we allocate a consecutive memory space for the buffer pool
                pages: Vec::with_capacity(pool_size),

                eviction_policy: (self.eviction_policy_creator)(pool_size),

                page_table: HashMap::with_capacity(pool_size),
                free_list,

                disk_scheduler: Arc::new(self.disk_scheduler.expect("Must have disk scheduler")),
            }),

            pending_fetch_requests: Mutex::new(HashMap::new()),

            #[cfg(feature = "statistics")]
            stats: BufferPoolManagerStats::default(),
        }
    }

    pub fn build_arc(self) -> Arc<BufferPoolManager> {
        Arc::new(self.build())
    }
}

impl Default for BufferPoolManagerBuilder {
    fn default() -> Self {
        Self {
            pool_size: None,
            disk_scheduler: None,
            eviction_policy_creator: Box::new(|number_of_frames: usize| Box::new(LRUKEvictionPolicy::new(number_of_frames, LRUKOptions::default()))),
            log_manager: None,
        }
    }
}
