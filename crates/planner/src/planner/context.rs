use binder::{CTEList, ExpressionTypeImpl};
use expression::ExpressionRef;
use std::rc::Rc;

/**
 * The context for the planner. Used for planning aggregation calls.
 */
#[derive(Clone)]
pub(crate) struct Context {

    /** Indicates whether aggregation is allowed in this context. */
    pub(crate) allow_aggregation: bool,

    /** Indicates the next agg call to be processed in this context. */
    pub(crate) next_aggregation: usize,

    /**
     * In the first phase of aggregation planning, we put all agg calls expressions into this vector.
     * The expressions in this vector should be used over the output of the original filter / table
     * scan plan node.
     */
    pub(crate) aggregations: Vec<Rc<ExpressionTypeImpl>>,

    /**
     * In the second phase of aggregation planning, we plan agg calls from `aggregations_`, and generate
     * an aggregation plan node. The expressions in the vector should be used over the output from the
     * aggregation plan node.
     */
    pub(crate) expr_in_agg: Vec<ExpressionRef>,

    /**
     * CTE in scope.
     */
    pub(crate) cte_list: Option<Rc<CTEList>>,
}

impl Context {
    pub(crate) fn add_aggregations(&mut self, expr: Rc<ExpressionTypeImpl>) {
        assert!(self.allow_aggregation, "AggCall not allowed in this position");
        self.aggregations.push(expr);
    }
}

impl Default for Context {
    fn default() -> Self {
        Self {
            allow_aggregation: false,
            next_aggregation: 0,
            aggregations: vec![],
            expr_in_agg: vec![],
            cte_list: None,
        }
    }
}
