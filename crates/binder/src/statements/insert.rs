use crate::sql_parser_helper::ColumnDefExt;
use crate::statements::traits::Statement;
use crate::statements::SelectStatement;
use crate::table_ref::{BaseTableRef, TableReferenceTypeImpl};
use crate::try_from_ast_error::{ParseASTError, ParseASTResult};
use crate::Binder;
use std::fmt::Debug;
use std::sync::Arc;

#[derive(Clone, Debug, PartialEq)]
pub struct InsertStatement {
    pub(crate) table: Arc<TableReferenceTypeImpl>,
    pub(crate) select: SelectStatement,
}

impl InsertStatement {
    pub fn new(table: Arc<TableReferenceTypeImpl>, select: SelectStatement) -> Self {
        assert!(matches!(*table, TableReferenceTypeImpl::BaseTable(_)), "table reference in insert must be base table");
        Self {
            table,
            select
        }
    }
}


impl Statement for InsertStatement {
    type ASTStatement = sqlparser::ast::Insert;

    fn try_parse_ast(ast: &Self::ASTStatement, binder: &mut Binder) -> ParseASTResult<Self> {
        if !ast.columns.is_empty() {
            return Err(ParseASTError::Unimplemented("insert only supports all columns, don't specify columns".to_string()))
        }

        let table_name = ast.table_name.to_string();
        let table =  BaseTableRef::try_parse(table_name, ast.table_alias.as_ref().map(|alias| alias.to_string()), binder)?;

        if table.table.starts_with("__") {
            return Err(ParseASTError::FailedParsing(format!("Invalid table to insert: {}", table.table)));
        }

        let select = ast.source.as_ref().ok_or(ParseASTError::FailedParsing("Must have source".to_string()))?;

        let select_statement = SelectStatement::try_parse_ast(select, binder)?;

        Ok(InsertStatement::new(Arc::new(TableReferenceTypeImpl::BaseTable(table)), select_statement))
    }


    fn try_parse_from_statement(statement: &sqlparser::ast::Statement, binder: &mut Binder) -> ParseASTResult<Self> {
        match &statement {
            sqlparser::ast::Statement::Insert(ast) => Self::try_parse_ast(ast, binder),
            _ => Err(ParseASTError::IncompatibleType)
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::statements::traits::Statement;
    use crate::statements::InsertStatement;
    use crate::try_from_ast_error::ParseASTError;
    use crate::Binder;
    use sqlparser::dialect::GenericDialect;
    use sqlparser::parser::Parser;

    fn parse_insert_sql(sql: &str) -> Result<Vec<InsertStatement>, ParseASTError> {
        let mut binder = Binder::default();
        let statements = Parser::parse_sql(&GenericDialect {}, sql).unwrap();
        statements.iter().map(|stmt| InsertStatement::try_parse_from_statement(stmt, &mut binder)).collect()
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
