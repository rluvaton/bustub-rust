use std::sync::Arc;
use crate::table_ref::{CTEList, TableRef};

pub(crate) struct ContextGuard {
    old_scope: Option<Arc<dyn TableRef>>,


    // was originally scope_ptr
    scope: Arc<Option<Arc<dyn TableRef>>>,

    old_cte_scope: Arc<CTEList>,

    // was originally cte_scope_ptr_
    cte_scope:Arc<Option<Arc<CTEList>>>,
}

impl ContextGuard {
    pub(crate) fn new(scope: Arc<Option<Arc<dyn TableRef>>>, cte_scope: Arc<Option<Arc<CTEList>>>) -> Self {
        // Self {
        //     // do not reset CTE scope because we want to use those from parents
        //     old_scope: (*scope).take(),
        //     scope,
        //     old_cte_scope: cte_scope.clone(),
        //     cte_scope,
        // }

        todo!()
    }
}
