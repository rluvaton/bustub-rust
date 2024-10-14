use crate::expressions::ExpressionTypeImpl;
use crate::table_ref::table_reference_type::{TableReferenceType, TableReferenceTypeImpl};
use crate::table_ref::TableRef;

#[derive(Debug, PartialEq)]
pub enum JoinType {
    Invalid,
    Left,
    Right,
    Inner,
    Outer
}

#[derive(Debug, PartialEq)]
pub struct JoinRef {
    /// Type of join.
    pub(crate) join_type: JoinType,

    /// The left side of the join.
    pub(crate) left: Box<TableReferenceTypeImpl>,

    /** The right side of the join. */
    pub(crate) right: Box<TableReferenceTypeImpl>,

    /** Join condition. */
    pub(crate) condition: Option<ExpressionTypeImpl>,
}

impl JoinRef {
    pub(crate) fn new(join_type: JoinType,
                      left: Box<TableReferenceTypeImpl>,
                      right: Box<TableReferenceTypeImpl>,
                      condition: Option<ExpressionTypeImpl>) -> Self {
        Self {
            join_type,
            left,
            right,
            condition,
        }
    }
}

impl TableRef for JoinRef {
}

impl From<JoinRef> for TableReferenceTypeImpl {
    fn from(value: JoinRef) -> Self {
        TableReferenceTypeImpl::Join(value)
    }
}

