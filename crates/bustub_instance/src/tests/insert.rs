#[cfg(test)]
mod tests {
    use crate::result_writer::NoopWriter;
    use crate::BustubInstance;
    use data_types::{DBTypeId, Value};
    use execution_common::CheckOptions;

    fn successful_create_table(instance: &mut BustubInstance, sql: &str) {
        instance.execute_user_input(sql, CheckOptions::default()).expect("Should execute");
    }

    fn successful_execute_insert_without_returning(instance: &mut BustubInstance, sql: &str, expected_count_to_insert: usize) {
        let inserted_rows_count_result = instance.execute_single_insert_sql(sql, CheckOptions::default()).expect("Should insert");

        assert_eq!(inserted_rows_count_result, inserted_rows_count_result.create_with_same_schema(vec![
            vec![Value::from(expected_count_to_insert as i32)]
        ]));

        instance.verify_integrity();
    }

    #[test]
    fn should_insert_to_newly_created_table() {
        let mut instance = BustubInstance::in_memory(None);

        {
            let sql = "CREATE TABLE books (id int);";

            instance.execute_user_input(sql, CheckOptions::default()).expect("Should execute");
        }

        let sql = "INSERT INTO books (id) VALUES (1), (24);";

        let inserted_rows_count_result = instance.execute_single_insert_sql(sql, CheckOptions::default()).expect("Should insert");

        assert_eq!(inserted_rows_count_result, inserted_rows_count_result.create_with_same_schema(vec![
            vec![Value::from(2)]
        ]));

        instance.verify_integrity();
    }

    #[test]
    fn should_insert_to_newly_created_table_with_big_int_column() {
        let mut instance = BustubInstance::in_memory(None);

        {
            let sql = "CREATE TABLE books (id bigint);";

            instance.execute_user_input(sql, CheckOptions::default()).expect("Should execute");
        }

        let sql = "INSERT INTO books (id) VALUES (1), (24);";

        let inserted_rows_count_result = instance.execute_single_insert_sql(sql, CheckOptions::default()).expect("Should insert");

        assert_eq!(inserted_rows_count_result, inserted_rows_count_result.create_with_same_schema(vec![
            vec![Value::from(2)]
        ]));

        instance.verify_integrity();
    }

    #[test]
    fn should_insert_to_newly_created_table_with_returning_only_column_name() {
        let mut instance = BustubInstance::in_memory(None);

        {
            let sql = "CREATE TABLE books (id int);";

            instance.execute_user_input(sql, CheckOptions::default()).expect("Should execute");
        }

        let sql = "INSERT INTO books (id) VALUES (1), (15) RETURNING id;";

        let actual = instance.execute_single_insert_sql(sql, CheckOptions::default()).expect("Should insert");

        let expected = actual.create_with_same_schema(vec![
            vec![Value::from(1)],
            vec![Value::from(15)],
        ]);

        assert_eq!(actual, expected);

        instance.verify_integrity();
    }

    #[test]
    fn should_insert_to_newly_created_table_with_returning_with_table_name() {
        let mut instance = BustubInstance::in_memory(None);

        {
            let sql = "CREATE TABLE books (id int);";

            instance.execute_user_input(sql, CheckOptions::default()).expect("Should execute");
        }

        let sql = "INSERT INTO books (id) VALUES (1), (15) RETURNING books.id;";

        let actual = instance.execute_single_insert_sql(sql, CheckOptions::default()).expect("Should insert");

        let expected = actual.create_with_same_schema(vec![
            vec![Value::from(1)],
            vec![Value::from(15)],
        ]);

        assert_eq!(actual, expected);

        instance.verify_integrity();
    }

    #[test]
    fn should_insert_to_newly_created_table_with_index_with_returning_only_column_name() {
        let mut instance = BustubInstance::in_memory(None);

        {
            let sql = "CREATE TABLE books (id int PRIMARY KEY);";

            instance.execute_user_input(sql, CheckOptions::default()).expect("Should execute");
        }

        let sql = "INSERT INTO books (id) VALUES (1), (15) RETURNING id;";

        let actual = instance.execute_single_insert_sql(sql, CheckOptions::default()).expect("Should insert");

        let expected = actual.create_with_same_schema(vec![
            vec![Value::from(1)],
            vec![Value::from(15)],
        ]);

        assert_eq!(actual, expected);

        instance.verify_integrity();
    }

    #[test]
    fn should_insert_to_newly_created_table_with_index_with_returning_with_table_name() {
        let mut instance = BustubInstance::in_memory(None);

        {
            let sql = "CREATE TABLE books (id int);";

            instance.execute_user_input(sql, CheckOptions::default()).expect("Should execute");
        }

        let sql = "INSERT INTO books (id) VALUES (1), (15) RETURNING books.id;";

        let actual = instance.execute_single_insert_sql(sql, CheckOptions::default()).expect("Should insert");

        let expected = actual.create_with_same_schema(vec![
            vec![Value::from(1)],
            vec![Value::from(15)],
        ]);

        assert_eq!(actual, expected);

        instance.verify_integrity();
    }

