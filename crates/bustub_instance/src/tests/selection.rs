#[cfg(test)]
mod tests {
    use catalog_schema_mocks::MockTableName;
    use data_types::{IntUnderlyingType, Value};
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

    #[test]
    fn should_select_from_empty_table() {
        let mut instance = BustubInstance::in_memory(None);

        let sql = "CREATE TABLE books (id int);";

        instance.execute_user_input(sql, CheckOptions::default()).expect("Should execute");

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

            instance.execute_user_input(sql, CheckOptions::default()).expect("Should execute");
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

            instance.execute_user_input(sql, CheckOptions::default()).expect("Should execute");
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

            instance.execute_user_input(sql, CheckOptions::default()).expect("Should execute");
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

    #[test]
    fn should_return_unsupported_for_sort() {
        let mut instance = BustubInstance::in_memory(None);

        // Create table
        {
            let sql = "CREATE TABLE books (id int);";

            instance.execute_user_input(sql, CheckOptions::default()).expect("Should execute");
        }

        let result = instance.execute_single_select_sql("SELECT id from books order by id;", CheckOptions::default());

        let err = result.expect_err("Should fail");

        // TODO - better error display where exactly the unsupported syntax
        assert_eq!(err.to_string(), "Using unimplemented features. ORDER BY is not supported at the moment");
    }

    #[test]
    fn select_star() {
        let mut instance = BustubInstance::in_memory(None);
        instance.generate_mock_table();

        let sql = format!("SELECT * from {} limit 3", MockTableName::Table1);

        let actual = instance.execute_single_select_sql(sql.as_str(), CheckOptions::default()).expect("Should execute");

        let expected = actual.create_with_same_schema(vec![
            vec![Value::from(0), Value::from(0)],
            vec![Value::from(1), Value::from(100)],
            vec![Value::from(2), Value::from(200)],
        ]);

        assert_eq!(actual, expected)
    }

    #[test]
    fn select_from_mock_table_3() {
        let mut instance = BustubInstance::in_memory(None);
        instance.generate_mock_table();

        let sql = format!("SELECT * from {} limit 3", MockTableName::Table3);

        let actual = instance.execute_single_select_sql(sql.as_str(), CheckOptions::default()).expect("Should execute");

        let expected = actual.create_with_same_schema(vec![
            vec![Value::from(0), Value::from("0-💩")],
            vec![Value::from(Option::<IntUnderlyingType>::None), Value::from("1-💩")],
            vec![Value::from(2), Value::from("2-💩")],
        ]);

        assert_eq!(actual, expected)
    }

    #[test]
    fn should_be_able_to_select_from_every_the_mock_tables() {
        let mut instance = BustubInstance::in_memory(None);
        instance.generate_mock_table();

        for table_name in MockTableName::create_iter() {
            let sql = format!("SELECT * from {} limit 10", table_name);

            instance
                .execute_single_select_sql(sql.as_str(), CheckOptions::default())
                .expect(format!("Should select from {}", table_name).as_str());
        }
    }

    #[test]
    fn select_from_table_with_varchar_columns() {
        let mut instance = BustubInstance::in_memory(None);

        // Create table
        {
            let sql = "CREATE TABLE books (id int, s varchar(128));";

            instance.execute_user_input(sql, CheckOptions::default()).expect("Should execute");
        }

        // Insert rows
        {
            // \u{1F4A9} is the poop icon
            let sql = "INSERT INTO books (id, s) VALUES (1, 'hello'), (2, 'world!');";

            let inserted_rows_count_result = instance.execute_single_insert_sql(sql, CheckOptions::default()).expect("Should insert");

            assert_eq!(inserted_rows_count_result, inserted_rows_count_result.create_with_same_schema(vec![
                vec![Value::from(2)]
            ]));
        }

        let actual = instance.execute_single_select_sql("SELECT * from books;", CheckOptions::default()).expect("Should execute");

        // TODO - the order is not guaranteed so we should assert eq without order
        let expected = actual.create_with_same_schema(vec![
            vec![Value::from(1), Value::from("hello")],
            vec![Value::from(2), Value::from("world!")],
        ]);

        assert_eq!(actual, expected);
    }

    #[test]
    fn select_from_table_with_emoji_in_strings() {
        let mut instance = BustubInstance::in_memory(None);

        // Create table
        {
            let sql = "CREATE TABLE books (id int, s varchar(128));";

            instance.execute_user_input(sql, CheckOptions::default()).expect("Should execute");
        }

        // Insert rows
        {
            // \u{1F4A9} is the poop icon
            let sql = "INSERT INTO books (id, s) VALUES (1, '\u{1F4A9}'), (NULL, '\u{1F4A9}');";

            let inserted_rows_count_result = instance.execute_single_insert_sql(sql, CheckOptions::default()).expect("Should insert");

            assert_eq!(inserted_rows_count_result, inserted_rows_count_result.create_with_same_schema(vec![
                vec![Value::from(2)]
            ]));
        }

        let actual = instance.execute_single_select_sql("SELECT * from books;", CheckOptions::default()).expect("Should execute");

        // TODO - the order is not guaranteed so we should assert eq without order
        let expected = actual.create_with_same_schema(vec![
            vec![Value::from(1), Value::from("\u{1F4A9}")],
            vec![Value::from(Option::<IntUnderlyingType>::None), Value::from("\u{1F4A9}")],
        ]);

        assert_eq!(actual, expected);
    }
}
