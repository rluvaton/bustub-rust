#[cfg(test)]
mod tests {
    use catalog_schema_mocks::MockTableName;
    use execution_common::CheckOptions;
    use crate::BustubInstance;

    #[test]
    fn should_select() {
        let mut instance = BustubInstance::in_memory(None);
        instance.generate_mock_table();

        let sql = format!("SELECT number from {}", MockTableName::Table123);

        let actual = instance.execute_single_select_sql(sql.as_str(), CheckOptions::default()).expect("Should execute");

        let expected = actual.create_with_same_schema(vec![
            vec![]
        ]);

        assert_eq!(actual, expected)
    }
}
