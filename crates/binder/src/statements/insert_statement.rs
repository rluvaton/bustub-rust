use crate::pg_query_helpers::{ColumnDefExt, ConstraintExt};
use crate::statements::traits::{ParsePgNodeError, Statement};
use crate::statements::StatementType;
use db_core::catalog::Column;
use pg_query::protobuf::node::Node;
use pg_query::NodeRef;
use std::fmt::Debug;
use std::sync::Arc;
use crate::table_ref::BoundBaseTableRef;

#[derive(Debug, PartialEq)]
pub struct InsertStatement {
    pub(crate) table: Arc<BoundBaseTableRef>,
    pub(crate) select: Arc<SelectStatement>,
}

impl InsertStatement {
    pub fn new(table: Arc<BoundBaseTableRef>, select: Arc<SelectStatement>) -> Self {
        Self {
            table,
            select
        }
    }
}

impl Statement for CreateStatement {
    const TYPE: StatementType = StatementType::Create;
}


impl TryFrom<NodeRef<'_>> for CreateStatement {
    type Error = ParsePgNodeError;

    fn try_from(value: NodeRef) -> Result<Self, Self::Error> {
        println!("{:#?}", value);
        let stmt = match value {
            NodeRef::CreateStmt(stmt) => {
                stmt
            }
            _ => return Err(ParsePgNodeError::IncompatibleType),
        };

        let relation_info = stmt.relation.as_ref();

        if relation_info.is_none() {
            return Err(ParsePgNodeError::FailedParsing("missing table name".to_string()));
        }

        let relation_info = relation_info.unwrap();


        let table_elts = &stmt.table_elts;

        let mut columns = vec![];
        let mut primary_key = vec![];

        for node in table_elts {
            if let Some(node) = &node.node {
                match node {
                    Node::ColumnDef(column_def) => {
                        let column = column_def.try_convert_into_column().map_err(|err| ParsePgNodeError::FailedParsing(err.to_string()))?;

                        if column_def.is_primary_key() {
                            primary_key.push(column.get_name().clone());
                        }

                        columns.push(column);
                    },
                    Node::Constraint(constraint) => {
                        if constraint.is_primary_key() {
                            primary_key.append(&mut constraint.get_keys_names());
                        }
                    }
                    _ => unimplemented!("Unknown column definition {:#?}", node)
                }
            }
        };

        Ok(Self {
            table: relation_info.relname.clone(),
            columns,
            primary_key,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::statements::create_statement::CreateStatement;
    use crate::statements::traits::StatementTryFromResult;
    use data_types::DBTypeId;
    use db_core::catalog::Column;
    use pg_query::NodeRef;

    #[test]
    fn convert_create_table_to_statement() {
        let create_table_sql = "CREATE TABLE distributors (
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

        let result = pg_query::parse(create_table_sql).expect("Should parse");

        let actual_create_statement: StatementTryFromResult<CreateStatement> = result.protobuf.nodes()[0].0.try_into();

        assert_eq!(actual_create_statement, Ok(expected_create_statement));
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

        let result = pg_query::parse(create_table_sql.as_str()).expect("Should parse");

        let actual_create_statement: StatementTryFromResult<CreateStatement> = result.protobuf.nodes()[0].0.try_into();

        assert_eq!(actual_create_statement, Ok(expected_create_statement));
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

        let result = pg_query::parse(create_table_sql).expect("Should parse");

        let actual_create_statement: StatementTryFromResult<CreateStatement> = result.protobuf.nodes()[0].0.try_into();

        assert_eq!(actual_create_statement, Ok(expected_create_statement));
    }

    #[test]
    fn test() {
        let create_table_sql = "CREATE TABLE distributors (
    did     integer PRIMARY KEY,
    name    varchar(40)
);";
        let result = pg_query::parse(create_table_sql);
        assert!(result.is_ok());
        let result = result.unwrap();
        let create_stmt = result.protobuf.nodes()[0].0;

        match create_stmt {
            NodeRef::CreateStmt(stmt) => {
                println!("{:#?}", stmt);
            }
            _ => unreachable!()
        }

        // println!("{:?}", create_stmt);
        // assert_eq!(result.tables(), vec!["contacts"]);
        // assert!(matches!(result.protobuf.nodes()[0].0, NodeRef::SelectStmt(_)));
    }
}