    #[test]
    fn insert_null_to_table() {
        let mut instance = BustubInstance::in_memory(None);

        // Create table
        {
            let sql = "CREATE TABLE books (id int, other int);";

            instance.execute_user_input(sql, CheckOptions::default()).expect("Should execute");
        }

        let sql = "INSERT INTO books (id, other) VALUES (1, 2), (NULL, 4), (5, NULL)";

        let inserted_rows_count_result = instance.execute_single_insert_sql(sql, CheckOptions::default()).expect("Should insert");

        assert_eq!(inserted_rows_count_result, inserted_rows_count_result.create_with_same_schema(vec![
            vec![Value::from(3)]
        ]));

        instance.verify_integrity();
    }

    #[test]
    fn insert_wrong_value_type() {
        let mut instance = BustubInstance::in_memory(None);

        // Create table
        {
            let sql = "CREATE TABLE t(id BIGINT, name varchar);";

            instance.execute_user_input(sql, CheckOptions::default()).expect("Should execute");
        }

        let sql = "insert into t(name, id) values(4, 'zoo');";

        let err = instance.execute_single_insert_sql(sql, CheckOptions::default())
            .expect_err("Should fail to insert");

        assert_eq!(err.to_string(), "schema error: expected INTEGER got VARCHAR");

        instance.verify_integrity();
    }

    #[test]
    fn insert_more_values_than_columns() {
        let mut instance = BustubInstance::in_memory(None);

        // Create table
        {
            let sql = "CREATE TABLE t(id BIGINT, name varchar);";

            instance.execute_user_input(sql, CheckOptions::default()).expect("Should execute");
        }

        let sql = "insert into t(id) values(4, 'zoo');";

        let err = instance.execute_single_insert_sql(sql, CheckOptions::default())
            .expect_err("Should fail to insert");

        assert_eq!(err.to_string(), "Failed to parse Error during planning: Column count doesn't match insert query!");

        instance.verify_integrity();
    }

    #[test]
    fn insert_more_columns_than_values() {
        let mut instance = BustubInstance::in_memory(None);

        // Create table
        {
            let sql = "CREATE TABLE t(id BIGINT, name varchar);";

            instance.execute_user_input(sql, CheckOptions::default()).expect("Should execute");
        }

        let sql = "insert into t(id, name) values(4);";

        let err = instance.execute_single_insert_sql(sql, CheckOptions::default())
            .expect_err("Should fail to insert");

        assert_eq!(err.to_string(), "Failed to parse Error during planning: Column count doesn't match insert query!");

        instance.verify_integrity();
    }

    #[test]
    fn insert_with_different_column_name() {
        let mut instance = BustubInstance::in_memory(None);

        // Create table
        {
            let sql = "CREATE TABLE t(id INT);";

            instance.execute_user_input(sql, CheckOptions::default()).expect("Should execute");
        }

        let sql = "insert into t(hello) values(4);";

        let err = instance.execute_single_insert_sql(sql, CheckOptions::default())
            .expect_err("Should fail to insert");

        assert_eq!(err.to_string(), "Failed to parse Column hello is missing");

        instance.verify_integrity();
    }

