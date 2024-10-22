#[cfg(test)]
mod tests {
    use catalog_schema_mocks::MockTableName;
    use data_types::Value;
    use execution_common::CheckOptions;
    use crate::BustubInstance;
    use crate::result_writer::NoopWriter;

    #[test]
    fn should_select_from_mock_table() {
        let mut instance = BustubInstance::in_memory(None);
        instance.generate_mock_table();

        let sql = format!("SELECT number from {}", MockTableName::Table123);

        let actual = instance.execute_single_select_sql(sql.as_str(), CheckOptions::default()).expect("Should execute");

        let expected = actual.create_with_same_schema(vec![
            vec![Value::from(0)],
            vec![Value::from(1)],
            vec![Value::from(2)],
        ]);

        assert_eq!(actual, expected)
    }

    #[test]
    fn should_fail_when_trying_to_select_missing_table() {
        let mut instance = BustubInstance::in_memory(None);
        instance.generate_mock_table();

        let sql = "SELECT number from some_table_name";

        let err = instance.execute_single_select_sql(sql, CheckOptions::default()).expect_err("Should fail");
        
        // TODO - add better error handling
        assert_eq!(err.to_string(), "Failed to parse Invalid table some_table_name");
    }

    #[test]
    fn should_select_from_mock_table_with_column_constant_filter() {
        let mut instance = BustubInstance::in_memory(None);
        instance.generate_mock_table();

        let sql = format!("SELECT number from {} where number = 1", MockTableName::Table123);

        let actual = instance.execute_single_select_sql(sql.as_str(), CheckOptions::default()).expect("Should execute");

        let expected = actual.create_with_same_schema(vec![
            vec![Value::from(1)],
        ]);

        assert_eq!(actual, expected)
    }

    #[ignore]
    #[test]
    fn should_select_from_empty_table() {
        let mut instance = BustubInstance::in_memory(None);

        let sql = "CREATE TABLE books (id int);";

        instance.execute_user_input(sql, &mut NoopWriter::default(), CheckOptions::default()).expect("Should execute");

        let select_result = instance.execute_single_select_sql("SELECT id from books;", CheckOptions::default()).expect("Should execute");

        let expected_select_result = select_result.create_with_same_schema(vec![]);

        assert_eq!(select_result, expected_select_result);
    }

    #[test]
    fn should_select_from_table_with_data() {
        let mut instance = BustubInstance::in_memory(None);

        // Create table
        {
            let sql = "CREATE TABLE books (id int);";

            instance.execute_user_input(sql, &mut NoopWriter::default(), CheckOptions::default()).expect("Should execute");
        }
        
        // Insert rows
        {
            let sql = "INSERT INTO books (id) VALUES (1), (15)";

            instance.execute_single_insert_sql(sql, CheckOptions::default()).expect("Should insert");
        }

        let actual = instance.execute_single_select_sql("SELECT id from books;", CheckOptions::default()).expect("Should execute");

        // TODO - the order is not guaranteed so we should assert eq without order
        let expected = actual.create_with_same_schema(vec![
            vec![Value::from(1)],
            vec![Value::from(15)],
        ]);

        assert_eq!(actual, expected);
    }

    #[test]
    fn should_select_with_aggregation() {
        let mut instance = BustubInstance::in_memory(None);

        // Create table
        {
            let sql = "CREATE TABLE books (id int);";

            instance.execute_user_input(sql, &mut NoopWriter::default(), CheckOptions::default()).expect("Should execute");
        }

        // Insert rows
        {
            let sql = "INSERT INTO books (id) VALUES (1), (15) returning id;";

            instance.execute_single_insert_sql(sql, CheckOptions::default()).expect("Should insert");
        }

        let actual = instance.execute_single_select_sql("SELECT count(1) from books;", CheckOptions::default()).expect("Should execute");

        // TODO - the order is not guaranteed so we should assert eq without order
        let expected = actual.create_with_same_schema(vec![
            vec![Value::from(2)],
        ]);

        assert_eq!(actual, expected);
    }

    #[test]
    fn should_select_with_many_aggregations_on_single_column() {
        let mut instance = BustubInstance::in_memory(None);

        // Create table
        {
            let sql = "CREATE TABLE books (id int);";

            instance.execute_user_input(sql, &mut NoopWriter::default(), CheckOptions::default()).expect("Should execute");
        }

        // Insert rows
        {
            let sql = "INSERT INTO books (id) VALUES (1), (15) returning id;";

            instance.execute_single_insert_sql(sql, CheckOptions::default()).expect("Should insert");
        }

        let actual = instance.execute_single_select_sql("SELECT count(*), count(2), count(null), sum(id), min(id), max(id) from books;", CheckOptions::default()).expect("Should execute");

        let expected = actual.create_with_same_schema(vec![
            vec![Value::from(2), Value::from(2), Value::from(0), Value::from(16), Value::from(1), Value::from(15)],
        ]);

        assert_eq!(actual, expected);
    }
}
