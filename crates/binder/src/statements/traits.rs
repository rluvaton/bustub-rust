use crate::try_from_ast_error::ParseASTResult;
use crate::Binder;
use std::fmt::Debug;
use crate::statements::StatementTypeImpl;

/// This is the BoundStatement in Bustub
pub trait Statement: Debug + Into<StatementTypeImpl> {

    type ASTStatement;

    fn try_parse_ast<'a>(ast: &Self::ASTStatement, binder: &'a mut Binder) -> ParseASTResult<Self>;

    fn try_parse_from_statement<'a>(statement: &sqlparser::ast::Statement, binder: &'a mut Binder) -> ParseASTResult<Self>;
}
