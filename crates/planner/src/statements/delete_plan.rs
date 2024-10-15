use std::fmt::{Display, Formatter};
use std::sync::Arc;
use binder::{DeleteStatement, Statement};
use catalog_schema::Schema;
use common::config::TableOID;
use crate::plan_nodes::{PlanNode, PlanType};
use crate::Planner;
use crate::statements::traits::StatementPlan;

/**
 * The DeletePlanNode identifies a table from which tuples should be deleted.
 * The tuple(s) to be updated come from the child of the DeleteExecutor.
 *
 * NOTE: To simplify the assignment, DeletePlanNode has at most one child.
 */
#[derive(Clone, Debug)]
pub struct DeletePlan {
    /**
     * The schema for the output of this plan node. In the volcano model, every plan node will spit out tuples,
     * and this tells you what schema this plan node's tuples will have.
     */
    output_schema: Arc<Schema>,

    /** The children of this plan node. */
    children: Vec<PlanType>,

    /** The identifier of the table from which tuples are deleted */
    table_oid: TableOID,
}

impl DeletePlan {
    /**
     * Construct a new DeletePlanNode.
     * @param child The child plan to obtain tuple from
     * @param table_oid The identifier of the table from which tuples are deleted
     */
    pub fn new(output: Arc<Schema>, child: PlanType, table_oid: TableOID) -> Self {
        Self {
            output_schema: output,
            children: vec![child],
            table_oid,
        }
    }

    /** @return The identifier of the table from which tuples are deleted*/
    pub fn get_table_oid(&self) -> TableOID { self.table_oid }

    /** @return The child plan providing tuples to be deleted */
    pub fn get_child_plan(&self) -> &PlanType {
        assert_eq!(self.children.len(), 1, "delete should have at most one child plan.");
        &self.children[0]
    }
}

impl Display for DeletePlan {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Foo")
            .field("table_oid", &self.table_oid)
            .finish()
    }
}

impl PlanNode for DeletePlan {
    fn output_schema(&self) -> Arc<Schema> {
        self.output_schema.clone()
    }

    fn get_children(&self) -> &Vec<PlanType> {
        &self.children
    }
}

impl StatementPlan for DeletePlan {
    type Statement = DeleteStatement;

    fn create_node<'a>(statement: Self::Statement, planner: &'a Planner<'a>) -> Self {
        todo!()
    }
}
