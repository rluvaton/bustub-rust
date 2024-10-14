use crate::Binder;
use crate::expressions::ColumnRef;
use crate::table_ref::table_reference_type::{TableReferenceType, TableReferenceTypeImpl};
use crate::table_ref::TableRef;
use crate::try_from_ast_error::{ParseASTError, ParseASTResult};

#[derive(Clone, Debug, PartialEq)]
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

impl Into<TableReferenceTypeImpl> for CrossProductRef {
    fn into(self) -> TableReferenceTypeImpl {
        TableReferenceTypeImpl::CrossProduct(self)
    }
}

impl TableRef for CrossProductRef {
    fn resolve_column(&self, col_name: &[String], binder: &Binder) -> ParseASTResult<Option<ColumnRef>> {
        let left_column = self.left.resolve_column(col_name, binder)?;
        let right_column = self.right.resolve_column(col_name, binder)?;

        if left_column.is_some() && right_column.is_some() {
            return Err(ParseASTError::FailedParsing(format!("{} is ambiguous", col_name.join("."))))
        }

        Ok(left_column.or(right_column))
    }
}

