use crate::{run_on_numeric_impl, BigIntType, BigIntUnderlyingType, ComparisonDBTypeTrait, DBTypeIdImpl, DecimalType, DecimalUnderlyingType, FormatDBTypeTrait, IntType, SmallIntType, TinyIntType, Value};
use std::cmp::Ordering;

impl PartialEq for BigIntType {
    fn eq(&self, other: &Self) -> bool {
        if self.is_null() && other.is_null() {
            return true;
        }

        self.value == other.value
    }
}

impl PartialEq<DecimalType> for BigIntType {
    fn eq(&self, other: &DecimalType) -> bool {

        if self.is_null() && other.is_null() {
            return true;
        }

        self.value as DecimalUnderlyingType == other.value
    }
}

impl PartialEq<IntType> for BigIntType {
    fn eq(&self, other: &IntType) -> bool {

        if self.is_null() && other.is_null() {
            return true;
        }

        self.value == other.value as BigIntUnderlyingType
    }
}

impl PartialEq<SmallIntType> for BigIntType {
    fn eq(&self, other: &SmallIntType) -> bool {

        if self.is_null() && other.is_null() {
            return true;
        }

        self.value == other.value as BigIntUnderlyingType
    }
}

impl PartialEq<TinyIntType> for BigIntType {
    fn eq(&self, other: &TinyIntType) -> bool {

        if self.is_null() && other.is_null() {
            return true;
        }

        self.value == other.value as BigIntUnderlyingType
    }
}

impl PartialEq<Value> for BigIntType {
    fn eq(&self, other: &Value) -> bool {
        let other_type_id = other.get_db_type_id();
        assert!(Self::TYPE.check_comparable(&other_type_id));

        if self.is_null() && other.is_null() {
            return true;
        }

        run_on_numeric_impl!(
            other.get_value(),
            rhs, self.eq(rhs),
            _ => unreachable!()
        )
    }
}

impl PartialEq<BigIntUnderlyingType> for BigIntType {
    fn eq(&self, other: &BigIntUnderlyingType) -> bool {
        self.value == *other
    }
}

impl PartialOrd for BigIntType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl PartialOrd<IntType> for BigIntType {
    fn partial_cmp(&self, other: &IntType) -> Option<Ordering> {
        if self.is_null() && other.is_null() {
            return Some(Ordering::Equal);
        }

        self.value.partial_cmp(&(other.value as BigIntUnderlyingType))
    }
}

impl PartialOrd<DecimalType> for BigIntType {
    fn partial_cmp(&self, other: &DecimalType) -> Option<Ordering> {
        if self.is_null() && other.is_null() {
            return Some(Ordering::Equal);
        }

        (self.value as DecimalUnderlyingType).partial_cmp(&other.value)
    }
}

impl PartialOrd<SmallIntType> for BigIntType {
    fn partial_cmp(&self, other: &SmallIntType) -> Option<Ordering> {
        if self.is_null() && other.is_null() {
            return Some(Ordering::Equal);
        }

        self.value.partial_cmp(&(other.value as BigIntUnderlyingType))
    }
}

impl PartialOrd<TinyIntType> for BigIntType {
    fn partial_cmp(&self, other: &TinyIntType) -> Option<Ordering> {
        if self.is_null() && other.is_null() {
            return Some(Ordering::Equal);
        }

        self.value.partial_cmp(&(other.value as BigIntUnderlyingType))
    }
}

impl PartialOrd<Value> for BigIntType {
    fn partial_cmp(&self, other: &Value) -> Option<Ordering> {
        let other_type_id = other.get_db_type_id();
        assert!(Self::TYPE.check_comparable(&other_type_id));

        if self.is_null() && other.is_null() {
            return Some(Ordering::Equal);
        }

        // TODO - support var char

        run_on_numeric_impl!(
            other.get_value(),
            rhs, self.partial_cmp(rhs),
            _ => unreachable!()
        )
    }
}

impl PartialOrd<BigIntUnderlyingType> for BigIntType {
    fn partial_cmp(&self, other: &BigIntUnderlyingType) -> Option<Ordering> {
        self.value.partial_cmp(other)
    }
}

impl Eq for BigIntType {}

impl Ord for BigIntType {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.is_null() && other.is_null() {
            return Ordering::Equal;
        }

        self.value.cmp(&other.value)
    }
}

impl ComparisonDBTypeTrait for BigIntType {
    fn is_zero(&self) -> bool {
        self.value == 0
    }

    fn get_min_value() -> Self {
        Self::new(Self::MIN)
    }

    fn get_max_value() -> Self {
        Self::new(Self::MAX)
    }

    // TODO - this is not the same as the value
    fn is_null(&self) -> bool {
        self.value == Self::NULL
    }
}
