use crate::{BooleanType, BooleanUnderlyingType, ComparisonDBTypeTrait, DBTypeIdImpl, FormatDBTypeTrait, Value};
use std::cmp::Ordering;

impl PartialEq for BooleanType {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl PartialEq<Value> for BooleanType {
    fn eq(&self, other: &Value) -> bool {
        let other_type_id = other.get_db_type_id();
        assert!(Self::TYPE.check_comparable(&other_type_id));

        match other.get_value() {
            DBTypeIdImpl::BOOLEAN(rhs) => self.eq(rhs),
            // TODO - add var char
            _ => unreachable!()
        }
    }
}

impl PartialEq<BooleanUnderlyingType> for BooleanType {
    fn eq(&self, other: &BooleanUnderlyingType) -> bool {
        self.value == *other
    }
}

impl PartialEq<bool> for BooleanType {
    fn eq(&self, other: &bool) -> bool {
        let val = self.get_as_bool();

        if let Some(val) = val {
            return val == *other
        }

        // If value is null they can't be equal
        false
    }
}

impl PartialEq<Option<bool>> for BooleanType {
    fn eq(&self, other: &Option<bool>) -> bool {
        let val = self.get_as_bool();

        val.eq(other)
    }
}

impl PartialOrd for BooleanType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl PartialOrd<Value> for BooleanType {
    fn partial_cmp(&self, other: &Value) -> Option<Ordering> {
        let other_type_id = other.get_db_type_id();
        assert!(Self::TYPE.check_comparable(&other_type_id));

        match other.get_value() {
            DBTypeIdImpl::BOOLEAN(rhs) => self.partial_cmp(rhs),
            // TODO - add var char
            _ => unreachable!()
        }
    }
}

impl Eq for BooleanType {}

impl Ord for BooleanType {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value.cmp(&other.value)
    }
}

impl ComparisonDBTypeTrait for BooleanType {
    fn is_zero(&self) -> bool {
        panic!("is_zero is not available for boolean")
    }

    fn get_min_value() -> Self {
        panic!("get_min_value is not available for boolean")
    }

    fn get_max_value() -> Self {
        panic!("get_max_value is not available for boolean")
    }

    // TODO - this is not the same as the value
    fn is_null(&self) -> bool {
        self.value == Self::NULL
    }
}
