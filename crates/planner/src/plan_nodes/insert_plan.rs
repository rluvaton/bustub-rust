use crate::plan_nodes::{PlanNode, PlanType};
use catalog_schema::Schema;
use std::fmt::{Display, Formatter};
use std::rc::Rc;
use std::sync::Arc;
use common::config::TableOID;

/**
 * The InsertPlanNode identifies a table into which tuples are inserted.
 *
 * The values to be inserted will come from the child of the node.
 */
#[derive(Clone, Debug, PartialEq)]
pub struct InsertPlan {
    /**
     * The schema for the output of this plan node. In the volcano model, every plan node will spit out tuples,
     * and this tells you what schema this plan node's tuples will have.
     */
    output_schema: Arc<Schema>,

    /** The children of this plan node. */
    children: Vec<Rc<PlanType>>,

    /** The identifier of the table from which tuples are inserted into */
    table_oid: TableOID,
}

impl InsertPlan {
    pub fn new(output: Arc<Schema>, child: Rc<PlanType>, table_oid: TableOID) -> Self {
        Self {
            output_schema: output,
            children: vec![child],
            table_oid,
        }
    }

    /** @return The identifier of the table into which tuples are inserted */
    pub fn get_table_oid(&self) -> TableOID { self.table_oid }

    /** @return the child plan providing tuples to be inserted */
    pub fn get_child_plan(&self) -> &PlanType {
        assert_eq!(self.children.len(), 1, "insert should have only one child plan.");
        &self.children[0]
    }
}

impl Display for InsertPlan {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Insert")
            .field("table_oid", &self.table_oid)
            .finish()
    }
}

impl Into<PlanType> for InsertPlan {
    fn into(self)-> PlanType {
        PlanType::Insert(self)
    }
}

impl PlanNode for InsertPlan {
    fn get_output_schema(&self) -> Arc<Schema> {
        self.output_schema.clone()
    }

    fn get_children(&self) -> &[Rc<PlanType>] {
        &self.children
    }
}
