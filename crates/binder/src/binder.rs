use std::rc::Rc;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use parking_lot::{Mutex, MutexGuard};
use sqlparser::parser::Parser;
use db_core::catalog::Catalog;
use crate::context_guard::ContextGuard;
use crate::statements::{Statement, StatementType, StatementTypeImpl};
use crate::table_ref::{CTEList, TableRef, TableReferenceTypeImpl};
use crate::try_from_ast_error::ParseASTResult;

pub struct Binder<'a> {
    /// Catalog will be used during the binding process
    pub(crate) catalog: MutexGuard<'a, Catalog>,

    pub(crate) context: Mutex<Context>,

    /** Sometimes we will need to assign a name to some unnamed items. This variable gives them a universal ID. */
    pub(crate) universal_id: AtomicUsize,
}

#[derive(Clone)]
pub(crate) struct Context {

    /// The current scope for resolving column ref, used in binding expressions
    pub(crate) scope: Option<Rc<TableReferenceTypeImpl>>,

    /// The current scope for resolving tables in CTEs, used in binding tables
    pub(crate) cte_scope: Option<Rc<CTEList>>,
}

impl<'a> Binder<'a> {
    pub fn new(catalog: MutexGuard<'a, Catalog>) -> Self {
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
