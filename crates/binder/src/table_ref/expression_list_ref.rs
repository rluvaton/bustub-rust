use crate::expressions::{ColumnRef, ExpressionTypeImpl};
use crate::table_ref::{TableRef, TableReferenceTypeImpl};
use crate::try_from_ast_error::{ParseASTError, ParseASTResult};
use crate::Binder;
use sqlparser::ast::{TableFactor, Values};

/// A bound table ref type for `values` clause.
#[derive(Clone, Debug, PartialEq)]
pub struct ExpressionListRef {
    pub identifier: String,
    pub values: Vec<Vec<ExpressionTypeImpl>>,
}

impl ExpressionListRef {
    pub fn new(identifier: Option<String>, values: Vec<Vec<ExpressionTypeImpl>>) -> Self {
        Self {
            identifier: identifier.unwrap_or("<unnamed>".to_string()),
            values,
        }
    }


    pub fn try_parse_from_values(identifier: Option<String>, ast: &Values, binder: &Binder) -> ParseASTResult<ExpressionListRef> {
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
    fn resolve_column(&self, col_name: &[String], _binder: &Binder) -> ParseASTResult<Option<ColumnRef>> {
        Err(ParseASTError::FailedParsing(format!("cannot resolve column {} in VALUES", col_name.join("."))))
    }

    fn get_all_columns(&self, _binder: &Binder) -> ParseASTResult<Vec<ExpressionTypeImpl>> {
        // TODO - remove the select * from error message
        Err(ParseASTError::FailedParsing("select * cannot be used with values".to_string()))
    }

    fn try_from_ast(_ast: &TableFactor, _binder: &Binder) -> ParseASTResult<Self> {
        // No table factor matching the expression list
        Err(ParseASTError::IncompatibleType)
    }
}

impl From<ExpressionListRef> for TableReferenceTypeImpl {
    fn from(value: ExpressionListRef) -> Self {
        TableReferenceTypeImpl::ExpressionList(value)
    }
}

