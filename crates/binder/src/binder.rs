use std::sync::Arc;
use db_core::catalog::Catalog;
use crate::bound_table_ref::BoundTableRef;
use crate::postgres_parser::PostgresParser;
use crate::table_ref::CTEList;

pub struct Binder {
    /// Catalog will be used during the binding process
    catalog: Arc<Catalog>,

    /// The current scope for resolving column ref, used in binding expressions
    scope: Option<BoundTableRef>,

    /// The current scope for resolving tables in CTEs, used in binding tables
    cte_scope: Option<CTEList>,

    /** Sometimes we will need to assign a name to some unnamed items. This variable gives them a universal ID. */
    universal_id: isize,

    parser: PostgresParser
}

impl Binder {
    pub fn new(catalog: Arc<Catalog>) -> Self {
        Self {
            catalog,
            scope: None,
            cte_scope: None,
            universal_id: 0,
            parser: PostgresParser::default()
        }
    }
}
