use crate::expressions::{Expression, ExpressionTypeImpl};
use crate::order_by::OrderBy;
use crate::sql_parser_helper::{ColumnDefExt};
use crate::statements::select::builder::SelectStatementBuilder;
use crate::statements::select::select_ext::SelectExt;
use crate::statements::select::values_ext::ValuesExt;
use crate::statements::traits::Statement;
use crate::table_ref::{CTEList, SubqueryRef, TableReferenceTypeImpl};
use crate::try_from_ast_error::{ParseASTError, ParseASTResult};
use crate::Binder;
use sqlparser::ast::SetExpr;
use std::fmt::Debug;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;
use std::sync::Arc;
use crate::statements::{CreateStatement, StatementTypeImpl};

#[derive(Clone, Debug, PartialEq)]
pub struct SelectStatement {
    /// Bound FROM clause.
    pub table: Rc<TableReferenceTypeImpl>,

    /// Bound SELECT list.
    pub select_list: Vec<ExpressionTypeImpl>,

    /// Bound WHERE clause.
    pub where_exp: Option<ExpressionTypeImpl>,

    /// Bound GROUP BY clause.
    pub group_by: Vec<ExpressionTypeImpl>,

    /// Bound HAVING clause.
    pub having: Option<ExpressionTypeImpl>,

    /// Bound LIMIT clause.
    pub limit_count: Option<ExpressionTypeImpl>,

    /// Bound OFFSET clause.
    pub limit_offset: Option<ExpressionTypeImpl>,

    /// Bound ORDER BY clause.
    pub sort: Vec<OrderBy>,

    /// Bound CTE
    pub ctes: Rc<CTEList>,

    /// Is SELECT DISTINCT
    pub is_distinct: bool,
}

impl SelectStatement {

    pub(crate) fn builder() -> SelectStatementBuilder {
        SelectStatementBuilder::default()
    }
}
impl Into<StatementTypeImpl> for SelectStatement {
    fn into(self) -> StatementTypeImpl {
        StatementTypeImpl::Select(self)
    }
}

impl Statement for SelectStatement {
    type ASTStatement = sqlparser::ast::Query;

    fn try_parse_ast<'a>(ast: &Self::ASTStatement, binder: &'a Binder) -> ParseASTResult<Self>
    where
        Self: Sized,
    {
        let mut builder = Self::builder();
        let mut ctx_guard = binder.new_context();


        // // Bind CTEs
        let ctes = if let Some(with) = &ast.with {
            let ctes = Rc::new(SubqueryRef::parse_with(with, ctx_guard.deref())?);

            ctx_guard.context.lock().cte_scope.replace(ctes.clone());
            // TODO(chi): allow access CTE from multiple levels of scopes

            ctes
        } else {
            Rc::new(vec![])
        };

        builder = builder.with_ctes(ctes);

        builder = match &*ast.body {
            SetExpr::Select(s) => {
                s.add_select_to_select_builder(builder, ctx_guard.deref())?
            }
            SetExpr::Values(values) => {
                // If have VALUES clause
                values
                    .add_values_to_select_builder(builder, ctx_guard.deref())?
            }
            SetExpr::Query(_) => return Err(ParseASTError::Unimplemented("Unsupported sub query".to_string())),
            SetExpr::SetOperation { .. } => return Err(ParseASTError::Unimplemented("Set operations are not supported".to_string())),
            SetExpr::Insert(_) | SetExpr::Update(_) => return Err(ParseASTError::Unimplemented("The update/insert command is not supported inside select".to_string())),
            SetExpr::Table(_) => return Err(ParseASTError::Unimplemented("The table command is not supported".to_string()))
        };


        // LIMIT
        if let Some(limit) = &ast.limit {
            builder = builder.with_limit_count(ExpressionTypeImpl::try_parse_from_expr(limit, ctx_guard.deref())?)
        }

        // OFFSET
        if let Some(offset) = &ast.offset {
            builder = builder.with_limit_offset(ExpressionTypeImpl::try_parse_from_expr(&offset.value, ctx_guard.deref())?)
        }

        // ORDER BY
        if let Some(order_by) = &ast.order_by {
            builder = builder.with_sort(OrderBy::parse_from_order_by(order_by, ctx_guard.deref())?)
        }

        builder
            .try_build()
            .map_err(|err| ParseASTError::Other(err.to_string()))
    }

    fn try_parse_from_statement(statement: &sqlparser::ast::Statement, binder: &Binder) -> ParseASTResult<Self> {
        match &statement {
            sqlparser::ast::Statement::Query(ast) => Self::try_parse_ast(ast, binder),
            _ => Err(ParseASTError::IncompatibleType)
        }
    }
}


#[cfg(test)]
mod tests {
    use parking_lot::Mutex;
    use crate::statements::select::statement::SelectStatement;
    use crate::statements::traits::Statement;
    use crate::try_from_ast_error::ParseASTError;
    use crate::Binder;
    use sqlparser::dialect::GenericDialect;
    use sqlparser::parser::Parser;
    use db_core::catalog::Catalog;

    fn parse_select_sql(sql: &str) -> Result<Vec<SelectStatement>, ParseASTError> {
        let catalog = Catalog::new(None, None, None);
        let mut binder = Binder::new(&catalog);
        let statements = Parser::parse_sql(&GenericDialect {}, sql).unwrap();
        statements.iter().map(|stmt| SelectStatement::try_parse_from_statement(stmt, &mut binder)).collect()
    }
}
