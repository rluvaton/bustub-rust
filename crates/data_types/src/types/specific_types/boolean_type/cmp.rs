use crate::{partial_eq_null, BooleanType, BooleanUnderlyingType, ComparisonDBTypeTrait, DBTypeIdImpl, FormatDBTypeTrait, Value};
use std::cmp::Ordering;

impl PartialEq for BooleanType {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl PartialEq<Value> for BooleanType {
    fn eq(&self, other: &Value) -> bool {
        let other_type_id = other.get_db_type_id();
        assert!(Self::TYPE.check_comparable(&other_type_id));

        match other.get_value() {
            DBTypeIdImpl::BOOLEAN(rhs) => self.eq(rhs),
            // TODO - add var char
            _ => {
                // Only doing null check here as it will already be checked inside eq
                partial_eq_null!(self.is_null(), other.is_null());

                unreachable!()
            }
        }
    }
}

impl PartialEq<BooleanUnderlyingType> for BooleanType {
    fn eq(&self, other: &BooleanUnderlyingType) -> bool {
        self.0 == *other
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

        if self.is_null() && other.is_null() {
            return Some(Ordering::Equal);
        }

        self.0.partial_cmp(&other.0)
    }
}

impl PartialOrd<Value> for BooleanType {
    fn partial_cmp(&self, other: &Value) -> Option<Ordering> {
        let other_type_id = other.get_db_type_id();
        assert!(Self::TYPE.check_comparable(&other_type_id));

        if self.is_null() && other.is_null() {
            return Some(Ordering::Equal);
        }

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
        if self.is_null() && other.is_null() {
            return Ordering::Equal;
        }

        self.0.cmp(&other.0)
    }
}

impl ComparisonDBTypeTrait for BooleanType {
    fn is_zero(&self) -> bool {
        panic!("is_zero is not available for boolean")
    }

    fn get_min_value() -> Self {
        Self::new(Self::FALSE)
    }

    fn get_max_value() -> Self {
        Self::new(Self::TRUE)
    }

    // TODO - this is not the same as the value
    fn is_null(&self) -> bool {
        self.0 == Self::NULL
    }
}
