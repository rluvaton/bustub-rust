use crate::binder::context_guard::ContextGuard;
use crate::binder::Context;
use crate::statements::{Statement, StatementTypeImpl};
use crate::try_from_ast_error::ParseASTResult;
use db_core::catalog::Catalog;
use parking_lot::Mutex;
use sqlparser::parser::Parser;
use std::sync::atomic::{AtomicUsize, Ordering};

pub struct Binder<'a> {
    /// Catalog will be used during the binding process
    pub(crate) catalog: &'a Catalog,

    pub(crate) context: Mutex<Context>,

    /** Sometimes we will need to assign a name to some unnamed items. This variable gives them a universal ID. */
    pub(crate) universal_id: AtomicUsize,
}

impl<'a> Binder<'a> {
    pub fn new(catalog: &'a Catalog) -> Self {
        Self {
            catalog,
            context: Mutex::new(Context {
                scope: None,
                cte_scope: None,
            }),
            universal_id: AtomicUsize::new(0),
        }
    }


    pub(crate) fn new_context(&'a self) -> ContextGuard<'a> {
        ContextGuard::new(self)
    }

    pub fn parse(mut self, sql: &str) -> ParseASTResult<Vec<StatementTypeImpl>> {
        let statements = Parser::parse_sql(&sqlparser::dialect::GenericDialect {}, sql).unwrap();
        statements.iter().map(|stmt| StatementTypeImpl::try_parse_from_statement(stmt, &mut self)).collect()
    }

    pub fn get_and_increment_universal_id(&self) -> usize {
        self.universal_id.fetch_add(1, Ordering::SeqCst)
    }

    // pub fn bind_statement()
}

// impl Default for Binder<'_> {
//     fn default() -> Self {
//         Self {
//             catalog: MutexGuard::new(Catalog::default()),
//             scope: None,
//             cte_scope: None,
//             universal_id: 0,
//         }
//     }
// }