    #[test]
    fn insert_with_different_column_order() {
        let mut instance = BustubInstance::in_memory(None);

        // Create table
        {
            let sql = "CREATE TABLE t(id INT, name varchar);";

            instance.execute_user_input(sql, CheckOptions::default()).expect("Should execute");
        }

        let sql = "insert into t(name, id) values('zoo', 100);";

        let actual = instance.execute_single_insert_sql(sql, CheckOptions::default()).expect("Should insert with default");

        let expected = actual.create_with_same_schema(vec![
            vec![Value::from(1)],
        ]);

        assert_eq!(actual, expected);

        instance.verify_integrity();

        // Assert inserted values exists
        {
            let sql = "SELECT * from t";

            let actual = instance.execute_single_select_sql(sql, CheckOptions::default()).expect("Should execute");

            let expected = actual.create_with_same_schema(vec![
                vec![Value::from(100), Value::from("zoo")],
            ]);

            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn insert_partial_with_null_default() {
        let mut instance = BustubInstance::in_memory(None);

        // Create table
        {
            let sql = "CREATE TABLE t(id INT, name varchar NULL);";

            instance.execute_user_input(sql, CheckOptions::default()).expect("Should execute");
        }

        let sql = "insert into t(id) values(100);";

        let actual = instance.execute_single_insert_sql(sql, CheckOptions::default()).expect("Should insert with default");

        let expected = actual.create_with_same_schema(vec![
            vec![Value::from(1)],
        ]);

        assert_eq!(actual, expected);

        instance.verify_integrity();

        // Assert inserted values exists
        {
            let sql = "SELECT * from t";

            let actual = instance.execute_single_select_sql(sql, CheckOptions::default()).expect("Should execute");

            let expected = actual.create_with_same_schema(vec![
                vec![Value::from(100), Value::null(DBTypeId::VARCHAR)],
            ]);

            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn insert_partial_with_null_default_bigint() {
        let mut instance = BustubInstance::in_memory(None);

        // Create table
        {
            let sql = "CREATE TABLE t(id BIGINT, name varchar NULL);";

            instance.execute_user_input(sql, CheckOptions::default()).expect("Should execute");
        }

        let sql = "insert into t(id) values(100);";

        let actual = instance.execute_single_insert_sql(sql, CheckOptions::default()).expect("Should insert with default");

        let expected = actual.create_with_same_schema(vec![
            vec![Value::from(1)],
        ]);

        assert_eq!(actual, expected);

        instance.verify_integrity();

        // Assert inserted values exists
        {
            let sql = "SELECT * from t";

            let actual = instance.execute_single_select_sql(sql, CheckOptions::default()).expect("Should execute");

            let expected = actual.create_with_same_schema(vec![
                vec![Value::from(100), Value::null(DBTypeId::VARCHAR)],
            ]);

            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn insert_partial_with_not_null_default() {
        let mut instance = BustubInstance::in_memory(None);

        // Create table
        {
            let sql = "CREATE TABLE t(id INT, name varchar DEFAULT 'zoo');";

            instance.execute_user_input(sql, CheckOptions::default()).expect("Should execute");
        }

        let sql = "insert into t(id) values(100);";

        let actual = instance.execute_single_insert_sql(sql, CheckOptions::default()).expect("Should insert with default");

        let expected = actual.create_with_same_schema(vec![
            vec![Value::from(1)],
        ]);

        assert_eq!(actual, expected);

        instance.verify_integrity();

        // Assert inserted values exists
        {
            let sql = "SELECT * from t";

            let actual = instance.execute_single_select_sql(sql, CheckOptions::default()).expect("Should execute");

            let expected = actual.create_with_same_schema(vec![
                vec![Value::from(100), Value::from("zoo")],
            ]);
        }
    }

    #[test]
    fn insert_partial_with_null_default_with_returning() {
        let mut instance = BustubInstance::in_memory(None);

        // Create table
        {
            let sql = "CREATE TABLE t(id INT, name varchar NULL);";

            instance.execute_user_input(sql, CheckOptions::default()).expect("Should execute");
        }

        let sql = "insert into t(id) values(2) returning id;";

        let actual = instance.execute_single_insert_sql(sql, CheckOptions::default()).expect("Should insert with default");

        let expected = actual.create_with_same_schema(vec![
            vec![Value::from(2)],
        ]);

        assert_eq!(actual, expected);

        instance.verify_integrity();
    }

    // TODO - fix this
    #[ignore]
    #[test]
    fn insert_partial_with_default_with_returning_default_column() {
        let mut instance = BustubInstance::in_memory(None);

        // Create table
        {
            let sql = "CREATE TABLE t(id INT, name varchar NULL);";

            instance.execute_user_input(sql, CheckOptions::default()).expect("Should execute");
        }

        let sql = "insert into t(id) values(2) returning id, name;";

        let actual = instance.execute_single_insert_sql(sql, CheckOptions::default()).expect("Should insert with default");

        let expected = actual.create_with_same_schema(vec![
            vec![Value::from(2), Value::null(DBTypeId::VARCHAR)],
        ]);

        assert_eq!(actual, expected);

        instance.verify_integrity();
    }

    #[test]
    fn insert_bigint_first_and_select() {
        let mut instance = BustubInstance::in_memory(None);

        successful_create_table(&mut instance, "CREATE TABLE t(id BIGINT, name varchar);");

        successful_execute_insert_without_returning(
            &mut instance,
            "insert into t(id, name) values(1, 'foo');",
            1
        );

        // Assert inserted values exists
        {
            let sql = "SELECT * from t";

            let mut actual = instance.execute_single_select_sql(sql, CheckOptions::default()).expect("Should execute");

            // TODO AVOID CREATE WITH SAME SCHEMA
            let expected = actual.create_with_same_schema(vec![
                vec![Value::from(1), Value::from("foo")],
            ]);

            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn multiple_insert_and_select() {
        let mut instance = BustubInstance::in_memory(None);

        successful_create_table(&mut instance, "CREATE TABLE table_without_values(id BIGINT, name varchar);");

        successful_execute_insert_without_returning(
            &mut instance,
            "insert into table_without_values(id, name) values(1, 'foo');",
            1
        );

        successful_execute_insert_without_returning(
            &mut instance,
            "insert into table_without_values(name, id) values('bar', 2)",
            1
        );

        successful_execute_insert_without_returning(
            &mut instance,
            "insert into table_without_values(id) values(4)",
            1
        );


        // Assert inserted values exists
        {
            let sql = "SELECT * from table_without_values";

            let mut actual = instance.execute_single_select_sql(sql, CheckOptions::default()).expect("Should execute");

            // TODO AVOID CREATE WITH SAME SCHEMA
            let expected = actual.create_with_same_schema(vec![
                vec![Value::from(1), Value::from("foo")],
                vec![Value::from(2), Value::from("bar")],
                vec![Value::from(4), Value::null(DBTypeId::VARCHAR)],
            ]);

            actual.row_sort();

            assert_eq!(actual, expected);
        }
    }
}
