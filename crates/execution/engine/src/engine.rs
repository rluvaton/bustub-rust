use std::sync::Arc;
use parking_lot::Mutex;
use buffer_pool_manager::BufferPoolManager;
use db_core::catalog::Catalog;
use db_core::concurrency::TransactionManager;
use planner::PlanType;
use transaction::Transaction;
use tuple::Tuple;
use crate::context::ExecutorContext;
use crate::executors::CreateExecutor;

/// The ExecutionEngine class executes query plans.
pub struct ExecutionEngine {
    bpm: Arc<BufferPoolManager>,
    txn_manager: Arc<TransactionManager>,
    catalog: Arc<Mutex<Catalog>>,
}

impl ExecutionEngine {
    pub fn new(bpm: Arc<BufferPoolManager>,
               txn_manager: Arc<TransactionManager>,
               catalog: Arc<Mutex<Catalog>>) -> Self {
        Self {
            bpm,
            txn_manager,
            catalog
        }
    }


    /**
     * Execute a query plan.
     * @param plan The query plan to execute
     * @param result_set The set of tuples produced by executing the plan
     * @param txn The transaction context in which the query executes
     * @param exec_ctx The executor context in which the query executes
     * @return `true` if execution of the query plan succeeds, `false` otherwise

     */
    pub fn execute(&self, plan: PlanType, txn: Arc<Transaction>, exec_ctx: Arc<ExecutorContext>) -> error_utils::anyhow::Result<Vec<Tuple>> {
        assert_eq!(txn.get_transaction_id(), exec_ctx.get_transaction().get_transaction_id(), "Broken Invariant");

        // Construct the executor for the abstract plan node
        let ex = plan.create_executor(exec_ctx);

        let res = ex
            .map(|item| item.0)
            .collect::<Vec<Tuple>>();

        // TODO - perform checks

        Ok(res)
    }
}
