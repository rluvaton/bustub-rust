use data_types::Value;

/** AggregateValue represents a value for each of the running aggregates */
pub struct AggregateValue {
    /** The aggregate values */
    pub aggregates: Vec<Value>,
}
