use crate::statements::traits::Statement;
use crate::statements::{DeleteStatement, SelectStatement, StatementType};
use crate::table_ref::BaseTableRef;
use std::fmt::Debug;
use std::sync::Arc;

#[derive(Debug, PartialEq)]
pub struct InsertStatement {
    pub(crate) table: Arc<BaseTableRef>,
    pub(crate) select: Arc<SelectStatement>,
}

impl InsertStatement {
    pub fn new(table: Arc<BaseTableRef>, select: Arc<SelectStatement>) -> Self {
        Self {
            table,
            select
        }
    }
}



// impl TryFrom<NodeRef<'_>> for CreateStatement {
//     type Error = TryFromASTError;
//
//     fn try_from(value: NodeRef) -> Result<Self, Self::Error> {
//         println!("{:#?}", value);
//         let stmt = match value {
//             NodeRef::CreateStmt(stmt) => {
//                 stmt
//             }
//             _ => return Err(TryFromASTError::IncompatibleType),
//         };
//
//         let relation_info = stmt.relation.as_ref();
//
//         if relation_info.is_none() {
//             return Err(TryFromASTError::FailedParsing("missing table name".to_string()));
//         }
//
//         let relation_info = relation_info.unwrap();
//
//
//         let table_elts = &stmt.table_elts;
//
//         let mut columns = vec![];
//         let mut primary_key = vec![];
//
//         for node in table_elts {
//             if let Some(node) = &node.node {
//                 match node {
//                     Node::ColumnDef(column_def) => {
//                         let column = column_def.try_convert_into_column().map_err(|err| TryFromASTError::FailedParsing(err.to_string()))?;
//
//                         if column_def.is_primary_key() {
//                             primary_key.push(column.get_name().clone());
//                         }
//
//                         columns.push(column);
//                     },
//                     Node::Constraint(constraint) => {
//                         if constraint.is_primary_key() {
//                             primary_key.append(&mut constraint.get_keys_names());
//                         }
//                     }
//                     _ => unimplemented!("Unknown column definition {:#?}", node)
//                 }
//             }
//         };
//
//         Ok(Self {
//             table: relation_info.relname.clone(),
//             columns,
//             primary_key,
//         })
//     }
// }
