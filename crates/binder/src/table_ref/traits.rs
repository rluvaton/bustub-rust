use std::fmt::Debug;
use std::sync::Arc;
use pg_query::protobuf::node::Node;
use crate::Binder;
use crate::context_guard::ContextGuard;
use crate::parse_node_error::ParsePgNodeError;
use crate::table_ref::base_table_ref::BaseTableRef;
use crate::table_ref::cross_product_ref::CrossProductRef;
use crate::table_ref::cte_ref::CTERef;
use crate::table_ref::join_ref::{JoinRef, JoinType};
use crate::table_ref::table_reference_type::{TableReferenceType, TableReferenceTypeImpl};

/// A bound table reference.
pub(crate) trait TableRef: Debug + PartialEq {
    const TYPE: TableReferenceType;
}


impl Binder {

    pub(crate) fn parse_base_table_ref(&mut self, table_name: String, alias: Option<String>) -> Result<BaseTableRef, ParsePgNodeError> {
        let table_info = self.catalog.unwrap().get_table_by_name(&table_name);

        if table_info.is_none() {
            return Err(ParsePgNodeError::FailedParsing(format!("invalid table {}", table_name)));
        }

        let table_info = table_info.unwrap();

        Ok(BaseTableRef::new(table_name, table_info.clone().get_oid(), alias, table_info.get_schema()))
    }


    pub(crate) fn parse_join_expr(&mut self, table_ref: &pg_query::protobuf::JoinExpr) -> Result<TableReferenceTypeImpl, ParsePgNodeError> {
        let _ctx_guard = self.new_context();

        let join_type = match table_ref.jointype() {
            pg_query::protobuf::JoinType::JoinInner => JoinType::Inner,
            pg_query::protobuf::JoinType::JoinLeft => JoinType::Left,
            pg_query::protobuf::JoinType::JoinFull => JoinType::Outer,
            pg_query::protobuf::JoinType::JoinRight => JoinType::Right,
            _ => return Err(ParsePgNodeError::Unimplemented("the requested Join type is not supported".to_string()))
        };

        if table_ref.larg.is_none() {
            return Err(ParsePgNodeError::FailedParsing("larg must be defined".to_string()));
        }

        if table_ref.rarg.is_none() {
            return Err(ParsePgNodeError::FailedParsing("rarg must be defined".to_string()));
        }

        let left_table = self.create_table_ref(table_ref.larg.as_ref().unwrap())?;
        let right_table = self.create_table_ref(table_ref.rarg.as_ref().unwrap())?;

        let join_ref = Arc::new(JoinRef::new(join_type, left_table, right_table, None));

        self.scope = Some(join_ref.clone());

        if let Some(quals) = table_ref.quals.as_ref() {
            join_ref.condition = Some(self.parse_expression(quals)?);
        }

        Ok(join_ref.into())
    }

    pub(crate) fn parse_range_var(&mut self, table_ref: &pg_query::protobuf::RangeVar) -> Result<TableReferenceTypeImpl, ParsePgNodeError> {
        if let Some(cte_scope) = &self.cte_scope {
            // Firstly, find the table in CTE list.
            for cte in cte_scope {
                if cte.alias == table_ref.relname {
                    let name = if let Some(alias) = &table_ref.alias {
                        alias.aliasname.clone()
                    } else {
                        table_ref.relname.clone()
                    };

                    return Ok(CTERef::new(cte.alias, name).into());
                }
            }
        }

        Ok(self.parse_base_table_ref(table_ref.relname.clone(), table_ref.alias.clone().map(|alias| alias.aliasname))?.into())
    }

    pub(crate) fn parse_range_subselect(&mut self, range: &pg_query::protobuf::RangeSubselect) -> Result<TableReferenceTypeImpl, ParsePgNodeError> {
        if range.lateral {
            return Err(ParsePgNodeError::Unimplemented("LATERAL in subquery is not supported".to_string()))
        }

        if let Some(alias) = range.alias.as_ref() {
            if range.subquery.is_none() {
                return Err(ParsePgNodeError::FailedParsing("Subquery cannot be missing".to_string()));
            }

            return self.parse_subquery(range.subquery.as_ref().unwrap(), range.alias.as_ref().unwrap().aliasname)
        }
        if let Some(cte_scope) = &self.cte_scope {
            // Firstly, find the table in CTE list.
            for cte in cte_scope {
                if cte.alias == table_ref.relname {
                    let name = if let Some(alias) = &table_ref.alias {
                        alias.aliasname.clone()
                    } else {
                        table_ref.relname.clone()
                    };

                    return Ok(CTERef::new(cte.alias, name).into());
                }
            }
        }

        Ok(self.parse_base_table_ref(table_ref.relname.clone(), table_ref.alias.clone().map(|alias| alias.aliasname))?.into())
    }

    pub(crate) fn create_table_ref(&mut self, node: &pg_query::protobuf::Node) -> Result<TableReferenceTypeImpl, ParsePgNodeError> {
        if node.node.is_none() {
            return Err(ParsePgNodeError::FailedParsing("node is none".to_string()));
        }

        match node.node.as_ref().unwrap() {
            Node::RangeVar(node) => self.parse_range_var(node),
            Node::JoinExpr(node) => self.parse_join_expr(node),
            Node::RangeSubselect(_) => {}
            _ => return Err(ParsePgNodeError::Unimplemented("Unsupported node type".to_string()))
        }
    }

    fn parse_from(&mut self, nodes: Vec<pg_query::protobuf::Node>) -> Arc<TableReferenceTypeImpl> {
        let ctx_guard = self.new_context();

        if nodes.is_empty() {
            return Arc::new(TableReferenceTypeImpl::Empty);
        }

        if nodes.len() > 1 {
            // Bind cross joins

            // Extract the first node
            let mut nodes = nodes.clone();

            let left_node = nodes.remove(0);
            let right_node = nodes.remove(0);

            let result = CrossProductRef::new(left_node, right_node);
        }
    }
}
