use std::fmt::{Display, Formatter};

/** WindowFunctionType enumerates all the possible window functions in our system */

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum WindowFunctionType {
    CountStarAggregate,
    CountAggregate,
    SumAggregate,
    MinAggregate,
    MaxAggregate,
    Rank,
}

impl Display for WindowFunctionType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            WindowFunctionType::CountStarAggregate => f.write_str("count_star"),
            WindowFunctionType::CountAggregate => f.write_str("count"),
            WindowFunctionType::SumAggregate => f.write_str("sum"),
            WindowFunctionType::MinAggregate => f.write_str("min"),
            WindowFunctionType::MaxAggregate => f.write_str("max"),
            WindowFunctionType::Rank => f.write_str("rank"),
        }
    }
}
