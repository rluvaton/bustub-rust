use crate::constants::UNNAMED_COLUMN;
use crate::plan_nodes::window_plan_node::window_function::WindowFunction;
use crate::plan_nodes::{PlanNode, PlanType, WindowFunctionType};
use binder::OrderByType;
use catalog_schema::{Column, Schema};
use data_types::DBTypeId;
use expression::{Expression, ExpressionRef};
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::sync::Arc;

#[derive(Clone, Debug, PartialEq)]
pub struct WindowFunctionPlanNode {
    /**
     * The schema for the output of this plan node. In the volcano model, every plan node will spit out tuples,
     * and this tells you what schema this plan node's tuples will have.
     */
    output_schema: Arc<Schema>,

    /** The children of this plan node. */
    children: Vec<PlanType>,

    /** all columns expressions */
    columns: Vec<ExpressionRef>,

    window_functions: HashMap<usize, WindowFunction>,
}

impl WindowFunctionPlanNode {
    /**
    * Construct a new WindowFunctionPlanNode.
    * @param output_schema The output format of this plan node
    * @param child The child plan to aggregate data over
    * @param window_func_indexes The indexes of the window functions
    * @param columns All columns include the placeholder for window functions
    * @param partition_bys The partition by clause of the window functions
    * @param order_bys The order by clause of the window functions
    * @param funcions The expressions that we are aggregating
    * @param window_func_types The types that we are aggregating
    *
    * Window Aggregation is different from normal aggregation as it outputs one row for each inputing rows,
    * and can be combined with normal selected columns. The columns in WindowFunctionPlanNode contains both
    * normal selected columns and placeholder columns for window aggregations.
    *
    * For example, if we have a query like:
    *    SELECT 0.1, 0.2, SUM(0.3) OVER (PARTITION BY 0.2 ORDER BY 0.3), SUM(0.4) OVER (PARTITION BY 0.1 ORDER BY
    * 0.2,0.3) FROM table;
    *
    * The WindowFunctionPlanNode should contains following structure:
    *    columns: std::vector<AbstractExpressionRef>{0.1, 0.2, 0.-1(placeholder), 0.-1(placeholder)}
    *    partition_bys: std::vector<std::vector<AbstractExpressionRef>>{{0.2}, {0.1}}
    *    order_bys: std::vector<std::vector<AbstractExpressionRef>>{{0.3}, {0.2,0.3}}
    *    functions: std::vector<AbstractExpressionRef>{0.3, 0.4}
    *    window_func_types: std::vector<WindowFunctionType>{SumAggregate, SumAggregate}
    */
    pub fn new(
        output_schema: Arc<Schema>,
        child: PlanType,
        window_func_indexes: Vec<usize>,
        columns: Vec<ExpressionRef>,
        partition_bys: Vec<Vec<ExpressionRef>>,
        order_bys: Vec<Vec<(OrderByType, ExpressionRef)>>,
        functions: Vec<ExpressionRef>,
        window_func_types: Vec<WindowFunctionType>,
    ) -> Self {
        let window_functions = HashMap::from_iter(
            window_func_indexes
                .iter()
                .map(|&i| {
                    (
                        window_func_indexes[i],
                        WindowFunction {
                            function: functions[i].clone(),
                            fn_type: window_func_types[i],
                            partition_by: partition_bys[i].clone(),
                            order_by: order_bys[i].clone(),
                        }
                    )
                })
        );

        Self {
            output_schema,
            children: vec![child],
            columns,
            window_functions,
        }
    }

    /** @return the child of this aggregation plan node */
    pub fn get_child_plan(&self) -> &PlanType {
        assert_eq!(self.children.len(), 1, "Window Aggregation expected to only have one child.");
        &self.children[0]
    }

    pub fn infer_window_schema(columns: &[ExpressionRef]) -> Schema {
        // TODO(avery): correctly infer window call return type
        columns
            .iter()
            .map(|column| {
                // TODO(chi): correctly process VARCHAR column
                if column.get_return_type() == DBTypeId::VARCHAR {
                    Column::new_variable_size(UNNAMED_COLUMN.to_string(), column.get_return_type(), 128)
                } else {
                    Column::new_fixed_size(UNNAMED_COLUMN.to_string(), column.get_return_type())
                }
            })
            .into()
    }
}

impl Display for WindowFunctionPlanNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Into<PlanType> for WindowFunctionPlanNode {
    fn into(self) -> PlanType {
        PlanType::Window(self)
    }
}

impl PlanNode for WindowFunctionPlanNode {
    fn get_output_schema(&self) -> Arc<Schema> {
        self.output_schema.clone()
    }

    fn get_children(&self) -> &[PlanType] {
        self.children.as_ref()
    }
}
