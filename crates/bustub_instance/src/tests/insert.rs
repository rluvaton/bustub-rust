#[cfg(test)]
mod tests {
    use data_types::Value;
    use crate::result_writer::NoopWriter;
    use crate::BustubInstance;
    use execution_common::CheckOptions;

    #[test]
    fn should_insert_to_newly_created_table() {
        let mut instance = BustubInstance::in_memory(None);

        {
            let sql = "CREATE TABLE books (id int);";

            instance.execute_user_input(sql, &mut NoopWriter::default(), CheckOptions::default()).expect("Should execute");
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

            instance.execute_user_input(sql, &mut NoopWriter::default(), CheckOptions::default()).expect("Should execute");
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

            instance.execute_user_input(sql, &mut NoopWriter::default(), CheckOptions::default()).expect("Should execute");
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

            instance.execute_user_input(sql, &mut NoopWriter::default(), CheckOptions::default()).expect("Should execute");
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

            instance.execute_user_input(sql, &mut NoopWriter::default(), CheckOptions::default()).expect("Should execute");
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

            instance.execute_user_input(sql, &mut NoopWriter::default(), CheckOptions::default()).expect("Should execute");
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

            instance.execute_user_input(sql, &mut NoopWriter::default(), CheckOptions::default()).expect("Should execute");
        }

        let sql = "insert into t(name, id) values(4, 'zoo');";

        let err = instance.execute_single_insert_sql(sql, CheckOptions::default())
            .expect_err("Should fail to insert");

        assert_eq!(err.to_string(), "aa");

        instance.verify_integrity();

        //
    }

    #[test]
    fn insert_partial_with_default() {
        let mut instance = BustubInstance::in_memory(None);

        // Create table
        {
            let sql = "CREATE TABLE t(id INT, name varchar NULL);";

            instance.execute_user_input(sql, &mut NoopWriter::default(), CheckOptions::default()).expect("Should execute");
        }

        let sql = "insert into t(id) values(1) RETURNING *;";

        let actual = instance.execute_single_insert_sql(sql, CheckOptions::default()).expect("Should insert with default");

        let expected = actual.create_with_same_schema(vec![
            vec![Value::from(1), Value::from(Option::<&str>::None)],
        ]);

        assert_eq!(actual, expected);

        instance.verify_integrity();
    }

}
