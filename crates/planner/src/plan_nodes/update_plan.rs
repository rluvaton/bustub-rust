use std::fmt::{Display, Formatter};
use std::rc::Rc;
use std::sync::Arc;
use catalog_schema::Schema;
use common::config::TableOID;
use expression::ExpressionRef;
use crate::plan_nodes::{PlanNode, PlanType};


/**
 * The UpdatePlanNode identifies a table that should be updated.
 * The tuple(s) to be updated come from the child of the UpdateExecutor.
 */
#[derive(Clone, Debug, PartialEq)]
pub struct UpdatePlan {
    /**
     * The schema for the output of this plan node. In the volcano model, every plan node will spit out tuples,
     * and this tells you what schema this plan node's tuples will have.
     */
    output_schema: Arc<Schema>,

    /** The children of this plan node. */
    children: Vec<Rc<PlanType>>,

    /** The table to be updated. */
    table_oid: TableOID,

    /** The new expression at each column */
    target_expressions: Vec<ExpressionRef>
}

impl UpdatePlan {

    /**
     * Construct a new UpdatePlanNode instance.
     * @param child The child plan to obtain tuple from
     * @param table_oid The identifier of the table that should be updated
     * @param target_expressions The target expressions for new tuples
     */
    pub fn new(output: Arc<Schema>, child: Rc<PlanType>, table_oid: TableOID, target_expressions: Vec<ExpressionRef>) -> Self {
        Self {
            output_schema: output,
            children: vec![child],
            table_oid,
            target_expressions
        }
    }

    /** @return The identifier of the table that should be updated */
    pub fn get_table_oid(&self) -> TableOID { self.table_oid }

    /** The new expression at each column */
    pub fn get_target_expression(&self) -> &[ExpressionRef] { self.target_expressions.as_slice() }

    /** @return The child plan providing tuples to be inserted */
    pub fn get_child_plan(&self) -> &PlanType {
        assert_eq!(self.children.len(), 1, "UPDATE should have at most one child plan.");
        &self.children[0]
    }
}

impl Display for UpdatePlan {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Update")
            .field("table_oid", &self.table_oid)
            .field("target_expressions", &self.target_expressions)
            .finish()
    }
}

impl Into<PlanType> for UpdatePlan {
    fn into(self)-> PlanType {
        PlanType::Update(self)
    }
}

impl PlanNode for UpdatePlan {
    fn get_output_schema(&self) -> Arc<Schema> {
        self.output_schema.clone()
    }

    fn get_children(&self) -> &[Rc<PlanType>] {
        &self.children
    }
}
