use std::fmt::{Display, Formatter};
use std::rc::Rc;
use std::sync::Arc;
use catalog_schema::Schema;
use common::config::TableOID;
use expression::ExpressionRef;
use crate::plan_nodes::{PlanNode, PlanType};
use crate::plan_nodes::traits::EMPTY_CHILDREN;

/**
 * The ValuesPlanNode represents rows of values. For example,
 * `INSERT INTO table VALUES ((0, 1), (1, 2))`, where we will have
 * `(0, 1)` and `(1, 2)` as the output of this executor.
 */
#[derive(Clone, Debug, PartialEq)]
pub struct ValuesPlanNode {
    /**
     * The schema for the output of this plan node. In the volcano model, every plan node will spit out tuples,
     * and this tells you what schema this plan node's tuples will have.
     */
    output_schema: Arc<Schema>,

    values: Vec<Vec<ExpressionRef>>,
}

impl ValuesPlanNode {
    /**
     * Construct a new ValuesPlanNode instance.
     * @param output The output schema of this values plan node
     * @param values The values produced by this plan node
     */
    pub fn new(output: Arc<Schema>, values: Vec<Vec<ExpressionRef>>) -> Self {
        Self {
            output_schema: output,
            values,
        }
    }

    pub fn get_values(&self) -> &Vec<Vec<ExpressionRef>> { &self.values }
}

impl Display for ValuesPlanNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Values")
            .field("rows", &self.values.len())
            .finish()
    }
}

impl Into<PlanType> for ValuesPlanNode {
    fn into(self)-> PlanType {
        PlanType::Values(self)
    }
}

impl PlanNode for ValuesPlanNode {
    fn get_output_schema(&self) -> Arc<Schema> {
        self.output_schema.clone()
    }

    fn get_children(&self) -> &[PlanType] {
        EMPTY_CHILDREN
    }
}
