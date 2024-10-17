use crate::{partial_eq_null, run_on_numeric_impl, BigIntType, BigIntUnderlyingType, ComparisonDBTypeTrait, DBTypeIdImpl, DecimalType, DecimalUnderlyingType, FormatDBTypeTrait, IntType, IntUnderlyingType, SmallIntType, SmallIntUnderlyingType, TinyIntType, TinyIntUnderlyingType, Value};
use std::cmp::Ordering;

impl PartialEq for TinyIntType {
    fn eq(&self, other: &Self) -> bool {
        partial_eq_null!(self.is_null(), other.is_null());

        self.value == other.value
    }
}

impl PartialEq<DecimalType> for TinyIntType {
    fn eq(&self, other: &DecimalType) -> bool {
        partial_eq_null!(self.is_null(), other.is_null());

        self.value as DecimalUnderlyingType == other.value
    }
}

impl PartialEq<BigIntType> for TinyIntType {
    fn eq(&self, other: &BigIntType) -> bool {
        partial_eq_null!(self.is_null(), other.is_null());

        self.value as BigIntUnderlyingType == other.value
    }
}

impl PartialEq<IntType> for TinyIntType {
    fn eq(&self, other: &IntType) -> bool {
        partial_eq_null!(self.is_null(), other.is_null());

        self.value as IntUnderlyingType == other.value
    }
}

impl PartialEq<SmallIntType> for TinyIntType {
    fn eq(&self, other: &SmallIntType) -> bool {
        partial_eq_null!(self.is_null(), other.is_null());

        self.value as SmallIntUnderlyingType == other.value
    }
}

impl PartialEq<Value> for TinyIntType {
    fn eq(&self, other: &Value) -> bool {
        let other_type_id = other.get_db_type_id();
        assert!(Self::TYPE.check_comparable(&other_type_id));

        if self.is_null() && other.is_null() {
            return true;
        }

        // TODO - support varchar

        run_on_numeric_impl!(
            other.get_value(),
            rhs, self.eq(rhs),
            _ => unreachable!()
        )
    }
}

impl PartialEq<TinyIntUnderlyingType> for TinyIntType {
    fn eq(&self, other: &TinyIntUnderlyingType) -> bool {
        self.value == *other
    }
}

impl PartialOrd for TinyIntType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.is_null() && other.is_null() {
            return Some(Ordering::Equal);
        }

        self.value.partial_cmp(&other.value)
    }
}

impl PartialOrd<BigIntType> for TinyIntType {
    fn partial_cmp(&self, other: &BigIntType) -> Option<Ordering> {
        if self.is_null() && other.is_null() {
            return Some(Ordering::Equal);
        }

        (self.value as BigIntUnderlyingType).partial_cmp(&other.value)
    }
}

impl PartialOrd<DecimalType> for TinyIntType {
    fn partial_cmp(&self, other: &DecimalType) -> Option<Ordering> {
        if self.is_null() && other.is_null() {
            return Some(Ordering::Equal);
        }

        (self.value as DecimalUnderlyingType).partial_cmp(&other.value)
    }
}

impl PartialOrd<IntType> for TinyIntType {
    fn partial_cmp(&self, other: &IntType) -> Option<Ordering> {
        if self.is_null() && other.is_null() {
            return Some(Ordering::Equal);
        }

        (self.value as IntUnderlyingType).partial_cmp(&other.value)
    }
}

impl PartialOrd<SmallIntType> for TinyIntType {
    fn partial_cmp(&self, other: &SmallIntType) -> Option<Ordering> {
        if self.is_null() && other.is_null() {
            return Some(Ordering::Equal);
        }

        (self.value as SmallIntUnderlyingType).partial_cmp(&other.value)
    }
}

impl PartialOrd<Value> for TinyIntType {
    fn partial_cmp(&self, other: &Value) -> Option<Ordering> {
        let other_type_id = other.get_db_type_id();
        assert!(Self::TYPE.check_comparable(&other_type_id));

        if self.is_null() && other.is_null() {
            return Some(Ordering::Equal);
        }

        run_on_numeric_impl!(
            other.get_value(),
            rhs, self.partial_cmp(rhs),

            // TODO - support varchar
            _ => unreachable!()
        )
    }
}

impl PartialOrd<TinyIntUnderlyingType> for TinyIntType {
    fn partial_cmp(&self, other: &TinyIntUnderlyingType) -> Option<Ordering> {
        self.value.partial_cmp(other)
    }
}

impl Eq for TinyIntType {}

impl Ord for TinyIntType {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.is_null() && other.is_null() {
            return Ordering::Equal;
        }

        self.value.cmp(&other.value)
    }
}

impl ComparisonDBTypeTrait for TinyIntType {
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
