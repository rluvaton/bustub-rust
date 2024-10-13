use std::sync::Arc;
use db_core::catalog::Catalog;
use crate::bound_table_ref::BoundTableRef;
use crate::table_ref::CTEList;




pub struct Binder<'a> {
    /// Catalog will be used during the binding process
    // catalog: Option<&'a Catalog>,

    /// The current scope for resolving column ref, used in binding expressions
    scope: Option<BoundTableRef>,

    /// The current scope for resolving tables in CTEs, used in binding tables
    cte_scope: Option<CTEList>,

    /** Sometimes we will need to assign a name to some unnamed items. This variable gives them a universal ID. */
    universal_id: isize,

    statement_nodes: Vec<pg_query::NodeRef<'a>>
}

impl<'a> Binder<'a> {
    pub fn new() -> Self {
        Self {
            // catalog: Some(catalog),
            scope: None,
            cte_scope: None,
            universal_id: 0,
            statement_nodes: vec![],
        }
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
