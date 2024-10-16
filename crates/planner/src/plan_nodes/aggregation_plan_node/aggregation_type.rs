use std::fmt::{Display, Formatter};

// AggregationType enumerates all the possible aggregation functions in our system
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum AggregationType {
    CountStarAggregate,
    CountAggregate,
    SumAggregate,
    MinAggregate,
    MaxAggregate
}

impl Display for AggregationType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AggregationType::CountStarAggregate => f.write_str("count_str"),
            AggregationType::CountAggregate => f.write_str("count"),
            AggregationType::SumAggregate => f.write_str("sum"),
            AggregationType::MinAggregate => f.write_str("min"),
            AggregationType::MaxAggregate => f.write_str("max"),
        }
    }
}
