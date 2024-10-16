use std::rc::Rc;
use db_core::catalog::Catalog;
use std::sync::atomic::{AtomicUsize, Ordering};
use parking_lot::Mutex;
use binder::StatementTypeImpl;
use crate::plan_nodes::{PlanNodeRef, PlanType};
use crate::planner::{Context, ContextGuard};
use crate::traits::Plan;

pub struct Planner<'a> {
    /// Catalog will be used during the binding process
    pub(crate) catalog: &'a Catalog,

    // TODO - replace with Cow? as this is single threaded and can't have 2 places changing in the same time
    pub(crate) context: Mutex<Context>,

    /** Sometimes we will need to assign a name to some unnamed items. This variable gives them a universal ID. */
    pub(crate) universal_id: AtomicUsize,
}

impl<'a> Planner<'a> {
    pub fn new(catalog: &'a Catalog) -> Self {
        Self {
            catalog,
            context: Mutex::new(Context::default()),
            universal_id: AtomicUsize::new(0),
        }
    }


    /** If any function needs to modify the scope, it MUST hold the context guard, so that
      * the context will be recovered after the function returns. Currently, it's used in
      * `BindFrom` and `BindJoin`.
     */
    pub(crate) fn new_context(&'a self) -> ContextGuard<'a> {
        ContextGuard::new(self)
    }

    pub fn plan(self, statement: &StatementTypeImpl)-> PlanNodeRef {
        statement.plan(&self)
    }

    pub fn get_and_increment_universal_id(&self) -> usize {
        self.universal_id.fetch_add(1, Ordering::SeqCst)
    }

    // pub fn bind_statement()
}
