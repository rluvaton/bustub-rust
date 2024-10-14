use crate::expressions::{ColumnRef, ExpressionTypeImpl};
use crate::statements::{SelectStatement, Statement};
use crate::table_ref::{TableRef, TableReferenceTypeImpl};
use crate::try_from_ast_error::{ParseASTError, ParseASTResult};
use crate::Binder;
use sqlparser::ast::{Cte, With};

#[derive(Clone, Debug, PartialEq)]
pub struct SubqueryRef {
    /// Subquery.
    pub(crate) subquery: SelectStatement,

    /// Name of each item in the select list.
    pub(crate) select_list_name: Vec<Vec<String>>,

    // Alias
    pub(crate) alias: String,
}

impl SubqueryRef {
    pub(crate) fn parse_with(with: &With, binder: &mut Binder) -> ParseASTResult<Vec<Self>> {
        if with.recursive {
            return Err(ParseASTError::Unimplemented("recursive CTE is not supported".to_string()));
        }

        with.cte_tables
            .iter()
            .map(|cte_ele| Self::parse_cte(cte_ele, binder))
            .collect()
    }

    pub(crate) fn parse_cte(cte: &Cte, binder: &mut Binder) -> ParseASTResult<Self> {
        let subquery = SelectStatement::try_parse_ast(&*cte.query, binder)?;

        let select_list_name: Vec<Vec<String>> = subquery.select_list
            .iter()
            .map(|col| {
                match &*col.clone() {
                    ExpressionTypeImpl::ColumnRef(c) => c.col_name.clone(),
                    ExpressionTypeImpl::Alias(a) => vec![a.alias.clone()],
                    _ => {
                        let name = format!("__item#{}", binder.universal_id);

                        binder.universal_id += 1;

                        vec![name]
                    }
                }
            })
            .collect::<Vec<_>>();

        Ok(SubqueryRef {
            subquery,
            select_list_name,
            alias: cte.alias.name.value.clone()
        })

    }
}

impl Into<TableReferenceTypeImpl> for SubqueryRef {
    fn into(self) -> TableReferenceTypeImpl {
        TableReferenceTypeImpl::SubQuery(self)
    }
}

impl TableRef for SubqueryRef {
    fn resolve_column(&self, col_name: &[String], binder: &Binder) -> ParseASTResult<Option<ColumnRef>> {
        let alias = &self.alias;

        // Firstly, try directly resolve the column name through schema
        let direct_resolved_expr = ColumnRef::resolve_from_select_list(&self.select_list_name, col_name)?.map(|mut c| c.prepend(alias.clone()));

        let mut strip_resolved_expr: Option<ColumnRef> = None;

        // Then, try strip the prefix and match again
        if col_name.len() > 1 {
            // Strip alias and resolve again
            if &col_name[0] == alias {
                let strip_column_name = &col_name[1..];

                strip_resolved_expr = ColumnRef::resolve_from_select_list(&self.select_list_name, strip_column_name)?.map(|mut c| c.prepend(alias.clone()));
            }
        }

        if strip_resolved_expr.is_some() && direct_resolved_expr.is_some() {
            return Err(ParseASTError::FailedParsing(format!("{} is ambiguous in subquery {}", col_name.join("."), alias)))
        }

        Ok(strip_resolved_expr.or(direct_resolved_expr))
    }
}


impl Binder<'_> {
    pub(crate) fn parse_subquery(&mut self, select_stmt: &sqlparser::ast::Select, alias: String) -> Result<SubqueryRef, ParseASTError> {
        todo!()
        // let subquery = self.parse_select_statement(select_stmt)?;
        // let mut select_list_name: Vec<Vec<String>> = vec![];
        //
        // for col in &subquery.select_list {
        //     match col {
        //         ExpressionTypeImpl::ColumnRef(col) => {
        //             select_list_name.push(col.col_name.clone());
        //         }
        //         ExpressionTypeImpl::Alias(alias) => {
        //             select_list_name.push(vec![alias.alias]);
        //         }
        //          _ => {
        //              select_list_name.push(vec![format!("__item#{}", self.universal_id)]);
        //              self.universal_id += 1;
        //          }
        //     }
        // }
        //
        // Ok(SubqueryRef {
        //     subquery,
        //     select_list_name,
        //     alias
        // })
    }

    pub(crate) fn convert_with_to_many_subqueries(&mut self, with: &sqlparser::ast::With) -> Result<Vec<SubqueryRef>, ParseASTError> {
        todo!()
        // let mut ctes: Vec<SubqueryRef> = vec![];
        //
        // for item in with.ctes {
        //     let item = item.node;
        //
        //     if item.is_none() {
        //         continue;
        //     }
        //
        //     let item = item.unwrap();
        //     let cte = match item {
        //         Node::CommonTableExpr(cte) => cte,
        //         _ => return Err(TryFromASTError::FailedParsing("Item in with clause must be common table expression".to_string()))
        //     };
        //
        //     if cte.ctequery.is_none() {
        //         return Err(TryFromASTError::FailedParsing("SELECT not found".to_string()));
        //     }
        //
        //     let ctequery = cte.ctequery.as_ref().unwrap();
        //
        //     if ctequery.node.is_none() {
        //         return Err(TryFromASTError::FailedParsing("SELECT not found".to_string()));
        //     }
        //
        //     let ctequery = ctequery.node.unwrap();
        //
        //     let select_stmt = match ctequery {
        //         Node::SelectStmt(stmt) => stmt,
        //         _ => return Err(TryFromASTError::FailedParsing("SELECT not found".to_string())),
        //     };
        //
        //     if cte.cterecursive {
        //         return Err(TryFromASTError::Unimplemented("Recursive CTE not supported".to_string()));
        //     }
        //
        //     let subquery = self.create_subquery_ref_from_select_and_alias(&select_stmt, cte.ctename)?;
        //
        //     ctes.push(subquery);
        // }
        //
        // Ok(ctes)
    }
}

pub type CTEList = Vec<SubqueryRef>;
