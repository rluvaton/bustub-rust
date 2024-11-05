use sqlparser::ast::TableFactor;
use crate::expressions::ColumnRef;
use crate::table_ref::table_reference_type::TableReferenceTypeImpl;
use crate::table_ref::TableRef;
use crate::try_from_ast_error::{ParseASTError, ParseASTResult};
use crate::{Binder, ExpressionTypeImpl};

#[derive(Clone, Debug, PartialEq)]
pub struct CTERef {
    pub cte_name: String,
    pub alias: String,
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
        let cte_scope = binder.context.lock().cte_scope.as_ref().expect("CTE not found").clone();

        for cte in cte_scope.iter() {
            if cte.alias == self.alias {
                // TODO(chi): disallow multiple CTE of the same alias
                return cte.resolve_column(col_name, binder)
            }
        }

        unreachable!("CTE not found")
    }

    fn get_all_columns(&self, binder: &Binder) -> ParseASTResult<Vec<ExpressionTypeImpl>> {
        let cte_scope = {
            let cte_scope_guard = binder.context.lock();

            cte_scope_guard.cte_scope
                .as_ref()
                .expect("CTE not found")
                .clone()
        };

        let cte = cte_scope
            .iter()
            // TODO(chi): disallow multiple CTE of the same alias
            .find(|cte| cte.alias == self.alias)
            .expect("CTE not found");

        let mut columns = cte.get_all_columns(binder)?;

        for mut column in &mut columns {
            match &mut column {
                ExpressionTypeImpl::ColumnRef(column_ref) => {
                    // TODO - is the [0] correct or we should insert and shift everything?
                    column_ref.col_name[0] = self.alias.clone();
                }
                _ => return Err(ParseASTError::FailedParsing(format!("Column type from CTE must be column ref {:?}", &column)))
            }
        }

        Ok(columns)
    }

    fn try_from_ast(_ast: &TableFactor, _binder: &Binder) -> ParseASTResult<Self> {
        // CTE is manually done
        Err(ParseASTError::IncompatibleType)
    }
}

impl From<CTERef> for TableReferenceTypeImpl {
    fn from(value: CTERef) -> Self {
        TableReferenceTypeImpl::CTE(value)
    }
}
