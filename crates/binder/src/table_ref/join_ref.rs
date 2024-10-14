use crate::Binder;
use crate::expressions::{ColumnRef, ExpressionTypeImpl};
use crate::table_ref::table_reference_type::{TableReferenceType, TableReferenceTypeImpl};
use crate::table_ref::TableRef;
use crate::try_from_ast_error::{ParseASTError, ParseASTResult};

#[derive(Clone, Debug, PartialEq)]
pub enum JoinType {
    Invalid,
    Left,
    Right,
    Inner,
    Outer
}

#[derive(Clone, Debug, PartialEq)]
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
    fn resolve_column(&self, col_name: &[String], binder: &Binder) -> ParseASTResult<Option<ColumnRef>> {
        let left_column = self.left.resolve_column(col_name, binder)?;
        let right_column = self.right.resolve_column(col_name, binder)?;

        if left_column.is_some() && right_column.is_some() {
            return Err(ParseASTError::FailedParsing(format!("{} is ambiguous", col_name.join("."))))
        }

        Ok(left_column.or(right_column))
    }
}

impl From<JoinRef> for TableReferenceTypeImpl {
    fn from(value: JoinRef) -> Self {
        TableReferenceTypeImpl::Join(value)
    }
}

