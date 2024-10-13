use pg_query::protobuf::a_const::Val;
use pg_query::protobuf::{ColumnDef, ConstrType};
use pg_query::protobuf::node::Node;
use data_types::DBTypeId;
use db_core::catalog::Column;
use crate::pg_query_helpers::ConstraintExt;

pub(crate) trait ColumnDefExt {
    fn try_convert_into_column(&self) -> error_utils::anyhow::Result<Column>;

    fn is_primary_key(&self) -> bool;
}

impl ColumnDefExt for ColumnDef {
    fn try_convert_into_column(&self) -> error_utils::anyhow::Result<Column> {
        let name = &self.colname;

        let node_type = &self.type_name;

        if node_type.is_none() {
            return Err(error_utils::anyhow!("node type is missing for column {}", name));
        }

        let node_type = node_type.as_ref().unwrap();

        let db_type_id: Option<DBTypeId> = node_type.names.iter().find_map(|name| {
            if name.node.is_none() {
                return None;
            }

            let node = name.node.as_ref().unwrap();

            let str = match node {
                Node::String(str) => str,
                _ => return None
            };

            let db_type_id = match str.sval.as_str() {
                "int4" => DBTypeId::INT,
                "varchar" => DBTypeId::VARCHAR,
                _ => return None,
            };

            return Some(db_type_id)
        });

        if db_type_id.is_none() {
            return Err(error_utils::anyhow!("Was unable to find the column type of node {:?}", node_type));
        }

        let db_type_id = db_type_id.unwrap();

        if !matches!(db_type_id, DBTypeId::VARCHAR) {
            return Ok(Column::new_fixed_size(name.clone(), db_type_id));
        }

        let varchar_size: Option<i32> = node_type.typmods.iter().find_map(|name| {
            if name.node.is_none() {
                return None;
            }

            let node = name.node.as_ref().unwrap();

            let aconst = match node {
                Node::AConst(aconst) => aconst,
                _ => return None
            };

            if let Some(val) = &aconst.val {
                return match val {
                    Val::Ival(size) => Some(size.ival),
                    _ => None
                }
            }

            None
        });

        if varchar_size.is_none() {
            return Err(error_utils::anyhow!("Was unable to find the size of varchar column {:?}", node_type));
        }

        Ok(Column::new_variable_size(name.clone(), db_type_id, varchar_size.unwrap() as u32))
    }

    fn is_primary_key(&self) -> bool {
        self.constraints.iter().any(|constraint| {
            if constraint.node.is_none() {
                return false;
            }
            match constraint.node.as_ref().unwrap() {
                Node::Constraint(c) => c.is_primary_key(),
                _ => return false,
            }
        })
    }
}
