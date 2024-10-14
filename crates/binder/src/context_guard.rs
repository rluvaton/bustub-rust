use std::sync::Arc;
use crate::table_ref::{CTEList, TableRef, TableReferenceTypeImpl};

pub(crate) struct ContextGuard {
    old_scope: Option<Arc<TableReferenceTypeImpl>>,


    // was originally scope_ptr
    scope: Arc<Option<Arc<TableReferenceTypeImpl>>>,

    old_cte_scope: Arc<CTEList>,

    // was originally cte_scope_ptr_
    cte_scope:Arc<Option<Arc<CTEList>>>,
}

impl ContextGuard {
    pub(crate) fn new(scope: Arc<Option<Arc<TableReferenceTypeImpl>>>, cte_scope: Arc<Option<Arc<CTEList>>>) -> Self {
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
