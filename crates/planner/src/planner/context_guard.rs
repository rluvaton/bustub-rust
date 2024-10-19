use crate::{Planner};
use std::ops::Deref;
use crate::planner::Context;

pub(crate) struct ContextGuard<'a> {
    old_context: Context,
    planner: &'a Planner<'a>
}

impl<'a> ContextGuard<'a> {
    pub(crate) fn new(planner: &'a Planner) -> Self {
        let mut context = planner.context.lock();
        let old_context = context.clone();

        let mut new_context = Context::default();
        new_context.cte_list = old_context.cte_list.clone();
        *context = new_context;

        Self {
            old_context,
            planner,
        }
    }
}

impl<'a> Deref for ContextGuard<'a> {
    type Target = Planner<'a>;

    fn deref(&self) -> &Self::Target {
        self.planner
    }
}

impl Drop for ContextGuard<'_> {
    fn drop(&mut self) {
        let mut c = self.planner.context.lock();
        *c = self.old_context.clone()
    }
}
