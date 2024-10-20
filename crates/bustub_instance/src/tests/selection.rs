#[cfg(test)]
mod tests {
    use catalog_schema_mocks::MockTableName;
    use data_types::Value;
    use execution_common::CheckOptions;
    use crate::BustubInstance;

    #[test]
    fn should_select() {
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
    fn should_select_with_column_constant_filter() {
        let mut instance = BustubInstance::in_memory(None);
        instance.generate_mock_table();

        let sql = format!("SELECT number from {} where number = 1", MockTableName::Table123);

        let actual = instance.execute_single_select_sql(sql.as_str(), CheckOptions::default()).expect("Should execute");

        let expected = actual.create_with_same_schema(vec![
            vec![Value::from(1)],
        ]);

        assert_eq!(actual, expected)
    }
}
