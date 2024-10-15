use crate::sql_parser_helper::{ColumnDefExt};
use crate::statements::traits::Statement;
use crate::statements::{StatementTypeImpl};
use crate::try_from_ast_error::{ParseASTResult, ParseASTError};
use std::fmt::Debug;
use catalog_schema::Column;
use crate::Binder;
use super::SqlParserCreateStatementExt;

#[derive(Clone, Debug, PartialEq)]
pub struct CreateStatement {
    pub table: String,
    pub columns: Vec<Column>,
    pub primary_key: Vec<String>,
}

impl CreateStatement {
    pub fn new(table: String, columns: Vec<Column>, primary_key: Vec<String>) -> Self {
        Self {
            table,
            columns,
            primary_key,
        }
    }
}

impl Into<StatementTypeImpl> for CreateStatement {
    fn into(self) -> StatementTypeImpl {
        StatementTypeImpl::Create(self)
    }
}

impl Statement for CreateStatement {
    type ASTStatement = sqlparser::ast::CreateTable;

    fn try_parse_ast(ast: &Self::ASTStatement, _binder: &mut Binder) -> ParseASTResult<Self> {
        let columns: ParseASTResult<Vec<Column>> = ast.columns.iter().map(|item| item.try_convert_into_column()).collect();
        let columns = columns?;

        if columns.is_empty() {
            return Err(ParseASTError::FailedParsing("Columns cannot be empty".to_string()))
        }


        Ok(CreateStatement {
            table: ast.name.to_string(),
            columns,
            primary_key: ast.try_get_primary_columns()?,
        })
    }

    fn try_parse_from_statement<'a>(statement: &sqlparser::ast::Statement, binder: &'a mut Binder) -> ParseASTResult<Self> {
        match &statement {
            sqlparser::ast::Statement::CreateTable(ast) => Self::try_parse_ast(ast, binder),
            _ => Err(ParseASTError::IncompatibleType)
        }
    }
}


#[cfg(test)]
mod tests {
    use std::rc::Rc;
    use std::sync::Arc;
    use crate::statements::create::CreateStatement;
    use data_types::DBTypeId;
    use db_core::catalog::{Catalog};

    use crate::try_from_ast_error::ParseASTError;
    use sqlparser::dialect::GenericDialect;
    use sqlparser::parser::Parser;
    use catalog_schema::Column;
    use crate::Binder;
    use crate::statements::traits::Statement;

    fn parse_create_sql(sql: &str) -> Result<Vec<CreateStatement>, ParseASTError> {
        let catalog = Catalog::new(None, None, None);
        let mut binder = Binder::new(catalog);
        let statements = Parser::parse_sql(&GenericDialect {}, sql).unwrap();
        statements.iter().map(|stmt| CreateStatement::try_parse_from_statement(stmt, &mut binder)).collect()
    }

    #[test]
    fn convert_create_table_to_statement() {
        let sql = "CREATE TABLE distributors (
    did     integer PRIMARY KEY,
    name    varchar(40)
);";

        let expected_create_statement = CreateStatement::new(
            "distributors".to_string(),
            vec![
                Column::new_fixed_size("did".to_string(), DBTypeId::INT),
                Column::new_variable_size("name".to_string(), DBTypeId::VARCHAR, 40),
            ],
            vec!["did".to_string()],
        );

        let create_statements = parse_create_sql(sql).expect("should parse");

        assert_eq!(create_statements, vec![expected_create_statement]);
    }

    #[test]
    fn convert_create_table_statement_with_all_supported_type() {
        let create_table_sql: String = vec![
            "CREATE TABLE distributors (",
            "some_boolean_1 BOOLEAN,",
            "some_boolean_2 boolean,",
            "some_boolean_3 BOOL,",
            "some_boolean_4 bool,",

            // there is no tinyint in postgres, so using char which has the same effect
            "some_tinyint_1 tinyint,",
            "some_smallint_1 smallint,",
            "some_smallint_2 int2,",
            "some_int_1 int,",
            "some_int_2 int4,",
            "some_int_3 integer,",
            "some_bigint_1 bigint,",
            "some_bigint_2 int8,",
            "some_decimal double precision,",
            "some_varchar_1 varchar(10),",
            "some_varchar_2 character varying(20)",

            // TODO - add timestamp

            ")",
        ].join("\n");

        let expected_create_statement = CreateStatement::new(
            "distributors".to_string(),
            vec![
                Column::new_fixed_size("some_boolean_1".to_string(), DBTypeId::BOOLEAN),
                Column::new_fixed_size("some_boolean_2".to_string(), DBTypeId::BOOLEAN),
                Column::new_fixed_size("some_boolean_3".to_string(), DBTypeId::BOOLEAN),
                Column::new_fixed_size("some_boolean_4".to_string(), DBTypeId::BOOLEAN),
                Column::new_fixed_size("some_tinyint_1".to_string(), DBTypeId::TINYINT),
                Column::new_fixed_size("some_smallint_1".to_string(), DBTypeId::SMALLINT),
                Column::new_fixed_size("some_smallint_2".to_string(), DBTypeId::SMALLINT),
                Column::new_fixed_size("some_int_1".to_string(), DBTypeId::INT),
                Column::new_fixed_size("some_int_2".to_string(), DBTypeId::INT),
                Column::new_fixed_size("some_int_3".to_string(), DBTypeId::INT),
                Column::new_fixed_size("some_bigint_1".to_string(), DBTypeId::BIGINT),
                Column::new_fixed_size("some_bigint_2".to_string(), DBTypeId::BIGINT),
                Column::new_fixed_size("some_decimal".to_string(), DBTypeId::DECIMAL),
                Column::new_variable_size("some_varchar_1".to_string(), DBTypeId::VARCHAR, 10),
                Column::new_variable_size("some_varchar_2".to_string(), DBTypeId::VARCHAR, 20),
            ],
            vec![],
        );


        let create_statements = parse_create_sql(create_table_sql.as_str()).expect("should parse");

        assert_eq!(create_statements, vec![expected_create_statement]);
    }

    #[test]
    fn convert_create_table_to_statement_multiple_primary_keys() {
        let create_table_sql = "CREATE TABLE distributors (
    did     integer,
    name    varchar(40),
CONSTRAINT code_title PRIMARY KEY(did,name)
);";

        let expected_create_statement = CreateStatement::new(
            "distributors".to_string(),
            vec![
                Column::new_fixed_size("did".to_string(), DBTypeId::INT),
                Column::new_variable_size("name".to_string(), DBTypeId::VARCHAR, 40),
            ],
            vec!["did".to_string(), "name".to_string()],
        );


        let create_statements = parse_create_sql(create_table_sql).expect("should parse");

        assert_eq!(create_statements, vec![expected_create_statement]);

    }
}
