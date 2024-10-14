use crate::expressions::ExpressionTypeImpl;
use crate::order_by::OrderBy;
use crate::statements::SelectStatement;
use crate::table_ref::{CTEList, TableReferenceTypeImpl};
use anyhow::anyhow;
use std::fmt::Debug;
use std::sync::Arc;

#[derive(Debug, PartialEq)]
pub(super) struct SelectStatementBuilder {
    /// Bound FROM clause.
    table: Option<Arc<TableReferenceTypeImpl>>,

    /// Bound SELECT list.
    select_list: Option<Vec<Arc<ExpressionTypeImpl>>>,

    /// Bound WHERE clause.
    where_exp: Arc<ExpressionTypeImpl>,

    /// Bound GROUP BY clause.
    group_by: Vec<Arc<ExpressionTypeImpl>>,

    /// Bound HAVING clause.
    having: Arc<ExpressionTypeImpl>,

    /// Bound LIMIT clause.
    limit_count: Arc<ExpressionTypeImpl>,

    /// Bound OFFSET clause.
    limit_offset: Arc<ExpressionTypeImpl>,

    /// Bound ORDER BY clause.
    sort: Vec<Arc<OrderBy>>,

    /// Bound CTE
    ctes: CTEList,

    /// Is SELECT DISTINCT
    is_distinct: bool,
}


impl SelectStatementBuilder {

    pub(super) fn with_table(mut self, table: Arc<TableReferenceTypeImpl>) -> Self {
        self.table = Some(table);

        self
    }

    pub(super) fn with_select_list(mut self, select_list: Vec<Arc<ExpressionTypeImpl>>) -> Self {
        self.select_list = Some(select_list);

        self
    }

    pub(super) fn with_where_exp(mut self, where_exp: Arc<ExpressionTypeImpl>) -> Self {
        self.where_exp = where_exp;

        self
    }

    pub(super) fn with_group_by(mut self, group_by: Vec<Arc<ExpressionTypeImpl>>) -> Self {
        self.group_by = group_by;

        self
    }

    pub(super) fn with_having(mut self, having: Arc<ExpressionTypeImpl>) -> Self {
        self.having = having;

        self
    }

    pub(super) fn with_limit_count(mut self, limit_count: Arc<ExpressionTypeImpl>) -> Self {
        self.limit_count = limit_count;

        self
    }

    pub(super) fn with_limit_offset(mut self, limit_offset: Arc<ExpressionTypeImpl>) -> Self {
        self.limit_offset = limit_offset;

        self
    }

    pub(super) fn with_sort(mut self, sort: Vec<Arc<OrderBy>>) -> Self {
        self.sort = sort;

        self
    }

    pub(super) fn with_ctes(mut self, ctes: CTEList) -> Self {
        self.ctes = ctes;

        self
    }

    pub(super) fn with_is_distinct(mut self, is_distinct: bool) -> Self {
        self.is_distinct = is_distinct;

        self
    }

    pub(super) fn try_build(self) -> error_utils::anyhow::Result<SelectStatement> {
        if self.table.is_none() {
            return Err(error_utils::Error::<anyhow::Error>::new_anyhow(anyhow!("table must be defined")));
        }

        let table = self.table.unwrap();

        if self.select_list.is_none() {
            return Err(error_utils::Error::<anyhow::Error>::new_anyhow(anyhow!("select list must be defined")));
        }

        let select_list = self.select_list.unwrap();

        Ok(SelectStatement {
            table,
            select_list,
            where_exp: self.where_exp,
            group_by: self.group_by,
            having: self.having,
            limit_count: self.limit_count,
            limit_offset: self.limit_offset,
            sort: self.sort,
            ctes: self.ctes,
            is_distinct: self.is_distinct,
        })
    }
}

impl Default for SelectStatementBuilder {
    fn default() -> Self {
        Self {
            table: None,
            select_list: None,

            where_exp: Arc::new(ExpressionTypeImpl::Invalid),
            group_by: vec![],
            having: Arc::new(ExpressionTypeImpl::Invalid),
            limit_count: Arc::new(ExpressionTypeImpl::Invalid),
            limit_offset: Arc::new(ExpressionTypeImpl::Invalid),
            sort: vec![],
            ctes: vec![],
            is_distinct: false,
        }
    }
}
