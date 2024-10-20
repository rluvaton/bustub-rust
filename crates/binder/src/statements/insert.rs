use crate::statements::traits::Statement;
use crate::statements::{SelectStatement, StatementTypeImpl};
use crate::table_ref::{BaseTableRef, TableReferenceTypeImpl};
use crate::try_from_ast_error::{ParseASTError, ParseASTResult};
use crate::Binder;
use std::fmt::Debug;
use std::sync::Arc;
use sqlparser::ast::Ident;
use catalog_schema::Column;

#[derive(Clone, Debug, PartialEq)]
pub struct InsertStatement {
    table: Arc<TableReferenceTypeImpl>,
    pub select: SelectStatement,
}

impl InsertStatement {
    pub fn new(table: Arc<TableReferenceTypeImpl>, select: SelectStatement) -> Self {
        assert!(matches!(*table, TableReferenceTypeImpl::BaseTable(_)), "table reference in insert must be base table");
        Self {
            table,
            select
        }
    }

    pub fn get_table(&self) -> &BaseTableRef {
        match &*self.table {
            TableReferenceTypeImpl::BaseTable(t) => t,
            _ => panic!("Invalid table ref in insert")
        }
    }
    
    fn is_same_columns(columns: &[Ident], schema_columns: &[Column]) -> bool {
        schema_columns.iter().all(|col| {
            let col_name = col.get_name();
            
            columns.iter().any(|query_col| query_col.to_string() == col_name.as_str())
        })
    }
}

impl Into<StatementTypeImpl> for InsertStatement {
    fn into(self) -> StatementTypeImpl {
        StatementTypeImpl::Insert(self)
    }
}

impl Statement for InsertStatement {
    type ASTStatement = sqlparser::ast::Insert;

    fn try_parse_ast<'a>(ast: &Self::ASTStatement, binder: &'a Binder) -> ParseASTResult<Self> {
        let table_name = ast.table_name.to_string();
        let table =  BaseTableRef::try_parse(table_name, ast.table_alias.as_ref().map(|alias| alias.to_string()), binder)?;

        if table.table.starts_with("__") {
            return Err(ParseASTError::FailedParsing(format!("Invalid table to insert: {}", table.table)));
        }
        
        if !InsertStatement::is_same_columns(ast.columns.as_slice(), table.schema.get_columns().as_slice()) {
            return Err(ParseASTError::Unimplemented("insert only supports all columns, don't specify columns".to_string()))
        }

        let select = ast.source.as_ref().ok_or(ParseASTError::FailedParsing("Must have source".to_string()))?;

        let select_statement = SelectStatement::try_parse_ast(select, binder)?;

        Ok(InsertStatement::new(Arc::new(TableReferenceTypeImpl::BaseTable(table)), select_statement))
    }


    fn try_parse_from_statement<'a>(statement: &sqlparser::ast::Statement, binder: &'a Binder) -> ParseASTResult<Self> {
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
    use db_core::catalog::Catalog;
    use sqlparser::dialect::GenericDialect;
    use sqlparser::parser::Parser;

    fn parse_insert_sql(sql: &str) -> Result<Vec<InsertStatement>, ParseASTError> {
        let catalog = Catalog::new(None, None, None);
        let mut binder = Binder::new(&catalog);
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
