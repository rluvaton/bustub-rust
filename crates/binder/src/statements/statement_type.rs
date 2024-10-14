use crate::statements::{CreateStatement, DeleteStatement, InsertStatement, SelectStatement};

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
    // Drop,           // drop statement type
    // Index,          // index statement type
    // VariableSet,   // set variable statement type
    // VariableShow,  // show variable statement type
    // Transaction,    // txn statement type
}
