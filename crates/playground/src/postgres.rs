#[cfg(test)]
mod tests {
    use pg_query::NodeRef;
    use pg_query::protobuf::CreateStmt;

    use sqlparser::dialect::GenericDialect;
    use sqlparser::parser::Parser;

    #[test]
    fn test() {
        let create_table_sql = "CREATE TABLE distributors (
    did     integer PRIMARY KEY,
    name    varchar(40)
);";
        let result = pg_query::parse(create_table_sql);
        assert!(result.is_ok());
        let result = result.unwrap();
        let create_stmt = result.protobuf.nodes()[0].0;

        match create_stmt {
            NodeRef::CreateStmt(stmt) => {
                println!("{:#?}", stmt);
            },
            _ => unreachable!()
        }

        // println!("{:?}", create_stmt);
        // assert_eq!(result.tables(), vec!["contacts"]);
        // assert!(matches!(result.protobuf.nodes()[0].0, NodeRef::SelectStmt(_)));
    }


    #[test]
    fn sql_parser() {

        let sql = "CREATE TABLE distributors (
    did     integer PRIMARY KEY,
    name    varchar(40)
);";

        let dialect = GenericDialect {}; // or AnsiDialect, or your own dialect ...

        let ast = Parser::parse_sql(&dialect, sql).unwrap();

        println!("{:#?}", ast);
    }
}
