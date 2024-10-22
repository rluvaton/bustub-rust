use crate::constants::UNNAMED_COLUMN;
use crate::plan_nodes::{AggregationType, PlanNode, PlanType};
use catalog_schema::{Column, Schema};
use data_types::{DBTypeId, Value};
use expression::{ConstantValueExpression, Expression, ExpressionRef, ExpressionType};
use std::fmt::{Display, Formatter};
use std::sync::Arc;


/**
 * AggregationPlanNode represents the various SQL aggregation functions.
 * For example, COUNT(), SUM(), MIN() and MAX().
 *
 * NOTE: To simplify this project, AggregationPlanNode must always have exactly one child.
 */
#[derive(Clone, Debug, PartialEq)]
pub struct AggregationPlanNode {
    /**
     * The schema for the output of this plan node. In the volcano model, every plan node will spit out tuples,
     * and this tells you what schema this plan node's tuples will have.
     */
    output_schema: Arc<Schema>,

    /** The children of this plan node. */
    children: Vec<PlanType>,

    /** The GROUP BY expressions */
    group_bys: Vec<ExpressionRef>,

    /** The aggregation expressions */
    aggregates: Vec<ExpressionRef>,

    /** The aggregation types */
    agg_types: Vec<AggregationType>,
}


/**
 * AggregationPlanNode represents the various SQL aggregation functions.
 * For example, COUNT(), SUM(), MIN() and MAX().
 *
 * NOTE: To simplify this project, AggregationPlanNode must always have exactly one child.
 */
impl AggregationPlanNode {
    /**
    * Construct a new AggregationPlanNode.
    * @param output_schema The output format of this plan node
    * @param child The child plan to aggregate data over
    * @param group_bys The group by clause of the aggregation
    * @param aggregates The expressions that we are aggregating
    * @param agg_types The types that we are aggregating
    */
    pub fn new(
        output_schema: Arc<Schema>,
        child: PlanType,
        group_bys: Vec<ExpressionRef>,
        aggregates: Vec<ExpressionRef>,
        agg_types: Vec<AggregationType>,
    ) -> Self {
        Self {
            output_schema,
            children: vec![child],
            group_bys,
            aggregates,
            agg_types,
        }
    }

    /** @return the child of this aggregation plan node */
    pub fn get_child_plan(&self) -> &PlanType {
        assert_eq!(self.children.len(), 1, "Aggregation expected to only have one child.");
        &self.children[0]
    }

    /** @return The idx'th group by expression */
    fn get_group_at(&self, index: usize) -> &ExpressionRef {
        &self.group_bys[index]
    }

    /** @return The group by expressions */
    pub fn get_group_bys(&self) -> &Vec<ExpressionRef> { &self.group_bys }

    /** @return The idx'th aggregate expression */
    pub fn get_aggregate_at(&self, index: usize) -> &ExpressionRef {
        &self.aggregates[index]
    }

    /** @return The aggregate expressions */
    pub fn get_aggregates(&self) -> &Vec<ExpressionRef> { &self.aggregates }

    /** @return The aggregate types */
    pub fn get_aggregate_types(&self) -> &Vec<AggregationType> { &self.agg_types }

    pub fn infer_agg_schema(group_bys: &[ExpressionRef], aggregates: &[ExpressionRef], _agg_types: &[AggregationType]) -> Schema {
        // TODO(avery): correctly infer window call return type
        let group_by_columns = group_bys
            .iter()
            .map(|column| {
                // TODO(chi): correctly process VARCHAR column
                if column.get_return_type() == DBTypeId::VARCHAR {
                    Column::new_variable_size(UNNAMED_COLUMN.to_string(), column.get_return_type(), 128)
                } else {
                    Column::new_fixed_size(UNNAMED_COLUMN.to_string(), column.get_return_type())
                }
            });

        let aggregate_columns = aggregates
            .iter()

            // TODO(chi): correctly infer agg call return type
            .map(|column| Column::new_fixed_size(UNNAMED_COLUMN.to_string(), DBTypeId::INT));

        group_by_columns
            .chain(aggregate_columns)
            .into()
    }
    
    pub fn create_internal_result_count(plan: PlanType, count_name: &str) -> Self {
        AggregationPlanNode::new(
            Arc::new(Schema::new(vec![
                Column::new_fixed_size(format!("__bustub_internal.{}", count_name), DBTypeId::INT)
            ])),
            plan,
            vec![],
            vec![ExpressionType::Constant(ConstantValueExpression::new(Value::from(1))).into_ref()],
            vec![AggregationType::CountStarAggregate]
        )
    }
}

impl Display for AggregationPlanNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Into<PlanType> for AggregationPlanNode {
    fn into(self) -> PlanType {
        PlanType::Aggregation(self)
    }
}

impl PlanNode for AggregationPlanNode {
    fn get_output_schema(&self) -> Arc<Schema> {
        self.output_schema.clone()
    }

    fn get_children(&self) -> &[PlanType] {
        self.children.as_ref()
    }
}
