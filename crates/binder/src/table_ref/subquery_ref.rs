use crate::expressions::{ColumnRef, ExpressionTypeImpl};
use crate::statements::{SelectStatement, Statement};
use crate::table_ref::{TableRef, TableReferenceTypeImpl};
use crate::try_from_ast_error::{ParseASTError, ParseASTResult};
use crate::Binder;
use sqlparser::ast::{Cte, Query, TableFactor, With};

#[derive(Clone, Debug, PartialEq)]
pub struct SubqueryRef {
    /// Subquery.
    pub subquery: SelectStatement,

    /// Name of each item in the select list.
    pub select_list_name: Vec<Vec<String>>,

    // Alias
    pub alias: String,
}

impl SubqueryRef {
    pub(crate) fn parse_with<'a>(with: &With, binder: &'a Binder<'a>) -> ParseASTResult<Vec<Self>> {
        if with.recursive {
            return Err(ParseASTError::Unimplemented("recursive CTE is not supported".to_string()));
        }

        with.cte_tables
            .iter()
            .map(|cte_ele| Self::parse_cte(cte_ele, binder))
            .collect()
    }

    pub(crate) fn parse_cte<'a>(cte: &Cte, binder: &'a Binder<'a>) -> ParseASTResult<Self> {
        Self::parse_from_subquery(&cte.query, Some(cte.alias.name.value.clone()), binder)
    }

    pub(crate) fn parse_from_subquery<'a>(subquery: &Box<Query>, alias: Option<String>, binder: &'a Binder<'a>) -> ParseASTResult<Self> {
        let subquery = SelectStatement::try_parse_ast(&*subquery, binder)?;

        let select_list_name: Vec<Vec<String>> = subquery.select_list
            .iter()
            .map(|col| {
                match col {
                    ExpressionTypeImpl::ColumnRef(c) => c.col_name.clone(),
                    ExpressionTypeImpl::Alias(a) => vec![a.alias.clone()],
                    _ => {
                        let name = format!("__item#{}", binder.get_and_increment_universal_id());

                        vec![name]
                    }
                }
            })
            .collect::<Vec<_>>();

        Ok(SubqueryRef {
            subquery,
            select_list_name,
            alias: alias.unwrap_or("<unnamed>".to_string())
        })
    }
}

impl Into<TableReferenceTypeImpl> for SubqueryRef {
    fn into(self) -> TableReferenceTypeImpl {
        TableReferenceTypeImpl::SubQuery(self)
    }
}

impl TableRef for SubqueryRef {
    fn resolve_column(&self, col_name: &[String], _binder: &Binder) -> ParseASTResult<Option<ColumnRef>> {
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

    fn try_from_ast<'a>(ast: &TableFactor, binder: &'a Binder) -> ParseASTResult<Self> {
        match ast {
            TableFactor::Derived { subquery, alias, ..} => {
                Self::parse_from_subquery(subquery, alias.as_ref().map(|alias| alias.name.value.clone()), binder)
            },
            _ => Err(ParseASTError::IncompatibleType)
        }
    }
}

pub type CTEList = Vec<SubqueryRef>;
