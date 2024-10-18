use std::fmt::{Display, Formatter};
use std::sync::Arc;
use std::thread::Scope;
use crate::expressions::{Expression, ExpressionTypeImpl};
use crate::try_from_ast_error::{ParseASTError, ParseASTResult};
use crate::Binder;
use sqlparser::ast::Expr;
use catalog_schema::Schema;
use crate::table_ref::{TableRef, TableReferenceTypeImpl};

/// A bound column reference, e.g., `y.x` in the SELECT list.
#[derive(Clone, Debug, PartialEq)]
pub struct ColumnRef {
    pub col_name: Vec<String>
}

impl ColumnRef {
    pub(crate) fn new(col_name: Vec<String>) -> Self {
        Self {
            col_name
        }
    }

    pub fn prepend(&mut self, prefix: String) -> Self {
        let mut new = self.clone();
        new.col_name.insert(0, prefix);

        new
    }

    pub fn resolve_from_schema(column_names: &[String], schema: Arc<Schema>) -> ParseASTResult<Option<Self>> {
        if column_names.len() != 1 {
            return Ok(None);
        }

        let column_name = column_names[0].clone().to_lowercase();

        let matching_columns = schema.get_columns()
            .iter()
            .filter(|&col| col.get_name().to_lowercase() == column_name)
            .collect::<Vec<_>>();

        match matching_columns.len() {
            0 => Ok(None),
            1 => Ok(Some(ColumnRef::new(vec![matching_columns[0].get_name().clone()]))),
            _ => Err(ParseASTError::FailedParsing(format!("{} is ambiguous in schema", column_name)))
        }
    }

    pub fn resolve_from_select_list(subquery_select_list: &Vec<Vec<String>>, col_name: &[String]) -> ParseASTResult<Option<Self>> {
        let matching_columns = subquery_select_list
            .iter()
            .filter(|&col| col.iter().map(|item| item.to_lowercase()).collect::<Vec<String>>().starts_with(col_name))
            .collect::<Vec<_>>();

        match matching_columns.len() {
            0 => Ok(None),
            1 => Ok(Some(ColumnRef::new(matching_columns[0].clone()))),
            _ => Err(ParseASTError::FailedParsing(format!("{} is ambiguous in subquery select list", col_name.join("."))))
        }
    }
}

impl Into<ExpressionTypeImpl> for ColumnRef {
    fn into(self) -> ExpressionTypeImpl {
        ExpressionTypeImpl::ColumnRef(self)
    }
}

impl Expression for ColumnRef {
    fn has_aggregation(&self) -> bool {
        false
    }

    fn try_parse_from_expr(expr: &Expr, binder: &Binder) -> ParseASTResult<Self>
    where
        Self: Sized
    {

        let col_name = match expr {
            Expr::Identifier(ident) => {
                vec![ident.value.clone()]
            }
            Expr::CompoundIdentifier(ident) => {
                if ident.is_empty() {
                    return Err(ParseASTError::FailedParsing("Identifier cannot be empty vector".to_string()));
                }

                ident.iter().map(|item| item.value.clone()).collect()
            }
            _ => return Err(ParseASTError::IncompatibleType)
        };

        if let Some(scope) = &binder.context.lock().scope {
            let column = scope.resolve_column(col_name.as_slice(), binder)?;

            if let Some(column) = column {
                Ok(column)
            } else {
                Err(ParseASTError::FailedParsing(format!("missing column {}", col_name.join("."))))
            }
        } else {
            Err(ParseASTError::FailedParsing("no scope".to_string()))
        }
    }
}

impl Display for ColumnRef {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.col_name.join("."))
    }
}
