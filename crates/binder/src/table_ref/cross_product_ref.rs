use sqlparser::ast::{TableFactor, TableWithJoins};
use crate::expressions::ColumnRef;
use crate::table_ref::table_reference_type::TableReferenceTypeImpl;
use crate::table_ref::TableRef;
use crate::try_from_ast_error::{ParseASTError, ParseASTResult};
use crate::Binder;

#[derive(Clone, Debug, PartialEq)]
pub struct CrossProductRef {

    /** The left side of the cross product. */
    pub left: Box<TableReferenceTypeImpl>,

    /** The right side of the cross product. */
    pub right: Box<TableReferenceTypeImpl>,
}

impl CrossProductRef {
    pub(crate) fn new(left: Box<TableReferenceTypeImpl>, right: Box<TableReferenceTypeImpl>) -> Self {
        Self {
            left,
            right
        }
    }

    pub(crate) fn try_to_parse_tables_with_joins(tables: &[TableWithJoins], binder: &Binder) -> ParseASTResult<TableReferenceTypeImpl> {
        // Bind cross join.

        // Extract the first node
        let (left_node, tables) = tables.split_first().unwrap();

        let left_node = TableReferenceTypeImpl::parse_from_table_with_join(left_node, binder)?;

        tables
            .iter()
            .map(|table_to_join| TableReferenceTypeImpl::parse_from_table_with_join(table_to_join, binder))

            // Extract the remaining nodes.
            //
            // The result bound tree will be like:
            //  ```
            //      O
            //     / \
            //    O   3
            //   / \
            //  1   2
            //  ```
            .try_fold(
                left_node,
                |cross_product_ref, table_to_join| {
                    Ok(CrossProductRef::new(
                        Box::new(cross_product_ref),
                        Box::new(table_to_join?),
                    ).into())
                })
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

    fn try_from_ast(_ast: &TableFactor, _binder: &Binder) -> ParseASTResult<Self> {
        // Always incompatible as we need to have table with joins
        Err(ParseASTError::IncompatibleType)
    }
}

