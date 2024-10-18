use std::fmt::{Display, Formatter};
use std::rc::Rc;
use std::sync::Arc;
use binder::BaseTableRef;
use catalog_schema::{Column, Schema};
use common::config::TableOID;
use expression::ExpressionRef;
use crate::plan_nodes::{PlanNode, PlanType};
use crate::plan_nodes::traits::EMPTY_CHILDREN;

/**
 * The SeqScanPlanNode represents a sequential table scan operation.
 */
#[derive(Clone, Debug, PartialEq)]
pub struct SeqScanPlanNode {
    /**
     * The schema for the output of this plan node. In the volcano model, every plan node will spit out tuples,
     * and this tells you what schema this plan node's tuples will have.
     */
    output_schema: Arc<Schema>,

    /** The table whose tuples should be scanned */
    table_oid: TableOID,

    /** The table name */
    table_name: String,

    /** The predicate to filter in seqscan.
         * For Fall 2023, We'll enable the MergeFilterScan rule, so we can further support index point lookup
     */
    filter_predicate: Option<ExpressionRef>,
}

impl SeqScanPlanNode {
    /**
    * Construct a new SeqScanPlanNode instance.
    * @param output The output schema of this sequential scan plan node
    * @param table_oid The identifier of table to be scanned
    */
    pub fn new(output: Arc<Schema>, table_oid: TableOID, table_name: String, filter_predicate: Option<ExpressionRef>) -> Self {
        Self {
            output_schema: output,
            table_oid,
            table_name,
            filter_predicate,
        }
    }

    /** @return The identifier of the table that should be scanned */
    pub fn get_table_oid(&self) -> TableOID { self.table_oid }

    pub fn infer_scan_schema(table: &BaseTableRef) -> Schema {
        table.schema
            .get_columns()
            .iter()
            .map(|col| Column::create_new_name(format!("{}.{}", table.get_table_name(), col.get_name()), col))
            .into()
    }
}

impl Display for SeqScanPlanNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut f_debug = f.debug_struct("SeqScan");
        f_debug
            .field("table", &self.table_name);

        if let Some(filter) = &self.filter_predicate {
            f_debug.field("filter", filter);
        }

        f_debug
            .finish()
    }
}

impl Into<PlanType> for SeqScanPlanNode {
    fn into(self) -> PlanType {
        PlanType::SeqScan(self)
    }
}

impl PlanNode for SeqScanPlanNode {
    fn get_output_schema(&self) -> Arc<Schema> {
        self.output_schema.clone()
    }

    fn get_children(&self) -> &[PlanType] {
        EMPTY_CHILDREN
    }
}
