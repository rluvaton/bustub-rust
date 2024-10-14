use crate::expressions::{Expression, ExpressionTypeImpl};
use crate::try_from_ast_error::{ParseASTError, ParseASTResult};
use crate::Binder;
use sqlparser::ast::Expr;

/// A bound column reference, e.g., `y.x` in the SELECT list.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct ColumnRef {
    pub(crate) col_name: Vec<String>
}

impl ColumnRef {

    pub(crate) fn new(col_name: Vec<String>) -> Self {
        Self {
            col_name
        }
    }

    pub fn prepend(&mut self, prefix: String) {
        self.col_name.insert(0, prefix);
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

    fn try_parse_from_expr(expr: &Expr, _binder: &mut Binder) -> ParseASTResult<Self>
    where
        Self: Sized
    {
        match expr {
            Expr::Identifier(ident) => {
                Ok(Self {
                    col_name: vec![ident.value.clone()]
                })
            }
            Expr::CompoundIdentifier(ident) => {
                Ok(Self {
                    col_name: ident.iter().map(|item| item.value.clone()).collect()
                })
            }
            _ => Err(ParseASTError::IncompatibleType)
        }
    }
}
