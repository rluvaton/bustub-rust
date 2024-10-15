use crate::Binder;
use std::ops::Deref;
use crate::binder::context::Context;

pub(crate) struct ContextGuard<'a> {
    old_context: Context,
    binder: &'a Binder<'a>
}

impl<'a> ContextGuard<'a> {
    pub(crate) fn new(binder: &'a Binder) -> Self {
        // Self {
        //     // do not reset CTE scope because we want to use those from parents
        //     old_scope: (*scope).take(),
        //     scope,
        //     old_cte_scope: cte_scope.clone(),
        //     cte_scope,
        // }

        let context = binder.context.lock();
        let mut old_context = context.clone();

        // Reset the scope
        old_context.scope.take();

        // do not reset CTE scope because we want to use those from parents

        Self {
            old_context,
            binder,
        }
    }
}

impl<'a> Deref for ContextGuard<'a> {
    type Target = Binder<'a>;

    fn deref(&self) -> &Self::Target {
        self.binder
    }
}

impl Drop for ContextGuard<'_> {
    fn drop(&mut self) {
        let mut c = self.binder.context.lock();
        *c = self.old_context.clone()
    }
}
