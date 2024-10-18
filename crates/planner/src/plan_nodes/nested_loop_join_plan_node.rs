use std::fmt::{Display, Formatter};
use std::rc::Rc;
use std::sync::Arc;
use binder::{BaseTableRef, JoinType};
use catalog_schema::{Column, Schema};
use common::config::{IndexOID, TableOID};
use expression::{ConstantValueExpression, ExpressionRef};
use crate::plan_nodes::{PlanNode, PlanType};
use crate::PlanNodeRef;

const EMPTY_CHILDREN: &'static [Rc<PlanType>] = &[];


/**
 * NestedLoopJoinPlanNode joins tuples from two child plan nodes.
 */
#[derive(Clone, Debug, PartialEq)]
pub struct NestedLoopJoinPlanNode {
    /**
     * The schema for the output of this plan node. In the volcano model, every plan node will spit out tuples,
     * and this tells you what schema this plan node's tuples will have.
     */
    output_schema: Arc<Schema>,

    children: Vec<PlanNodeRef>,

    /** The join predicate */
    predicate: ExpressionRef,

    /** The join type */
    join_type: JoinType,
}

impl NestedLoopJoinPlanNode {
    /**
    * Construct a new NestedLoopJoinPlanNode instance.
    * @param output The output format of this nested loop join node
    * @param children Two sequential scan children plans
    * @param predicate The predicate to join with, the tuples are joined
    * if predicate(tuple) = true.
    */
    pub fn new(output: Arc<Schema>, left: PlanNodeRef, right: PlanNodeRef, predicate: ExpressionRef, join_type: JoinType) -> Self {
        Self {
            output_schema: output,
            children: vec![left, right],
            predicate,
            join_type
        }
    }

    /** @return The predicate to be used in the nested loop join */
    pub fn get_predicate(&self) -> ExpressionRef {
        self.predicate.clone()
    }

    /** @return The left plan node of the nested loop join, by convention it should be the smaller table */
    pub fn get_left_plan(&self) -> PlanNodeRef {
        assert_eq!(self.children.len(), 2, "Nested loop joins should have exactly two children plans.");
        self.children[0].clone()
    }

    /** @return The right plan node of the nested loop join */
    pub fn get_right_plan(&self) -> PlanNodeRef {
        assert_eq!(self.children.len(), 2, "Nested loop joins should have exactly two children plans.");
        self.children[1].clone()
    }

    /** @return The join type used in the hash join */
    pub fn get_join_type(&self) -> JoinType { self.join_type }
}

impl Display for NestedLoopJoinPlanNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NestedLoopJoin")
            .field("join_type", &self.join_type)
            .field("predicate", &self.predicate)
            .finish()
    }
}

impl Into<PlanType> for NestedLoopJoinPlanNode {
    fn into(self) -> PlanType {
        PlanType::NestedLoopJoin(self)
    }
}

impl PlanNode for NestedLoopJoinPlanNode {
    fn get_output_schema(&self) -> Arc<Schema> {
        self.output_schema.clone()
    }

    fn get_children(&self) -> &[Rc<PlanType>] {
        self.children.as_slice()
    }
}
