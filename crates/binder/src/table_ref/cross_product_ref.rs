use crate::table_ref::table_reference_type::{TableReferenceType, TableReferenceTypeImpl};
use crate::table_ref::TableRef;

#[derive(Debug, PartialEq)]
pub struct CrossProductRef {

    /** The left side of the cross product. */
    pub(crate) left: Box<TableReferenceTypeImpl>,

    /** The right side of the cross product. */
    pub(crate) right: Box<TableReferenceTypeImpl>,
}

impl CrossProductRef {
    pub(crate) fn new(left: Box<TableReferenceTypeImpl>, right: Box<TableReferenceTypeImpl>) -> Self {
        Self {
            left,
            right
        }
    }
}

impl TableRef for CrossProductRef {
    const TYPE: TableReferenceType = TableReferenceType::CrossProduct;
}

