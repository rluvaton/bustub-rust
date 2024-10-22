use crate::sql_parser_helper::SelectItemExt;
use crate::statements::traits::Statement;
use crate::statements::{SelectStatement, StatementTypeImpl};
use crate::table_ref::{BaseTableRef, TableReferenceTypeImpl};
use crate::try_from_ast_error::{ParseASTError, ParseASTResult};
use crate::{Binder, ExpressionTypeImpl};
use catalog_schema::Column;
use sqlparser::ast::Ident;
use std::fmt::Debug;
use std::ops::Deref;
use std::rc::Rc;
use crate::statements::parse_returning::parse_returning;

#[derive(Clone, Debug, PartialEq)]
pub struct InsertStatement {
    table: Rc<TableReferenceTypeImpl>,
    pub select: SelectStatement,
    
    returning: Vec<ExpressionTypeImpl>
}

impl InsertStatement {
    pub fn new(table: Rc<TableReferenceTypeImpl>, select: SelectStatement, returning: Vec<ExpressionTypeImpl>) -> Self {
        assert!(matches!(*table, TableReferenceTypeImpl::BaseTable(_)), "table reference in insert must be base table");
        Self {
            table,
            select,
            returning
        }
    }

    pub fn get_table(&self) -> &BaseTableRef {
        match &*self.table {
            TableReferenceTypeImpl::BaseTable(t) => t,
            _ => panic!("Invalid table ref in insert")
        }
    }

    pub fn get_returning(&self) -> &[ExpressionTypeImpl] {
        self.returning.as_slice()
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
            // When we support this we should change the table context for returning, right?
            return Err(ParseASTError::Unimplemented("insert only supports all columns, don't specify columns".to_string()))
        }

        let select = ast.source.as_ref().ok_or(ParseASTError::FailedParsing("Must have source".to_string()))?;

        let select_statement = SelectStatement::try_parse_ast(select, binder)?;
        
        let table: Rc<TableReferenceTypeImpl> = Rc::new(table.into());
        
        let returning = parse_returning(&ast.returning, table.clone(), binder)?;

        Ok(InsertStatement::new(table, select_statement, returning))
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
}
