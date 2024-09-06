use std::cmp::Ordering;
use crate::types::{BigIntType, Value, BUSTUB_I64_MAX, BUSTUB_I64_MIN};
use crate::types::types::specific_types_trait::ComparisonDBTypeTrait;

impl PartialEq for BigIntType {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl PartialEq<Value> for BigIntType {
    fn eq(&self, other: &Value) -> bool {
        todo!()
    }
}

impl PartialEq<i64> for BigIntType {
    fn eq(&self, other: &i64) -> bool {
        self.value == *other
    }
}

impl PartialOrd for BigIntType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl PartialOrd<Value> for BigIntType {
    fn partial_cmp(&self, other: &Value) -> Option<Ordering> {
        todo!()
    }
}

impl PartialOrd<i64> for BigIntType {
    fn partial_cmp(&self, other: &i64) -> Option<Ordering> {
        self.value.partial_cmp(other)
    }
}

impl ComparisonDBTypeTrait for BigIntType {
    fn is_zero(&self) -> bool {
        self.value == 0
    }

    fn get_min_value() -> Self {
        Self::new(BUSTUB_I64_MIN)
    }

    fn get_max_value() -> Self {
        Self::new(BUSTUB_I64_MAX)
    }
}
