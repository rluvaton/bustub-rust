use crate::statements::drop::{DropTableStatement};
use crate::statements::{CreateStatement, DeleteStatement, InsertStatement, SelectStatement, Statement};
use crate::try_from_ast_error::{ParseASTError, ParseASTResult};
use crate::{fallback_on_incompatible_2_args, Binder};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum StatementType {
    Invalid,        // invalid statement type
    Select,         // select statement type
    Insert,         // insert statement type
    Update,         // update statement type
    Create,         // create statement type
    Delete,         // delete statement type
    Explain,        // explain statement type
    Drop,           // drop statement type
    Index,          // index statement type
    VariableSet,   // set variable statement type
    VariableShow,  // show variable statement type
    Transaction,    // txn statement type
}
#[derive(Clone, Debug, PartialEq)]
pub enum StatementTypeImpl {
    Invalid,        // invalid statement type
    Select(SelectStatement),         // select statement type
    Insert(InsertStatement),         // insert statement type
    // Update,         // update statement type
    Create(CreateStatement),         // create statement type
    Delete(DeleteStatement),         // delete statement type
    // Explain,        // explain statement type
    DropTable(DropTableStatement),           // drop statement type
    // Index,          // index statement type
    // VariableSet,   // set variable statement type
    // VariableShow,  // show variable statement type
    // Transaction,    // txn statement type
}

impl Statement for StatementTypeImpl {
    type ASTStatement = ();

    fn try_parse_ast(_ast: &Self::ASTStatement, _binder: &Binder) -> ParseASTResult<Self>
    where
        Self: Sized
    {
        unreachable!()
    }

    fn try_parse_from_statement(statement: &sqlparser::ast::Statement, binder: &Binder) -> ParseASTResult<Self>
    where
        Self: Sized
    {
        fallback_on_incompatible_2_args!(try_parse_from_statement, statement, binder, {
            SelectStatement,
            InsertStatement,
            CreateStatement,
            DeleteStatement,
            DropTableStatement
        });

        Err(ParseASTError::IncompatibleType)
    }
}
