use crate::statements::{DeletePlan, InsertPlan};

/** PlanType represents the types of plans that we have in our system. */

#[derive(Clone, Debug, PartialEq)]
pub enum PlanType {
    // SeqScan,
    // IndexScan,
    Insert(InsertPlan),
    // Update,
    Delete(DeletePlan),
    // Aggregation,
    // Limit,
    // NestedLoopJoin,
    // NestedIndexJoin,
    // HashJoin,
    // Filter,
    // Values,
    // Projection,
    // Sort,
    // TopN,
    // TopNPerGroup,
    // MockScan,
    // InitCheck,
    // Window
}
