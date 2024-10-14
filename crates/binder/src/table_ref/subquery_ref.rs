use std::ops::Deref;
use std::sync::Arc;
use pg_query::protobuf::node::Node;
use pg_query::protobuf::{SelectStmt, WithClause};
use crate::Binder;
use crate::expressions::{ExpressionType, ExpressionTypeImpl};
use crate::parse_node_error::ParsePgNodeError;
use crate::statements::SelectStatement;
use crate::table_ref::table_reference_type::TableReferenceType;
use crate::table_ref::TableRef;

#[derive(Debug, PartialEq)]
pub struct SubqueryRef {
    /// Subquery.
    pub(crate) subquery: SelectStatement,

    /// Name of each item in the select list.
    pub(crate) select_list_name: Vec<Vec<String>>,

    // Alias
    pub(crate) alias: String,
}

impl TableRef for SubqueryRef {
    const TYPE: TableReferenceType = TableReferenceType::SubQuery;
}


impl Binder {
    pub(crate) fn parse_subquery(&mut self, select_stmt: &SelectStmt, alias: String) -> Result<SubqueryRef, ParsePgNodeError> {
        let subquery = self.parse_select_statement(select_stmt)?;
        let mut select_list_name: Vec<Vec<String>> = vec![];

        for col in &subquery.select_list {
            match col {
                ExpressionTypeImpl::ColumnRef(col) => {
                    select_list_name.push(col.col_name.clone());
                }
                ExpressionTypeImpl::Alias(alias) => {
                    select_list_name.push(vec![alias.alias]);
                }
                 _ => {
                     select_list_name.push(vec![format!("__item#{}", self.universal_id)]);
                     self.universal_id += 1;
                 }
            }
        }

        Ok(SubqueryRef {
            subquery,
            select_list_name,
            alias
        })
    }

    pub(crate) fn convert_with_to_many_subqueries(&mut self, with: &WithClause) -> Result<Vec<SubqueryRef>, ParsePgNodeError> {
        let mut ctes: Vec<SubqueryRef> = vec![];

        for item in with.ctes {
            let item = item.node;

            if item.is_none() {
                continue;
            }

            let item = item.unwrap();
            let cte = match item {
                Node::CommonTableExpr(cte) => cte,
                _ => return Err(ParsePgNodeError::FailedParsing("Item in with clause must be common table expression".to_string()))
            };

            if cte.ctequery.is_none() {
                return Err(ParsePgNodeError::FailedParsing("SELECT not found".to_string()));
            }

            let ctequery = cte.ctequery.as_ref().unwrap();

            if ctequery.node.is_none() {
                return Err(ParsePgNodeError::FailedParsing("SELECT not found".to_string()));
            }

            let ctequery = ctequery.node.unwrap();

            let select_stmt = match ctequery {
                Node::SelectStmt(stmt) => stmt,
                _ => return Err(ParsePgNodeError::FailedParsing("SELECT not found".to_string())),
            };

            if cte.cterecursive {
                return Err(ParsePgNodeError::Unimplemented("Recursive CTE not supported".to_string()));
            }

            let subquery = self.create_subquery_ref_from_select_and_alias(&select_stmt, cte.ctename)?;

            ctes.push(subquery);
        }

        Ok(ctes)
    }
}

pub type CTEList = Vec<SubqueryRef>;
