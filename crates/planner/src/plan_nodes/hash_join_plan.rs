use std::fmt::{Display, Formatter};
use std::rc::Rc;
use std::sync::Arc;
use binder::JoinType;
use catalog_schema::Schema;
use common::config::TableOID;
use expression::ExpressionRef;
use crate::plan_nodes::{PlanNode, PlanType};
use crate::PlanNodeRef;

/**
 * Hash join performs a JOIN operation with a hash table.
 */
#[derive(Clone, Debug, PartialEq)]
pub struct HashJoinPlan {
    /**
     * The schema for the output of this plan node. In the volcano model, every plan node will spit out tuples,
     * and this tells you what schema this plan node's tuples will have.
     */
    output_schema: Arc<Schema>,

    /** The children of this plan node. */
    children: Vec<PlanNodeRef>,

    /** The expression to compute the left JOIN key */
    left_key_expressions: Vec<ExpressionRef>,
    /** The expression to compute the right JOIN key */
    right_key_expressions: Vec<ExpressionRef>,

    /** The join type */
    join_type: JoinType,
}

impl HashJoinPlan {
    /**
    * Construct a new HashJoinPlanNode instance.
    * @param output_schema The output schema for the JOIN
    * @param children The child plans from which tuples are obtained
    * @param left_key_expression The expression for the left JOIN key
    * @param right_key_expression The expression for the right JOIN key
    */
    pub fn new(output: Arc<Schema>, left: PlanNodeRef, right: PlanNodeRef, left_key_expressions: Vec<ExpressionRef>, right_key_expressions: Vec<ExpressionRef>, join_type: JoinType) -> Self {
        Self {
            output_schema: output,
            children: vec![left, right],
            left_key_expressions,
            right_key_expressions,
            join_type
        }
    }


    /** @return The expression to compute the left join key */
    pub fn get_left_join_key_expressions(&self) -> &Vec<ExpressionRef> { &self.left_key_expressions }

    /** @return The expression to compute the right join key */
    pub fn get_right_join_key_expressions(&self) -> &Vec<ExpressionRef> { &self.right_key_expressions }

    /** @return The left plan node of the hash join */
    pub fn get_left_plan(&self) -> PlanNodeRef {
        assert_eq!(self.children.len(), 2, "Hash joins should have exactly two children plans.");
        self.children[0].clone()
    }

/** @return The right plan node of the hash join */

pub fn get_right_plan(&self) -> PlanNodeRef {
    assert_eq!(self.children.len(), 2, "Hash joins should have exactly two children plans.");
    self.children[1].clone()
}

/** @return The join type used in the hash join */
pub fn get_join_type(&self) -> JoinType { self.join_type }
}

impl Display for HashJoinPlan {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HashJoin")
            .field("join_type", &self.join_type)
            .field("left_plan", &self.get_left_plan())
            .field("right_plan", &self.get_right_plan())
            .field("left_join_key_expressions", &self.get_left_join_key_expressions())
            .field("right_join_key_expressions", &self.get_right_join_key_expressions())
            .finish()
    }
}

impl Into<PlanType> for HashJoinPlan {
    fn into(self)-> PlanType {
        PlanType::HashJoin(self)
    }
}

impl PlanNode for HashJoinPlan {
    fn get_output_schema(&self) -> Arc<Schema> {
        self.output_schema.clone()
    }

    fn get_children(&self) -> &[Rc<PlanType>] {
        &self.children
    }
}
