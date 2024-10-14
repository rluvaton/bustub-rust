use crate::expressions::ExpressionTypeImpl;
use crate::table_ref::{TableRef, TableReferenceTypeImpl};
use crate::try_from_ast_error::{ParseASTError, ParseASTResult};
use crate::Binder;
use sqlparser::ast::Values;

/// A bound table ref type for `values` clause.
#[derive(Debug, PartialEq)]
pub struct ExpressionListRef {
    pub(crate) identifier: String,
    pub(crate) values: Vec<Vec<ExpressionTypeImpl>>,
}

impl ExpressionListRef {
    pub fn new(identifier: Option<String>, values: Vec<Vec<ExpressionTypeImpl>>) -> Self {
        Self {
            identifier: identifier.unwrap_or("<unnamed>".to_string()),
            values,
        }
    }


    pub fn try_parse_from_values(identifier: Option<String>, ast: &Values, binder: &mut Binder) -> ParseASTResult<ExpressionListRef> {
        let parsed_values : ParseASTResult<Vec<Vec<ExpressionTypeImpl>>> = ast.rows.iter().map(|row| ExpressionTypeImpl::parse_expression_list(row, binder)).collect();

        let parsed_values = parsed_values?;

        if parsed_values.is_empty() {
            return Err(ParseASTError::FailedParsing("At least one row of values should be provided".to_string()))
        }

        let row_size = parsed_values[0].len();

        let has_different_row_length = parsed_values.iter().any(|row| row.len() != row_size);

        if has_different_row_length {
            return Err(ParseASTError::FailedParsing("values must have the same length".to_string()));
        }

        Ok(ExpressionListRef::new(identifier, parsed_values))
    }
}

impl TableRef for ExpressionListRef {
}

impl From<ExpressionListRef> for TableReferenceTypeImpl {
    fn from(value: ExpressionListRef) -> Self {
        TableReferenceTypeImpl::ExpressionList(value)
    }
}

// impl TryFrom<NodeRef<'_>> for ExpressionListRef {
//     type Error = TryFromASTError;
//
//     fn try_from(value: NodeRef) -> Result<Self, Self::Error> {
//         let list = match value {
//             NodeRef::List(list) => list,
//             _ => return Err(TryFromASTError::IncompatibleType),
//         };
//         let nodes: &Vec<pg_query::Node> = &list.items;
//
//         ExpressionListRef::try_from_nodes(nodes)
//     }
// }

