use crate::disk::disk_manager::DiskManager;
use crate::disk::disk_scheduler::disk_request::DiskRequest;
use crate::disk::disk_scheduler::disk_scheduler::DiskScheduler;
use common::Promise;

type DiskSchedulerPromise = Promise<bool>;

impl DiskScheduler {
    pub fn new(disk_manager: &impl DiskManager) -> Self {
        unimplemented!();
    }

    /**
     * TODO(P1): Add implementation
     *
     * @brief Schedules a request for the DiskManager to execute.
     *
     * @param r The request to be scheduled.
     */
    pub fn schedule(&self, r: DiskRequest) {
        unimplemented!()
    }


    /**
     * TODO(P1): Add implementation
     *
     * @brief Background worker thread function that processes scheduled requests.
     *
     * The background thread needs to process requests while the DiskScheduler exists, i.e., this function should not
     * return until ~DiskScheduler() is called. At that point you need to make sure that the function does return.
     */
    pub fn start_worker_thread(&self) {

    }



    /**
     * @brief Create a Promise object. If you want to implement your own version of promise, you can change this function
     * so that our test cases can use your promise implementation.
     *
     * @return std::promise<bool>
     */
    pub fn create_promise(&self) -> DiskSchedulerPromise {
        Promise::new()
    }
}
