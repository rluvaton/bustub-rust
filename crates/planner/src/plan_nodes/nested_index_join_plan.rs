use std::fmt::{Display, Formatter};
use std::rc::Rc;
use std::sync::Arc;
use binder::JoinType;
use catalog_schema::Schema;
use common::config::{IndexOID, TableOID};
use expression::ExpressionRef;
use crate::plan_nodes::{PlanNode, PlanType};

/**
 * NestedIndexJoinPlan is used to represent performing a nested index join between two tables
 * The outer table tuples are propagated using a child executor, but the inner table tuples should be
 * obtained using the outer table tuples as well as the index from the catalog.
 */
#[derive(Clone, Debug, PartialEq)]
pub struct NestedIndexJoinPlan {
    /**
     * The schema for the output of this plan node. In the volcano model, every plan node will spit out tuples,
     * and this tells you what schema this plan node's tuples will have.
     */
    output_schema: Arc<Schema>,

    /** The children of this plan node. */
    children: Vec<PlanType>,

    key_predicate: ExpressionRef,

    inner_table_oid: TableOID,
    index_oid: IndexOID,
    index_name: String,
    index_table_name: String,
    inner_table_schema: Arc<Schema>,

    /** The join type */
    join_type: JoinType,
}

impl NestedIndexJoinPlan {
    pub fn new(output: Arc<Schema>, child: PlanType, key_predicate: ExpressionRef, inner_table_oid: TableOID, index_oid: IndexOID, index_name: String, index_table_name: String, inner_table_schema: Arc<Schema>, join_type: JoinType) -> Self {
        Self {
            output_schema: output,
            children: vec![child],
            key_predicate,
            inner_table_oid,
            index_oid,
            index_name,
            index_table_name,
            inner_table_schema,
            join_type,
        }
    }


    /** @return The predicate to be used to extract the join key from the child */
    pub fn get_key_predicate(&self) -> ExpressionRef { self.key_predicate.clone() }

    /** @return The plan node for the outer table of the nested index join */
    pub fn get_child_plan(&self) -> &PlanType { &self.children[0] }

    /** @return The table oid for the inner table of the nested index join */
    pub fn get_inner_table_oid(&self) -> TableOID { self.inner_table_oid }

    /** @return The index associated with the nested index join */
    pub fn get_index_name(&self) -> &str { self.index_name.as_str() }

    /** @return The index oid associated with the nested index join */
    pub fn get_index_oid(&self) -> IndexOID { self.index_oid }

    /** @return Schema with needed columns in from the inner table */
    pub fn get_inner_table_schema(&self) -> Arc<Schema> { self.inner_table_schema.clone() }

    /** @return The join type used in the nested index join */
    pub fn get_join_type(&self) -> JoinType { self.join_type }
}

impl Display for NestedIndexJoinPlan {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NestedIndexJoin")
            .field("join_type", &self.join_type)
            .field("key_predicate", &self.get_key_predicate())
            .field("index", &self.get_index_name())
            .field("index_table", &self.index_table_name)
            .finish()
    }
}

impl Into<PlanType> for NestedIndexJoinPlan {
    fn into(self) -> PlanType {
        PlanType::NestedIndexJoin(self)
    }
}

impl PlanNode for NestedIndexJoinPlan {
    fn get_output_schema(&self) -> Arc<Schema> {
        self.output_schema.clone()
    }

    fn get_children(&self) -> &[PlanType] {
        &self.children
    }
}
