use crate::table_ref::table_reference_type::{TableReferenceType, TableReferenceTypeImpl};
use crate::table_ref::TableRef;

#[derive(Debug, PartialEq)]
pub struct CTERef {
    pub(crate) cte_name: String,
    pub(crate) alias: String,
}

impl CTERef {
    pub(crate) fn new(cte_name: String, alias: String) -> Self {
        Self {
            cte_name,
            alias,
        }
    }
}

impl TableRef for CTERef {
}

impl From<CTERef> for TableReferenceTypeImpl {
    fn from(value: CTERef) -> Self {
        TableReferenceTypeImpl::CTE(value)
    }
}
