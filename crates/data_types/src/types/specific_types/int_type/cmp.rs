use crate::{run_on_numeric_impl, BigIntType, BigIntUnderlyingType, ComparisonDBTypeTrait, DBTypeIdImpl, DecimalType, DecimalUnderlyingType, FormatDBTypeTrait, IntType, IntUnderlyingType, SmallIntType, TinyIntType, Value};
use std::cmp::Ordering;

impl PartialEq for IntType {
    fn eq(&self, other: &Self) -> bool {
        if self.is_null() && other.is_null() {
            return true;
        }

        self.value == other.value
    }
}

impl PartialEq<DecimalType> for IntType {
    fn eq(&self, other: &DecimalType) -> bool {
        if self.is_null() && other.is_null() {
            return true;
        }

        self.value as DecimalUnderlyingType == other.value
    }
}

impl PartialEq<BigIntType> for IntType {
    fn eq(&self, other: &BigIntType) -> bool {
        if self.is_null() && other.is_null() {
            return true;
        }

        self.value as BigIntUnderlyingType == other.value
    }
}

impl PartialEq<SmallIntType> for IntType {
    fn eq(&self, other: &SmallIntType) -> bool {
        if self.is_null() && other.is_null() {
            return true;
        }

        self.value == other.value as IntUnderlyingType
    }
}

impl PartialEq<TinyIntType> for IntType {
    fn eq(&self, other: &TinyIntType) -> bool {
        if self.is_null() && other.is_null() {
            return true;
        }

        self.value == other.value as IntUnderlyingType
    }
}

impl PartialEq<Value> for IntType {
    fn eq(&self, other: &Value) -> bool {
        let other_type_id = other.get_db_type_id();
        assert!(Self::TYPE.check_comparable(&other_type_id));

        if self.is_null() && other.is_null() {
            return true;
        }

        run_on_numeric_impl!(
            other.get_value(),
            rhs, self.eq(rhs),
            // TODO - add support for varchar
            _ => unreachable!()
        )
    }
}

impl PartialEq<IntUnderlyingType> for IntType {
    fn eq(&self, other: &IntUnderlyingType) -> bool {
        self.value == *other
    }
}

impl PartialOrd for IntType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.is_null() && other.is_null() {
            return Some(Ordering::Equal);
        }

        self.value.partial_cmp(&other.value)
    }
}

impl PartialOrd<DecimalType> for IntType {
    fn partial_cmp(&self, other: &DecimalType) -> Option<Ordering> {
        if self.is_null() && other.is_null() {
            return Some(Ordering::Equal);
        }

        (self.value as DecimalUnderlyingType).partial_cmp(&other.value)
    }
}

impl PartialOrd<BigIntType> for IntType {
    fn partial_cmp(&self, other: &BigIntType) -> Option<Ordering> {
        if self.is_null() && other.is_null() {
            return Some(Ordering::Equal);
        }

        (self.value as BigIntUnderlyingType).partial_cmp(&other.value)
    }
}

impl PartialOrd<SmallIntType> for IntType {
    fn partial_cmp(&self, other: &SmallIntType) -> Option<Ordering> {
        if self.is_null() && other.is_null() {
            return Some(Ordering::Equal);
        }

        self.value.partial_cmp(&(other.value as IntUnderlyingType))
    }
}

impl PartialOrd<TinyIntType> for IntType {
    fn partial_cmp(&self, other: &TinyIntType) -> Option<Ordering> {
        if self.is_null() && other.is_null() {
            return Some(Ordering::Equal);
        }

        self.value.partial_cmp(&(other.value as IntUnderlyingType))
    }
}

impl PartialOrd<Value> for IntType {
    fn partial_cmp(&self, other: &Value) -> Option<Ordering> {
        let other_type_id = other.get_db_type_id();
        assert!(Self::TYPE.check_comparable(&other_type_id));

        if self.is_null() && other.is_null() {
            return Some(Ordering::Equal);
        }

        // TODO - support varchar
        run_on_numeric_impl!(
            other.get_value(),
            rhs, self.partial_cmp(rhs),
            _ => unreachable!()
        )
    }
}

impl PartialOrd<IntUnderlyingType> for IntType {
    fn partial_cmp(&self, other: &IntUnderlyingType) -> Option<Ordering> {
        self.value.partial_cmp(other)
    }
}

impl Eq for IntType {}

impl Ord for IntType {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.is_null() && other.is_null() {
            return Ordering::Equal;
        }

        self.value.cmp(&other.value)
    }
}

impl ComparisonDBTypeTrait for IntType {
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
