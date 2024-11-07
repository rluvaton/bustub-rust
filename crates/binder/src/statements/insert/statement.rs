use crate::statements::parse_returning::parse_returning;
use crate::statements::traits::Statement;
use crate::statements::{SelectStatement, StatementTypeImpl};
use crate::table_ref::{BaseTableRef, TableReferenceTypeImpl};
use crate::try_from_ast_error::{ParseASTError, ParseASTResult};
use crate::{Binder, ColumnOrderingAndDefaultValuesForInsert, ExpressionTypeImpl};
use std::fmt::Debug;
use std::ops::Deref;
use std::rc::Rc;

#[derive(Clone, Debug, PartialEq)]
pub struct InsertStatement {
    table: Rc<TableReferenceTypeImpl>,

    columns_ordering_and_default_values: Rc<ColumnOrderingAndDefaultValuesForInsert>,
    select: SelectStatement,

    returning: Vec<ExpressionTypeImpl>,
}

impl InsertStatement {
    pub fn new(table: Rc<TableReferenceTypeImpl>, columns_ordering_and_default_values: ColumnOrderingAndDefaultValuesForInsert, select: SelectStatement, returning: Vec<ExpressionTypeImpl>) -> Self {
        assert!(matches!(*table, TableReferenceTypeImpl::BaseTable(_)), "table reference in insert must be base table");
        Self {
            table,
            columns_ordering_and_default_values: Rc::new(columns_ordering_and_default_values),
            select,
            returning,
        }
    }

    pub fn get_table(&self) -> &BaseTableRef {
        match &*self.table {
            TableReferenceTypeImpl::BaseTable(t) => t,
            _ => panic!("Invalid table ref in insert")
        }
    }

    pub fn get_select(&self) -> &SelectStatement {
        &self.select
    }

    pub fn get_column_ordering(&self) -> &Rc<ColumnOrderingAndDefaultValuesForInsert> {
        &self.columns_ordering_and_default_values
    }

    pub fn get_returning(&self) -> &[ExpressionTypeImpl] {
        self.returning.as_slice()
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
        let table = BaseTableRef::try_parse(table_name, ast.table_alias.as_ref().map(|alias| alias.to_string()), binder)?;

        if table.table.starts_with("__") {
            return Err(ParseASTError::FailedParsing(format!("Invalid table to insert: {}", table.table)));
        }

        // Fail if there are duplicate columns
        {
            let mut columns = ast.columns.as_slice();
            for (index, column) in columns.iter().enumerate() {
                for other_column in columns[index + 1..].iter() {
                    if column == other_column {
                        return Err(ParseASTError::FailedParsing(format!("Schema contains duplicate unqualified field name {}", column)));
                    }
                }
            }
        }

        // Fails if we have unknown columns
        {
            let unknown_column = ast.columns.iter().find(|col| {
                let col_name = col.to_string();

                let column_index = table.schema.try_get_col_idx(col_name.as_str());

                // If column is missing
                column_index.is_none()
            });

            if let Some(missing_column) = unknown_column {
                return Err(ParseASTError::FailedParsing(format!("Column {missing_column} is missing")))
            }
        }

        let column_ordering = ColumnOrderingAndDefaultValuesForInsert::from_ast_and_schema(ast.columns.as_slice(), table.schema.deref());

        let select = ast.source.as_ref().ok_or(ParseASTError::FailedParsing("Must have source".to_string()))?;

        let select_statement = SelectStatement::try_parse_ast(select, binder)?;

        {
            match select_statement.table.deref() {
                TableReferenceTypeImpl::ExpressionList(list) => {
                    let number_of_columns = ast.columns.len();
                    let row_with_mismatch_values_count = list.values.iter()
                        .any(|row| row.len() != number_of_columns);

                    if row_with_mismatch_values_count {
                        return Err(ParseASTError::FailedParsing("Error during planning: Column count doesn't match insert query!".to_string()))
                    }
                }
                _ => return Err(ParseASTError::Unimplemented(format!("Insert data source {} is not supported", select_statement.table.get_name())))
            }
        }

        let table: Rc<TableReferenceTypeImpl> = Rc::new(table.into());

        let returning = parse_returning(&ast.returning, table.clone(), binder)?;

        Ok(InsertStatement::new(table, column_ordering, select_statement, returning))
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
