use std::sync::Arc;
use db_core::catalog::Catalog;
use crate::context_guard::ContextGuard;
use crate::statements::{StatementType, StatementTypeImpl};
use crate::table_ref::{CTEList, TableRef, TableReferenceTypeImpl};




pub struct Binder<'a> {
    /// Catalog will be used during the binding process
    pub(crate) catalog: Option<&'a Catalog>,

    /// The current scope for resolving column ref, used in binding expressions
    pub(crate) scope: Option<Arc<TableReferenceTypeImpl>>,

    /// The current scope for resolving tables in CTEs, used in binding tables
    pub(crate) cte_scope: Option<Arc<CTEList>>,

    /** Sometimes we will need to assign a name to some unnamed items. This variable gives them a universal ID. */
    pub(crate) universal_id: isize,

    pub(crate) statement_nodes: Vec<StatementTypeImpl>
}

impl<'a> Binder<'a> {
    pub fn new(catalog: &'a Catalog) -> Self {
        Self {
            catalog: Some(catalog),
            scope: None,
            cte_scope: None,
            universal_id: 0,
            statement_nodes: vec![],
        }
    }


    pub(crate) fn new_context(&self) -> ContextGuard {
        todo!()
        // ContextGuard::new(self.scope, self.cte_scope)
    }

    // pub fn parse_and_save(&mut self, catalog: &Catalog, sql: &str) -> error_utils::anyhow::Result<bool> {
    //     let parsed = pg_query::parse(sql).map_err(|e| error_utils::anyhow::anyhow!(e))?;
    //
    //     if parsed.is_empty() {
    //         println!("Parser received empty statement");
    //
    //         return Ok(false)
    //     }
    //
    //     self.statement_nodes = parsed;
    //
    //     return Ok(true)
    // }

    // pub fn bind_statement()
}

impl Default for Binder<'_> {
    fn default() -> Self {
        Self {
            catalog: None,
            scope: None,
            cte_scope: None,
            universal_id: 0,
            statement_nodes: vec![],
        }
    }
}
