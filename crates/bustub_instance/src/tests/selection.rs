#[cfg(test)]
mod tests {
    use crate::BustubInstance;

    #[test]
    fn should_select() {
        let instance = BustubInstance::in_memory(None);

        instance.execute_user_input("SELECT id from ");
    }
}
