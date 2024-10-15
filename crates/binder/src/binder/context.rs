use std::rc::Rc;
use crate::table_ref::{CTEList, TableReferenceTypeImpl};

#[derive(Clone)]
pub(crate) struct Context {

    /// The current scope for resolving column ref, used in binding expressions
    pub(crate) scope: Option<Rc<TableReferenceTypeImpl>>,

    /// The current scope for resolving tables in CTEs, used in binding tables
    pub(crate) cte_scope: Option<Rc<CTEList>>,
}
