use std::fmt::Debug;
use crate::statements::StatementType;

pub trait Statement: Debug + From<Statement::PgNode> {
    const TYPE: StatementType;

    type PgNode;

    // type PgNode: pg_parse::ast::Node;

    // TODO - add from
}
