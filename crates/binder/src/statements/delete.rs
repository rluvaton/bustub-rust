use crate::expressions::{Constant, Expression, ExpressionTypeImpl};
use crate::sql_parser_helper::ColumnDefExt;
use crate::statements::traits::Statement;
use crate::table_ref::{BaseTableRef, TableReferenceTypeImpl};
use crate::try_from_ast_error::{ParseASTError, ParseASTResult};
use crate::Binder;
use sqlparser::ast::{FromTable, TableFactor};
use std::fmt::Debug;
use std::ops::DerefMut;
use std::rc::Rc;
use std::sync::Arc;
use crate::statements::{CreateStatement, StatementTypeImpl};

#[derive(Debug, PartialEq, Clone)]
pub struct DeleteStatement {
    pub(crate) table: Rc<TableReferenceTypeImpl>,
    pub(crate) expr: ExpressionTypeImpl,
}

impl DeleteStatement {
    pub fn new(table: Rc<TableReferenceTypeImpl>, expr: ExpressionTypeImpl) -> Self {
        assert!(matches!(*table, TableReferenceTypeImpl::BaseTable(_)), "table reference in delete must be base table");
        Self {
            table,
            expr
        }
    }
}

impl Into<StatementTypeImpl> for DeleteStatement {
    fn into(self) -> StatementTypeImpl {
        StatementTypeImpl::Delete(self)
    }
}

impl Statement for DeleteStatement {
    type ASTStatement = sqlparser::ast::Delete;

    fn try_parse_ast<'a>(ast: &Self::ASTStatement, binder: &'a Binder<'a>) -> ParseASTResult<Self> {
        let from = match &ast.from {
            FromTable::WithFromKeyword(with) => with,
            FromTable::WithoutKeyword(_) => return Err(ParseASTError::Unimplemented("From without keyword is not supported".to_string())),
        };

        if from.len() != 1 {
            return Err(ParseASTError::Unimplemented("From must have only 1 table".to_string()));
        }

        let from = &from[0];

        let table = match &from.relation {
            TableFactor::Table { name, alias, .. } => {
                BaseTableRef::try_parse(name.to_string(), alias.as_ref().map(|alias| alias.to_string()), binder)
            }
            _ => return Err(ParseASTError::Unimplemented(format!("From relation is not supported {}", from.relation)))
        };

        let table: Rc<TableReferenceTypeImpl> = Rc::new(table?.into());

        // TODO - guard
        let mut ctx_guard = binder.new_context();

        ctx_guard.context.lock().scope.replace(table.clone());

        let expr: ExpressionTypeImpl = if let Some(selection) = &ast.selection {
            ExpressionTypeImpl::try_parse_from_expr(selection, &*ctx_guard)?
        } else {
            Constant::new(true.into()).into()
        };

        Ok(DeleteStatement::new(table, expr))
    }


    fn try_parse_from_statement<'a>(statement: &sqlparser::ast::Statement, binder: &'a Binder) -> ParseASTResult<Self> {
        match &statement {
            sqlparser::ast::Statement::Delete(ast) => Self::try_parse_ast(ast, binder),
            _ => Err(ParseASTError::IncompatibleType)
        }
    }
}


#[cfg(test)]
mod tests {
    use parking_lot::Mutex;
    use crate::statements::traits::Statement;
    use crate::statements::DeleteStatement;
    use crate::try_from_ast_error::ParseASTError;
    use crate::Binder;
    use sqlparser::dialect::GenericDialect;
    use sqlparser::parser::Parser;
    use db_core::catalog::Catalog;

    fn parse_delete_sql(sql: &str) -> Result<Vec<DeleteStatement>, ParseASTError> {

        let catalog = Catalog::new(None, None, None);
        let mut binder = Binder::new(&catalog);
        let statements = Parser::parse_sql(&GenericDialect {}, sql).unwrap();
        statements.iter().map(|stmt| DeleteStatement::try_parse_from_statement(stmt, &mut binder)).collect()
    }

    // #[test]
    // fn convert_delete_to_statement() {
    //     let sql = "DELETE FROM tasks WHERE status = 'DONE' RETURNING *";
    //
    //     let expected_create_statement = DeleteStatement::new(
    //         BaseTableRef::new()
    //         "distributors".to_string(),
    //         vec![
    //             Column::new_fixed_size("did".to_string(), DBTypeId::INT),
    //             Column::new_variable_size("name".to_string(), DBTypeId::VARCHAR, 40),
    //         ],
    //         vec!["did".to_string()],
    //     );
    //
    //     let create_statements = parse_create_sql(sql).expect("should parse");
    //
    //     assert_eq!(create_statements, vec![expected_create_statement]);
    // }
}
