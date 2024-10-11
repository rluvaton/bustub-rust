pub trait CreateTableOfStatistics {
    fn create_table(&self) -> prettytable::Table;
}
