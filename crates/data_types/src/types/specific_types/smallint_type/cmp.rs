use crate::{partial_eq_null, run_on_numeric_impl, BigIntType, BigIntUnderlyingType, ComparisonDBTypeTrait, DBTypeIdImpl, DecimalType, DecimalUnderlyingType, FormatDBTypeTrait, IntType, IntUnderlyingType, SmallIntType, SmallIntUnderlyingType, TinyIntType, Value};
use std::cmp::Ordering;

impl PartialEq for SmallIntType {
    fn eq(&self, other: &Self) -> bool {
        partial_eq_null!(self.is_null(), other.is_null());

        self.value == other.value
    }
}

impl PartialEq<DecimalType> for SmallIntType {
    fn eq(&self, other: &DecimalType) -> bool {
        partial_eq_null!(self.is_null(), other.is_null());

        self.value as DecimalUnderlyingType == other.value
    }
}

impl PartialEq<BigIntType> for SmallIntType {
    fn eq(&self, other: &BigIntType) -> bool {
        if self.is_null() && other.is_null() {
            return true;
        }

        self.value as BigIntUnderlyingType == other.value
    }
}

impl PartialEq<IntType> for SmallIntType {
    fn eq(&self, other: &IntType) -> bool {
        partial_eq_null!(self.is_null(), other.is_null());

        self.value as IntUnderlyingType == other.value
    }
}

impl PartialEq<TinyIntType> for SmallIntType {
    fn eq(&self, other: &TinyIntType) -> bool {
        partial_eq_null!(self.is_null(), other.is_null());

        self.value == other.value as SmallIntUnderlyingType
    }
}

impl PartialEq<Value> for SmallIntType {
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

impl PartialEq<SmallIntUnderlyingType> for SmallIntType {
    fn eq(&self, other: &SmallIntUnderlyingType) -> bool {
        self.value == *other
    }
}

impl PartialOrd for SmallIntType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.is_null() && other.is_null() {
            return Some(Ordering::Equal);
        }

        self.value.partial_cmp(&other.value)
    }
}

impl PartialOrd<DecimalType> for SmallIntType {
    fn partial_cmp(&self, other: &DecimalType) -> Option<Ordering> {
        if self.is_null() && other.is_null() {
            return Some(Ordering::Equal);
        }

        (self.value as DecimalUnderlyingType).partial_cmp(&other.value)
    }
}

impl PartialOrd<BigIntType> for SmallIntType {
    fn partial_cmp(&self, other: &BigIntType) -> Option<Ordering> {
        if self.is_null() && other.is_null() {
            return Some(Ordering::Equal);
        }

        (self.value as BigIntUnderlyingType).partial_cmp(&other.value)
    }
}

impl PartialOrd<IntType> for SmallIntType {
    fn partial_cmp(&self, other: &IntType) -> Option<Ordering> {
        if self.is_null() && other.is_null() {
            return Some(Ordering::Equal);
        }

        (self.value as IntUnderlyingType).partial_cmp(&other.value)
    }
}

impl PartialOrd<TinyIntType> for SmallIntType {
    fn partial_cmp(&self, other: &TinyIntType) -> Option<Ordering> {
        if self.is_null() && other.is_null() {
            return Some(Ordering::Equal);
        }

        self.value.partial_cmp(&(other.value as SmallIntUnderlyingType))
    }
}

impl PartialOrd<Value> for SmallIntType {
    fn partial_cmp(&self, other: &Value) -> Option<Ordering> {
        let other_type_id = other.get_db_type_id();
        assert!(Self::TYPE.check_comparable(&other_type_id));

        if self.is_null() && other.is_null() {
            return Some(Ordering::Equal);
        }

        run_on_numeric_impl!(
            other.get_value(),
            rhs, self.partial_cmp(rhs),

            // TODO - support var char
            _ => unreachable!()
        )
    }
}

impl PartialOrd<SmallIntUnderlyingType> for SmallIntType {
    fn partial_cmp(&self, other: &SmallIntUnderlyingType) -> Option<Ordering> {
        self.value.partial_cmp(other)
    }
}

impl Eq for SmallIntType {}

impl Ord for SmallIntType {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.is_null() && other.is_null() {
            return Ordering::Equal;
        }

        self.value.cmp(&other.value)
    }
}

impl ComparisonDBTypeTrait for SmallIntType {
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
