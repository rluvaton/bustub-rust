use crate::pg_query_helpers::{ColumnDefExt, ConstraintExt};
use crate::statements::traits::Statement;
use crate::statements::StatementType;
use db_core::catalog::Column;
use std::fmt::Debug;
use crate::try_from_ast_error::TryFromASTError;

#[derive(Debug, PartialEq)]
pub struct CreateStatement {
    pub(crate) table: String,
    pub(crate) columns: Vec<Column>,
    pub(crate) primary_key: Vec<String>,
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

impl TryFrom<&sqlparser::ast::Statement> for CreateStatement {
    type Error = TryFromASTError;

    fn try_from(value: &sqlparser::ast::Statement) -> Result<Self, Self::Error> {
        let create_table = match value {
            sqlparser::ast::Statement::CreateTable(create_table) => create_table,
            _ => return Err(TryFromASTError::IncompatibleType)
        };

        todo!()



    }
}

impl Statement for CreateStatement {
    const TYPE: StatementType = StatementType::Create;
}



#[cfg(test)]
mod tests {
    use crate::statements::create::CreateStatement;
    use crate::statements::traits::StatementTryFromResult;
    use data_types::DBTypeId;
    use db_core::catalog::Column;

    use sqlparser::dialect::GenericDialect;
    use sqlparser::parser::Parser;

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

        let dialect = GenericDialect {}; // or AnsiDialect, or your own dialect ...

        let statements = Parser::parse_sql(&dialect, sql).unwrap();

        let actual_create_statement: Vec<StatementTryFromResult<CreateStatement>> = statements.iter().map(|item| item.try_into()).collect();

        assert_eq!(actual_create_statement, vec![Ok(expected_create_statement)]);
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
            "some_tinyint_1 char,",
            "some_tinyint_2 CHAR,",

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
                Column::new_fixed_size("some_tinyint_2".to_string(), DBTypeId::TINYINT),

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

        // let result = pg_query::parse(create_table_sql.as_str()).expect("Should parse");
        //
        // let actual_create_statement: StatementTryFromResult<CreateStatement> = result.protobuf.nodes()[0].0.try_into();
        //
        // assert_eq!(actual_create_statement, Ok(expected_create_statement));
        todo!()
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

        // let result = pg_query::parse(create_table_sql).expect("Should parse");
        //
        // let actual_create_statement: StatementTryFromResult<CreateStatement> = result.protobuf.nodes()[0].0.try_into();
        //
        // assert_eq!(actual_create_statement, Ok(expected_create_statement));
        todo!()
    }

}
