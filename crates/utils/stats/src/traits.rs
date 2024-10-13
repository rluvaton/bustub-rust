pub trait CreateTableOfStatistics {
    fn create_table(&self) -> comfy_table::Table;
}
