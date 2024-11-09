use crate::statements::traits::Statement;
use crate::statements::StatementTypeImpl;
use crate::try_from_ast_error::{ParseASTError, ParseASTResult};
use crate::Binder;
use sqlparser::ast::ObjectType;
use std::fmt::Debug;

#[derive(Debug, PartialEq, Clone)]
pub struct DropTableStatement {
    table_name: String,
    if_exists: bool,
}

impl DropTableStatement {
    pub fn get_table_name(&self) -> &str {
        self.table_name.as_str()
    }

    pub fn get_if_exists(&self) -> bool {
        self.if_exists
    }
}

impl Into<StatementTypeImpl> for DropTableStatement {
    fn into(self) -> StatementTypeImpl {
        StatementTypeImpl::DropTable(self)
    }
}

impl Statement for DropTableStatement {
    type ASTStatement = sqlparser::ast::Statement;

    fn try_parse_ast<'a>(ast: &Self::ASTStatement, _binder: &'a Binder<'a>) -> ParseASTResult<Self> {
        match ast {
            sqlparser::ast::Statement::Drop {
                object_type,
                if_exists,
                names,
                ..
            } => {
                if !matches!(object_type, ObjectType::Table) {
                    return Err(ParseASTError::IncompatibleType);
                }

                if names.is_empty() {
                    return Err(ParseASTError::FailedParsing("Tables to drop cannot be empty".to_string()));
                }

                if names.len() > 1 {
                    return Err(ParseASTError::Unimplemented(format!("Only 1 table table to drop is supported, instead got {names:?}")))
                }

                let table_name = names[0].to_string();
                if table_name.starts_with("__") {
                    return Err(ParseASTError::FailedParsing(format!("Invalid table to drop: {table_name}")));
                }

                Ok(Self {
                    table_name,
                    if_exists: *if_exists,
                })
            },
            _ => Err(ParseASTError::IncompatibleType),
        }
    }

    fn try_parse_from_statement<'a>(statement: &sqlparser::ast::Statement, binder: &'a Binder) -> ParseASTResult<Self> {
        match &statement {
            sqlparser::ast::Statement::Drop { .. } => Self::try_parse_ast(statement, binder),
            _ => Err(ParseASTError::IncompatibleType)
        }
    }
}
