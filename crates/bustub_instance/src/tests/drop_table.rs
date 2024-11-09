#[cfg(test)]
mod tests {
    use catalog_schema_mocks::MockTableName;
    use crate::result_writer::NoopWriter;
    use crate::BustubInstance;
    use execution_common::CheckOptions;

    #[test]
    fn should_drop_table_when_exists() {
        let mut instance = BustubInstance::in_memory(None);

        let sql = "CREATE TABLE books (id int);";

        instance.execute_user_input(sql, CheckOptions::default()).expect("Should execute");

        let sql = "DROP TABLE books;";

        instance.execute_user_input(sql, CheckOptions::default()).expect("Should execute");
    }

    #[test]
    fn should_fail_to_drop_table_when_missing() {
        let mut instance = BustubInstance::in_memory(None);

        let sql = "DROP TABLE books;";

        let error = instance.execute_user_input(sql, CheckOptions::default()).expect_err("Should fail");

        assert_eq!(error.to_string(), "fail missing table");
    }

    #[test]
    fn should_drop_table_when_exists_and_have_if_exists() {
        let mut instance = BustubInstance::in_memory(None);

        let sql = "CREATE TABLE books (id int);";

        instance.execute_user_input(sql, CheckOptions::default()).expect("Should execute");

        let sql = "DROP TABLE IF EXISTS books;";

        instance.execute_user_input(sql, CheckOptions::default()).expect("Should execute");
    }

    #[test]
    fn should_succeed_dropping_missing_table_when_have_if_exists() {
        let mut instance = BustubInstance::in_memory(None);

        let sql = "DROP TABLE IF EXISTS books;";

        instance.execute_user_input(sql, CheckOptions::default()).expect("Should execute");
    }

    #[test]
    fn should_fail_to_drop_mock_table() {
        let mut instance = BustubInstance::in_memory(None);
        instance.generate_mock_table();

        let sql = format!("DROP TABLE {};", MockTableName::Table1);

        let error = instance.execute_user_input(sql.as_str(), CheckOptions::default()).expect_err("Should fail");

        assert_eq!(error.to_string(), format!("Failed to parse Invalid table to drop: {}", MockTableName::Table1));
    }
}
