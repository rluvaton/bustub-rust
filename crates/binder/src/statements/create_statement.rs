use crate::statements::traits::Statement;
use crate::statements::StatementType;
use db_core::catalog::Column;
use std::fmt::Debug;

#[derive(Debug)]
pub struct CreateStatement {
    table: String,
    columns: Vec<Column>,
    primary_key: Vec<String>
}

impl Statement for CreateStatement {
    const TYPE: StatementType = StatementType::Create;
    type PgNode = pg_query::protobuf::CreateStmt;
}

impl From<pg_query::protobuf::CreateStmt> for CreateStatement {
    fn from(value: pg_query::protobuf::CreateStmt) -> Self {
        value.
        Self {

        }
    }
}
