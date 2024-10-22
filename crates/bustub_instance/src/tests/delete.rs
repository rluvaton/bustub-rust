#[cfg(test)]
mod tests {
    use crate::result_writer::NoopWriter;
    use crate::BustubInstance;
    use data_types::Value;
    use execution_common::CheckOptions;

    #[test]
    fn should_fail_when_trying_to_delete_missing_table_without_where_condition() {
        let mut instance = BustubInstance::in_memory(None);

        let sql = "DELETE from some_table_name;";

        let err = instance.execute_single_delete_sql(sql, CheckOptions::default()).expect_err("Should fail");

        // TODO - add better error handling
        assert_eq!(err.to_string(), "Failed to parse Invalid table some_table_name");
    }

    #[test]
    fn should_fail_when_trying_to_delete_missing_table_with_where_condition() {
        let mut instance = BustubInstance::in_memory(None);

        let sql = "DELETE from some_table_name where id = 1;";

        let err = instance.execute_single_delete_sql(sql, CheckOptions::default()).expect_err("Should fail");

        // TODO - add better error handling
        assert_eq!(err.to_string(), "Failed to parse Invalid table some_table_name");
    }

    #[test]
    fn should_delete() {
        let mut instance = BustubInstance::in_memory(None);

        // Create table
        {
            let sql = "CREATE TABLE books (id int);";

            instance.execute_user_input(sql, &mut NoopWriter::default(), CheckOptions::default()).expect("Should execute");
        }

        // Insert rows
        {
            let sql = "INSERT INTO books (id) VALUES (1), (15), (42);";

            instance.execute_single_insert_sql(sql, CheckOptions::default()).expect("Should insert");
        }

        // Delete rows
        {
            let sql = "DELETE FROM books WHERE id = 15;";

            let deleted_rows_count_result = instance.execute_single_delete_sql(sql, CheckOptions::default()).expect("Should delete");

            assert_eq!(deleted_rows_count_result, deleted_rows_count_result.create_with_same_schema(vec![
                vec![Value::from(1)]
            ]));
        }

        instance.verify_integrity();

        let actual = instance.execute_single_select_sql("SELECT id from books;", CheckOptions::default()).expect("Should execute");

        // TODO - the order is not guaranteed so we should assert eq without order
        let expected = actual.create_with_same_schema(vec![
            vec![Value::from(1)],
            vec![Value::from(42)],
        ]);

        assert_eq!(actual, expected);
    }

    #[test]
    fn should_delete_with_returning() {
        let mut instance = BustubInstance::in_memory(None);

        // Create table
        {
            let sql = "CREATE TABLE books (id int);";

            instance.execute_user_input(sql, &mut NoopWriter::default(), CheckOptions::default()).expect("Should execute");
        }

        // Insert rows
        {
            let sql = "INSERT INTO books (id) VALUES (1), (15), (42);";

            instance.execute_single_insert_sql(sql, CheckOptions::default()).expect("Should insert");
        }

        // Delete rows
        let sql = "DELETE FROM books WHERE id = 15 returning id;";

        let actual = instance.execute_single_delete_sql(sql, CheckOptions::default()).expect("Should delete");

        let expected = actual.create_with_same_schema(vec![
            vec![Value::from(15)],
        ]);

        assert_eq!(actual, expected);

        instance.verify_integrity();
    }

    #[test]
    fn delete_should_return_number_of_delete() {
        let mut instance = BustubInstance::in_memory(None);

        // Create table
        {
            let sql = "CREATE TABLE books (id int);";

            instance.execute_user_input(sql, &mut NoopWriter::default(), CheckOptions::default()).expect("Should execute");
        }

        // Insert rows
        {
            let sql = "INSERT INTO books (id) VALUES (1), (15), (42);";

            instance.execute_single_insert_sql(sql, CheckOptions::default()).expect("Should insert");
        }

        // Delete rows
        let sql = "DELETE FROM books WHERE id = 15 or id = 1";

        let actual = instance.execute_single_delete_sql(sql, CheckOptions::default()).expect("Should delete");

        let expected = actual.create_with_same_schema(vec![
            // Number of deleted items
            vec![Value::from(2)],
        ]);

        assert_eq!(actual, expected);

        instance.verify_integrity();
    }

    #[test]
    fn should_delete_from_table_with_pk_index() {
        let mut instance = BustubInstance::in_memory(None);

        {
            let sql = "CREATE TABLE books (id int PRIMARY KEY);";

            instance.execute_user_input(sql, &mut NoopWriter::default(), CheckOptions::default()).expect("Should execute");
        }

        {
            let sql = "INSERT INTO books (id) VALUES (1), (15), (42), (13)";
            instance.execute_single_insert_sql(sql, CheckOptions::default()).expect("Should insert");
        }

        // Delete rows
        let sql = "DELETE FROM books WHERE id >= 15";

        let actual = instance.execute_single_delete_sql(sql, CheckOptions::default()).expect("Should delete");

        let expected = actual.create_with_same_schema(vec![
            // Number of deleted items - 15 and 42
            vec![Value::from(2)],
        ]);

        assert_eq!(actual, expected);

        // Verify integrity of table with indexes
        instance.verify_integrity();
    }

}
