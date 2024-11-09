#[cfg(test)]
mod tests {
    use crate::result_writer::NoopWriter;
    use crate::BustubInstance;
    use execution_common::CheckOptions;

    #[test]
    fn should_create_table() {
        let mut instance = BustubInstance::in_memory(None);

        let sql = "CREATE TABLE books (id int);";

        instance.execute_user_input(sql, CheckOptions::default()).expect("Should execute");
    }
}
