use sqlparser::ast::TableFactor;
use crate::expressions::ColumnRef;
use crate::table_ref::table_reference_type::TableReferenceTypeImpl;
use crate::table_ref::TableRef;
use crate::try_from_ast_error::{ParseASTError, ParseASTResult};
use crate::Binder;

#[derive(Clone, Debug, PartialEq)]
pub struct CTERef {
    pub(crate) cte_name: String,
    pub(crate) alias: String,
}

impl CTERef {
    pub(crate) fn new(cte_name: String, alias: String) -> Self {
        Self {
            cte_name,
            alias,
        }
    }
}

impl TableRef for CTERef {
    fn resolve_column(&self, col_name: &[String], binder: &Binder) -> ParseASTResult<Option<ColumnRef>> {

        // TODO - should move the CTE scope in this ref?
        let cte_scope = binder.cte_scope.as_ref().expect("CTE not found").clone();

        for cte in cte_scope.iter() {
            if cte.alias == self.alias {
                // TODO(chi): disallow multiple CTE of the same alias
                return cte.resolve_column(col_name, binder)
            }
        }

        unreachable!("CTE not found")
    }

    fn try_from_ast(ast: &TableFactor, binder: &mut Binder) -> ParseASTResult<Self> {
        // CTE is manually done
        Err(ParseASTError::IncompatibleType)
    }
}

impl From<CTERef> for TableReferenceTypeImpl {
    fn from(value: CTERef) -> Self {
        TableReferenceTypeImpl::CTE(value)
    }
}
