use std::fmt::{Display, Formatter};
use std::rc::Rc;
use std::sync::Arc;
use catalog_schema::{Column, Schema};
use common::config::TableOID;
use data_types::DBTypeId;
use expression::{Expression, ExpressionRef};
use crate::constants::UNNAMED_COLUMN;
use crate::plan_nodes::{PlanNode, PlanNodeRef, PlanType};

/**
 * The ProjectionPlanNode represents a project operation.
 * It computes expressions based on the input.
 */
#[derive(Clone, Debug, PartialEq)]
pub struct ProjectionPlanNode {
    /**
     * The schema for the output of this plan node. In the volcano model, every plan node will spit out tuples,
     * and this tells you what schema this plan node's tuples will have.
     */
    output_schema: Arc<Schema>,

    /** The children of this plan node. */
    children: Vec<PlanNodeRef>,

    expressions: Vec<ExpressionRef>
}

impl ProjectionPlanNode {

    /**
     * Construct a new ProjectionPlanNode instance.
     * @param output The output schema of this projection node
     * @param expressions The expression to evaluate
     * @param child The child plan node
     */
    pub fn new(output: Arc<Schema>, expressions: Vec<ExpressionRef>, child: PlanNodeRef) -> Self {
        Self {
            output_schema: output,
            children: vec![child],
            expressions,
        }
    }


    /** @return Projection expressions */
    pub fn get_expressions(&self) -> &Vec<ExpressionRef> { &self.expressions }

    /** @return The child plan providing tuples to be deleted */
    pub fn get_child_plan(&self) -> PlanNodeRef {
        assert_eq!(self.children.len(), 1, "Projection should have exactly one child plan.");
        self.children[0].clone()
    }

    pub fn infer_projection_schema(expressions: &[ExpressionRef]) -> Schema {
        expressions
            .iter()
            .map(|column| {
                if column.get_return_type() == DBTypeId::VARCHAR {
                    // TODO(chi): infer the correct VARCHAR length. Maybe it doesn't matter for executors?
                    Column::new_variable_size(UNNAMED_COLUMN.to_string(), column.get_return_type(), 128)
                } else {
                    Column::new_fixed_size(UNNAMED_COLUMN.to_string(), column.get_return_type())
                }
            })
            .into()
    }

    pub fn rename_schema(schema: Schema, col_names: &[String]) -> Schema {
        assert_eq!(col_names.len(), schema.get_column_count(), "mismatched number of columns");

        col_names.iter()
            .zip(schema.get_columns())
            .map(|(name, column)| Column::create_new_name(name.to_string(), column))
            .into()
    }
}

impl Display for ProjectionPlanNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Projection")
            .field("expressions", &self.expressions)
            .finish()
    }
}

impl Into<PlanType> for ProjectionPlanNode {
    fn into(self)-> PlanType {
        PlanType::Projection(self)
    }
}

impl PlanNode for ProjectionPlanNode {
    fn get_output_schema(&self) -> Arc<Schema> {
        self.output_schema.clone()
    }

    fn get_children(&self) -> &[Rc<PlanType>] {
        &self.children
    }
}
