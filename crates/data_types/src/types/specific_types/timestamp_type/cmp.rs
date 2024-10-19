use crate::{partial_eq_null, ComparisonDBTypeTrait, DBTypeIdImpl, FormatDBTypeTrait, TimestampType, TimestampUnderlyingType, Value, VarcharType};
use std::cmp::Ordering;

impl PartialEq for TimestampType {
    fn eq(&self, other: &Self) -> bool {
        partial_eq_null!(self.is_null(), other.is_null());

        self.0 == other.0
    }
}

impl PartialEq<Value> for TimestampType {
    fn eq(&self, other: &Value) -> bool {
        partial_eq_null!(self.is_null(), other.is_null());

        match other.get_value() {
            DBTypeIdImpl::VARCHAR(other) => other.eq(&VarcharType::from(self)),
            DBTypeIdImpl::TIMESTAMP(other) => self.eq(other),
            _ => unreachable!()
        }
    }
}

impl PartialEq<TimestampUnderlyingType> for TimestampType {
    fn eq(&self, other: &TimestampUnderlyingType) -> bool {
        self.0 == *other
    }
}


impl PartialOrd for TimestampType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.is_null() && other.is_null() {
            return Some(Ordering::Equal);
        }

        self.0.partial_cmp(&other.0)
    }
}

impl PartialOrd<Value> for TimestampType {
    fn partial_cmp(&self, other: &Value) -> Option<Ordering> {
        let other_type_id = other.get_db_type_id();
        assert!(Self::TYPE.check_comparable(&other_type_id));

        if self.is_null() && other.is_null() {
            return Some(Ordering::Equal);
        }


        match other.get_value() {
            DBTypeIdImpl::TIMESTAMP(rhs) => self.partial_cmp(rhs),
            // TODO - add var char
            _ => unreachable!()
        }
    }
}

impl Eq for TimestampType {}

impl Ord for TimestampType {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl ComparisonDBTypeTrait for TimestampType {
    fn is_zero(&self) -> bool {
        panic!("is_zero is not available for timestamp")
    }

    fn get_min_value() -> Self {
        Self::new(Self::MIN)
    }

    fn get_max_value() -> Self {
        Self::new(Self::MAX)
    }

    // TODO - this is not the same as the value
    fn is_null(&self) -> bool {
        self.0 == Self::NULL
    }
}
