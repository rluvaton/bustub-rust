use crate::try_from_ast_error::ParseASTResult;
use crate::Binder;
use std::fmt::Debug;

/// This is the BoundStatement in Bustub
pub trait Statement: Debug {

    type ASTStatement;

    fn try_parse_ast(ast: &Self::ASTStatement, binder: &mut Binder) -> ParseASTResult<Self> where Self: Sized;

    fn try_parse_from_statement(statement: &sqlparser::ast::Statement, binder: &mut Binder) -> ParseASTResult<Self> where Self: Sized;
}
