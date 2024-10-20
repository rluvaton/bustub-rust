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

        let select_result = instance.execute_single_select_sql("SELECT id from books;", CheckOptions::default()).expect("Should execute");

        let expected_select_result = select_result.create_with_same_schema(vec![]);

        assert_eq!(select_result, expected_select_result);
    }
}
