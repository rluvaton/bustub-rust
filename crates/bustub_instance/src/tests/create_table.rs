#[cfg(test)]
mod tests {
    use catalog_schema_mocks::MockTableName;
    use data_types::Value;
    use execution_common::CheckOptions;
    use crate::BustubInstance;
    use crate::result_writer::NoopWriter;

    #[test]
    fn should_create_table() {
        let mut instance = BustubInstance::in_memory(None);

        let sql = "CREATE TABLE books (id int);";

        instance.execute_user_input(sql, &mut NoopWriter::default(), CheckOptions::default()).expect("Should execute");

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
