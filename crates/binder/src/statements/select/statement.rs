use crate::expressions::{ColumnRef, ExpressionTypeImpl};
use crate::order_by::OrderBy;
use crate::sql_parser_helper::{ColumnDefExt, ConstraintExt};
use crate::statements::traits::Statement;
use crate::table_ref::{CTEList, ExpressionListRef, TableReferenceTypeImpl};
use crate::try_from_ast_error::{ParseASTError, ParseASTResult};
use crate::Binder;
use std::fmt::Debug;
use std::sync::Arc;
use sqlparser::ast::SetExpr;
use crate::statements::select::builder::SelectStatementBuilder;

#[derive(Clone, Debug, PartialEq)]
pub struct SelectStatement {
    /// Bound FROM clause.
    pub(crate) table: Arc<TableReferenceTypeImpl>,

    /// Bound SELECT list.
    pub(crate) select_list: Vec<Arc<ExpressionTypeImpl>>,

    /// Bound WHERE clause.
    pub(crate) where_exp: Arc<ExpressionTypeImpl>,

    /// Bound GROUP BY clause.
    pub(crate) group_by: Vec<Arc<ExpressionTypeImpl>>,

    /// Bound HAVING clause.
    pub(crate) having: Arc<ExpressionTypeImpl>,

    /// Bound LIMIT clause.
    pub(crate) limit_count: Arc<ExpressionTypeImpl>,

    /// Bound OFFSET clause.
    pub(crate) limit_offset: Arc<ExpressionTypeImpl>,

    /// Bound ORDER BY clause.
    pub(crate) sort: Vec<Arc<OrderBy>>,

    /// Bound CTE
    pub(crate) ctes: CTEList,

    /// Is SELECT DISTINCT
    pub(crate) is_distinct: bool,
}

impl SelectStatement {
    pub fn new(
        table: Arc<TableReferenceTypeImpl>,
        select_list: Vec<Arc<ExpressionTypeImpl>>,
        where_exp: Arc<ExpressionTypeImpl>,
        group_by: Vec<Arc<ExpressionTypeImpl>>,
        having: Arc<ExpressionTypeImpl>,
        limit_count: Arc<ExpressionTypeImpl>,
        limit_offset: Arc<ExpressionTypeImpl>,
        sort: Vec<Arc<OrderBy>>,
        ctes: CTEList,
        is_distinct: bool,
    ) -> Self {
        Self {
            table,
            select_list,
            where_exp,
            group_by,
            having,
            limit_count,
            limit_offset,
            sort,
            ctes,
            is_distinct,
        }
    }

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
        let ctx_guard = binder.new_context();

        match &*ast.body {
            SetExpr::Select(_) => {}
            SetExpr::Query(_) => {}
            SetExpr::SetOperation { .. } => {}
            SetExpr::Values(values) => {
                // If have VALUES clause
                let values_list_name = format!("__values#{}", binder.universal_id);
                binder.universal_id += 1;

                let value_list: ExpressionListRef = ExpressionListRef::try_parse_from_values(Some(values_list_name.clone()), values, binder)?;

                let exprs: Vec<Arc<ExpressionTypeImpl>> = value_list
                    .values[0]
                    .iter()
                    .enumerate()
                    .map(|(i, _)| {
                        let col = ColumnRef::new(vec![values_list_name.clone(), i.to_string()]);

                        Arc::new(col.into())
                    })
                    .collect();

                return Self::builder()
                    .with_table(Arc::new(value_list.into()))
                    .with_select_list(exprs)
                    .try_build()
                    .map_err(|err| ParseASTError::Other(err.to_string()));
            }
            SetExpr::Insert(_) => {}
            SetExpr::Update(_) => {}
            SetExpr::Table(_) => {}
        }
        // if !stmt.values_lists.is_empty() {
        //     let values_list_name = format!("__values#{}", self.universal_id);
        //     self.universal_id += 1;
        //
        //     let mut value_list: ExpressionListRef = ExpressionListRef::try_from_nodes(&stmt.values_lists)?;
        //
        //     value_list.identifier = values_list_name;
        //
        //     let mut exprs: Vec<Arc<dyn Expression>> = vec![];
        //
        //     for i in 0..value_list.values[0].len() {
        //         exprs.push(Arc::new(ColumnRef::new(vec![
        //             values_list_name,
        //             i.to_string()
        //         ])));
        //     }
        //
        //     return Ok(SelectStatement {
        //         table: Arc::new(value_list),
        //         select_list: exprs,
        //         where_exp: Arc::new(()),
        //         group_by: vec![],
        //         having: Arc::new(()),
        //         limit_count: Arc::new(()),
        //         limit_offset: Arc::new(()),
        //         sort: vec![],
        //         ctes: vec![],
        //         is_distinct: false,
        //     })
        // }
        //
        // // Bind CTEs
        // let mut ctes = vec![];
        // if let Some(with) = &stmt.with_clause {
        //     ctes = self.convert_with_to_many_subqueries(with)?;
        //     self.cte_scope = Some(ctes);
        //     // TODO(chi): allow access CTE from multiple levels of scopes
        // }

        // Bind FROM clause
        // let table = stmt.from_clause

        todo!()
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
