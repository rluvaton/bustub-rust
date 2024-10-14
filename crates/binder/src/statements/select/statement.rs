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
use std::sync::Arc;

#[derive(Clone, Debug, PartialEq)]
pub struct SelectStatement {
    /// Bound FROM clause.
    pub(crate) table: Arc<TableReferenceTypeImpl>,

    /// Bound SELECT list.
    pub(crate) select_list: Vec<ExpressionTypeImpl>,

    /// Bound WHERE clause.
    pub(crate) where_exp: Option<ExpressionTypeImpl>,

    /// Bound GROUP BY clause.
    pub(crate) group_by: Vec<ExpressionTypeImpl>,

    /// Bound HAVING clause.
    pub(crate) having: Option<ExpressionTypeImpl>,

    /// Bound LIMIT clause.
    pub(crate) limit_count: Option<ExpressionTypeImpl>,

    /// Bound OFFSET clause.
    pub(crate) limit_offset: Option<ExpressionTypeImpl>,

    /// Bound ORDER BY clause.
    pub(crate) sort: Vec<OrderBy>,

    /// Bound CTE
    pub(crate) ctes: Arc<CTEList>,

    /// Is SELECT DISTINCT
    pub(crate) is_distinct: bool,
}

impl SelectStatement {

    pub(crate) fn builder() -> SelectStatementBuilder {
        SelectStatementBuilder::default()
    }
}

impl Statement for SelectStatement {
    type ASTStatement = sqlparser::ast::Query;

    fn try_parse_ast(ast: &Self::ASTStatement, binder: &mut Binder) -> ParseASTResult<Self>
    where
        Self: Sized,
    {
        let mut builder = Self::builder();
        let ctx_guard = binder.new_context();


        // // Bind CTEs
        let ctes = if let Some(with) = &ast.with {
            let ctes = Arc::new(SubqueryRef::parse_with(with, binder)?);

            binder.cte_scope.replace(ctes.clone());
            // TODO(chi): allow access CTE from multiple levels of scopes

            ctes
        } else {
            Arc::new(vec![])
        };

        builder = builder.with_ctes(ctes);

        builder = match &*ast.body {
            SetExpr::Select(s) => {
                s.add_select_to_select_builder(builder, binder)?
            }
            SetExpr::Values(values) => {
                // If have VALUES clause
                values
                    .add_values_to_select_builder(builder, binder)?
            }
            SetExpr::Query(_) => return Err(ParseASTError::Unimplemented("Unsupported sub query".to_string())),
            SetExpr::SetOperation { .. } => return Err(ParseASTError::Unimplemented("Set operations are not supported".to_string())),
            SetExpr::Insert(_) | SetExpr::Update(_) => return Err(ParseASTError::Unimplemented("The update/insert command is not supported inside select".to_string())),
            SetExpr::Table(_) => return Err(ParseASTError::Unimplemented("The table command is not supported".to_string()))
        };


        // LIMIT
        if let Some(limit) = &ast.limit {
            builder = builder.with_limit_count(ExpressionTypeImpl::try_parse_from_expr(limit, binder)?)
        }

        // OFFSET
        if let Some(offset) = &ast.offset {
            builder = builder.with_limit_offset(ExpressionTypeImpl::try_parse_from_expr(&offset.value, binder)?)
        }

        // ORDER BY
        if let Some(order_by) = &ast.order_by {
            builder = builder.with_sort(OrderBy::parse_from_order_by(order_by, binder)?)
        }

        builder
            .try_build()
            .map_err(|err| ParseASTError::Other(err.to_string()))
    }

    fn try_parse_from_statement(statement: &sqlparser::ast::Statement, binder: &mut Binder) -> ParseASTResult<Self> {
        match &statement {
            sqlparser::ast::Statement::Query(ast) => Self::try_parse_ast(ast, binder),
            _ => Err(ParseASTError::IncompatibleType)
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::statements::select::statement::SelectStatement;
    use crate::statements::traits::Statement;
    use crate::try_from_ast_error::ParseASTError;
    use crate::Binder;
    use sqlparser::dialect::GenericDialect;
    use sqlparser::parser::Parser;

    fn parse_select_sql(sql: &str) -> Result<Vec<SelectStatement>, ParseASTError> {
        let mut binder = Binder::default();
        let statements = Parser::parse_sql(&GenericDialect {}, sql).unwrap();
        statements.iter().map(|stmt| SelectStatement::try_parse_from_statement(stmt, &mut binder)).collect()
    }
}
