use crate::{partial_eq_null, run_on_numeric_impl, BigIntType, BigIntUnderlyingType, ComparisonDBTypeTrait, DBTypeIdImpl, DecimalType, DecimalUnderlyingType, FormatDBTypeTrait, IntType, SmallIntType, TinyIntType, Value};
use std::cmp::Ordering;

impl PartialEq for BigIntType {
    fn eq(&self, other: &Self) -> bool {
        partial_eq_null!(self.is_null(), other.is_null());

        self.0 == other.0
    }
}

impl PartialEq<DecimalType> for BigIntType {
    fn eq(&self, other: &DecimalType) -> bool {
        partial_eq_null!(self.is_null(), other.is_null());

        self.0 as DecimalUnderlyingType == other.0
    }
}

impl PartialEq<IntType> for BigIntType {
    fn eq(&self, other: &IntType) -> bool {
        partial_eq_null!(self.is_null(), other.is_null());

        self.0 == other.0 as BigIntUnderlyingType
    }
}

impl PartialEq<SmallIntType> for BigIntType {
    fn eq(&self, other: &SmallIntType) -> bool {
        partial_eq_null!(self.is_null(), other.is_null());

        self.0 == other.0 as BigIntUnderlyingType
    }
}

impl PartialEq<TinyIntType> for BigIntType {
    fn eq(&self, other: &TinyIntType) -> bool {
        partial_eq_null!(self.is_null(), other.is_null());

        self.0 == other.0 as BigIntUnderlyingType
    }
}

impl PartialEq<Value> for BigIntType {
    fn eq(&self, other: &Value) -> bool {
        let other_type_id = other.get_db_type_id();
        assert!(Self::TYPE.check_comparable(&other_type_id));

        run_on_numeric_impl!(
            other.get_value(),
            rhs, self.eq(rhs),
            _ => {
                // Only doing null check here as it will already be checked inside eq
                partial_eq_null!(self.is_null(), other.is_null());

                // If not nulls
                unreachable!()
            }
        )
    }
}

impl PartialEq<BigIntUnderlyingType> for BigIntType {
    fn eq(&self, other: &BigIntUnderlyingType) -> bool {
        self.0 == *other
    }
}

impl PartialOrd for BigIntType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl PartialOrd<IntType> for BigIntType {
    fn partial_cmp(&self, other: &IntType) -> Option<Ordering> {
        if self.is_null() && other.is_null() {
            return Some(Ordering::Equal);
        }

        self.0.partial_cmp(&(other.0 as BigIntUnderlyingType))
    }
}

impl PartialOrd<DecimalType> for BigIntType {
    fn partial_cmp(&self, other: &DecimalType) -> Option<Ordering> {
        if self.is_null() && other.is_null() {
            return Some(Ordering::Equal);
        }

        (self.0 as DecimalUnderlyingType).partial_cmp(&other.0)
    }
}

impl PartialOrd<SmallIntType> for BigIntType {
    fn partial_cmp(&self, other: &SmallIntType) -> Option<Ordering> {
        if self.is_null() && other.is_null() {
            return Some(Ordering::Equal);
        }

        self.0.partial_cmp(&(other.0 as BigIntUnderlyingType))
    }
}

impl PartialOrd<TinyIntType> for BigIntType {
    fn partial_cmp(&self, other: &TinyIntType) -> Option<Ordering> {
        if self.is_null() && other.is_null() {
            return Some(Ordering::Equal);
        }

        self.0.partial_cmp(&(other.0 as BigIntUnderlyingType))
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
        self.0.partial_cmp(other)
    }
}

impl Eq for BigIntType {}

impl Ord for BigIntType {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.is_null() && other.is_null() {
            return Ordering::Equal;
        }

        self.0.cmp(&other.0)
    }
}

impl ComparisonDBTypeTrait for BigIntType {
    fn is_zero(&self) -> bool {
        self.0 == 0
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
