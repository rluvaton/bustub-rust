#[cfg(test)]
mod tests {
    use pg_query::NodeRef;
    #[test]
    fn test() {
        let result = pg_query::parse("SELECT * FROM contacts");
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result.tables(), vec!["contacts"]);
        assert!(matches!(result.protobuf.nodes()[0].0, NodeRef::SelectStmt(_)));
    }
}
