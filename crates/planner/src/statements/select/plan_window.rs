use std::rc::Rc;
use binder::SelectStatement;
use crate::plan_nodes::PlanType;
use crate::Planner;

pub(crate) trait PlanWindow {
    fn plan_window(&self, plan: Rc<PlanType>, planner: &Planner) -> Rc<PlanType>;
}

impl PlanWindow for SelectStatement {
    fn plan_window(&self, plan: Rc<PlanType>, planner: &Planner) -> Rc<PlanType> {
        /* For window function we don't do two passes rewrites like planning normal aggregations.
   *  Because standard sql does not allow using window function results in where clause, and
   *  our implementation does not support having on window function. We assume window functions
   *  only appear in select list, and we can plan them in one pass.
   */

        todo!()
    }
}
