use data_types::Value;

/** AggregateKey represents a key in an aggregation operation */
pub struct AggregateKey {
    /** The group-by values */
    pub group_bys: Vec<Value>,
}

// TODO - can derive this?
impl PartialEq for AggregateKey {
    fn eq(&self, other: &Self) -> bool {
        self.group_bys
            .iter()
            .zip(other.group_bys.iter())
            .all(|(me, you)| me == you)
    }
}
