use std::ops::{Deref, DerefMut};
use std::rc::Rc;
use std::sync::Arc;
use crate::Binder;
use crate::table_ref::{CTEList, TableRef, TableReferenceTypeImpl};

pub(crate) struct ContextGuard<'a> {
    old_scope: Option<Rc<TableReferenceTypeImpl>>,
    old_cte_scope: Option<Rc<CTEList>>,
    binder: &'a mut Binder
}

impl<'a> ContextGuard<'a> {
    pub(crate) fn new(binder: &'a mut Binder) -> Self {
        // Self {
        //     // do not reset CTE scope because we want to use those from parents
        //     old_scope: (*scope).take(),
        //     scope,
        //     old_cte_scope: cte_scope.clone(),
        //     cte_scope,
        // }

        Self {
            // Take the scope from binder and reset it
            old_scope: binder.scope.take(),

            // do not reset CTE scope because we want to use those from parents
            old_cte_scope: binder.cte_scope.clone(),
            binder,
        }
    }
}

impl<'a> Deref for ContextGuard<'a> {
    type Target = Binder;

    fn deref(&self) -> &Self::Target {
        self.binder
    }
}

impl<'a> DerefMut for ContextGuard<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.binder
    }
}

impl Drop for ContextGuard<'_> {
    fn drop(&mut self) {
        self.binder.scope = self.old_scope.clone();
        self.binder.cte_scope = self.old_cte_scope.clone();
    }
}
