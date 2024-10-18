use crate::plan_nodes::PlanType;
use crate::traits::Plan;
use crate::Planner;
use binder::CTERef;

impl Plan for CTERef {
    fn plan<'a>(&self, planner: &'a Planner<'a>)-> PlanType {
        let cte_list = planner.context.lock().cte_list.clone();

        cte_list
            .expect("Must have CTE")
            .iter()
            .find(|cte| cte.alias == self.cte_name)
            .expect("Must have CTE")
            .plan(planner)
    }
}
