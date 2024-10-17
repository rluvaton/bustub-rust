use std::rc::Rc;
use std::sync::Arc;
use binder::BaseTableRef;
use crate::plan_nodes::PlanType;
use crate::{MockScanPlanNode, PlanNode, Planner, SeqScanPlanNode};
use crate::traits::Plan;

impl Plan for BaseTableRef {
    fn plan<'a>(&self, planner: &'a Planner<'a>) -> Rc<PlanType> {
        // We always scan ALL columns of the table, and use projection executor to
        // remove some of them, therefore simplifying the planning process.

        // It is also possible to have a "virtual" logical projection node, and
        // we can merge it with table scan when optimizing. But for now, having
        // an optimizer or not remains undecided. So I'd prefer going with a new
        // ProjectionExecutor.
        let table = planner.catalog.get_table_by_name(self.table.as_str()).expect("Must have table");

        if table.get_name().starts_with("__") {
            // Plan as MockScanExecutor if it is a mock table.
            assert!(table.get_name().starts_with("__mock"), "unsupported internal table: {}", table.get_name());
            return MockScanPlanNode::new(
                Arc::new(SeqScanPlanNode::infer_scan_schema(self)),
                table.get_name().clone(),
            ).into_ref();
        }

        // Otherwise, plan as normal SeqScan.
        SeqScanPlanNode::new(
            Arc::new(SeqScanPlanNode::infer_scan_schema(self)),
            table.get_oid(),
            table.get_name().clone(),
            None
        ).into_ref()
    }
}
