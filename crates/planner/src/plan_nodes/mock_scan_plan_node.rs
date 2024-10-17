use crate::plan_nodes::{PlanNode, PlanType};
use catalog_schema::Schema;
use std::fmt::{Display, Formatter};
use std::rc::Rc;
use std::sync::Arc;

const EMPTY_CHILDREN: &'static [Rc<PlanType>] = &[];


/**
 * The MockScanPlanNode represents a "dummy" sequential
 * scan over a table, without requiring the table to exist.
 * NOTE: This class is used solely for testing.
 */
#[derive(Clone, Debug, PartialEq)]
pub struct MockScanPlanNode {
    /**
     * The schema for the output of this plan node. In the volcano model, every plan node will spit out tuples,
     * and this tells you what schema this plan node's tuples will have.
     */
    output_schema: Arc<Schema>,

    table: String
}

impl MockScanPlanNode {
    /**
     * Construct a new MockScanPlanNode instance.
     * @param output The output schema of this mock scan plan node
     */
    pub fn new(output: Arc<Schema>, table: String) -> Self {
        Self {
            output_schema: output,
            table,
        }
    }

    /** @return The table name of this mock scan node, used to determine the generated content. */
    pub fn get_table(&self) -> &String { &self.table }
}

impl Display for MockScanPlanNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MockScan")
            .field("table", &self.table)
            .finish()
    }
}

impl Into<PlanType> for MockScanPlanNode {
    fn into(self)-> PlanType {
        PlanType::MockScan(self)
    }
}

impl PlanNode for MockScanPlanNode {
    fn get_output_schema(&self) -> Arc<Schema> {
        self.output_schema.clone()
    }

    fn get_children(&self) -> &[Rc<PlanType>] {
        EMPTY_CHILDREN
    }
}
