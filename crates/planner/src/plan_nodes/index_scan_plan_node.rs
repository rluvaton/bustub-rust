use std::fmt::{Display, Formatter};
use std::rc::Rc;
use std::sync::Arc;
use binder::BaseTableRef;
use catalog_schema::{Column, Schema};
use common::config::{IndexOID, TableOID};
use expression::{ConstantValueExpression, ExpressionRef};
use crate::plan_nodes::{PlanNode, PlanType};

const EMPTY_CHILDREN: &'static [Rc<PlanType>] = &[];


/**
 * The SeqScanPlanNode represents a sequential table scan operation.
 */
#[derive(Clone, Debug, PartialEq)]
pub struct IndexScanPlanNode {
    /**
     * The schema for the output of this plan node. In the volcano model, every plan node will spit out tuples,
     * and this tells you what schema this plan node's tuples will have.
     */
    output_schema: Arc<Schema>,

    /** The table which the index is created on. */
    table_oid: TableOID,

    /** The index whose tuples should be scanned. */
    index_oid: IndexOID,

    /** The predicate to filter in index scan.
      * For Fall 2023, after you implemented seqscan to indexscan optimizer rule,
      * we can use this predicate to do index point lookup
     */
    filter_predicate: Option<ExpressionRef>,

    /**
     * The constant value key to lookup.
     * For example when dealing "WHERE v = 1" we could store the constant value 1 here
     */
    pred_key: Option<ConstantValueExpression>
}

impl IndexScanPlanNode {
    /**
     * Creates a new index scan plan node with filter predicate.
     * @param output The output format of this scan plan node
     * @param table_oid The identifier of table to be scanned
     * @param filter_predicate The predicate pushed down to index scan.
     * @param pred_key The key for point lookup
     */
    pub fn new(output: Arc<Schema>, table_oid: TableOID, index_oid: IndexOID, filter_predicate: Option<ExpressionRef>, pred_key: Option<ConstantValueExpression>) -> Self {
        Self {
            output_schema: output,
            table_oid,
            index_oid,
            filter_predicate,
            pred_key
        }
    }

    /** @return The identifier of the table that should be scanned */
    pub fn get_table_oid(&self) -> TableOID { self.table_oid }

    /** @return the identifier of the table that should be scanned */
    pub fn get_index_oid(&self) -> IndexOID { self.index_oid }
}

impl Display for IndexScanPlanNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut f_debug = f.debug_struct("IndexScan");
        f_debug
            .field("index_oid", &self.index_oid);

        if let Some(filter) = &self.filter_predicate {
            f_debug.field("filter", filter);
        }

        f_debug
            .finish()
    }
}

impl Into<PlanType> for IndexScanPlanNode {
    fn into(self) -> PlanType {
        PlanType::IndexScan(self)
    }
}

impl PlanNode for IndexScanPlanNode {
    fn get_output_schema(&self) -> Arc<Schema> {
        self.output_schema.clone()
    }

    fn get_children(&self) -> &[Rc<PlanType>] {
        EMPTY_CHILDREN
    }
}
