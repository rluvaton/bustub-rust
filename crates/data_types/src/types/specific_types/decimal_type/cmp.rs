use crate::{partial_eq_null, run_on_numeric_impl, BigIntType, ComparisonDBTypeTrait, DBTypeIdImpl, DecimalType, DecimalUnderlyingType, FormatDBTypeTrait, IntType, SmallIntType, TinyIntType, Value};
use std::cmp::Ordering;

impl PartialEq for DecimalType {
    fn eq(&self, other: &Self) -> bool {
        partial_eq_null!(self.is_null(), other.is_null());

        self.0 == other.0
    }
}

impl PartialEq<BigIntType> for DecimalType {
    fn eq(&self, other: &BigIntType) -> bool {
        partial_eq_null!(self.is_null(), other.is_null());

        self.0 == other.0 as DecimalUnderlyingType
    }
}

impl PartialEq<IntType> for DecimalType {
    fn eq(&self, other: &IntType) -> bool {
        partial_eq_null!(self.is_null(), other.is_null());

        self.0 == other.0 as DecimalUnderlyingType
    }
}

impl PartialEq<SmallIntType> for DecimalType {
    fn eq(&self, other: &SmallIntType) -> bool {
        partial_eq_null!(self.is_null(), other.is_null());

        self.0 == other.0 as DecimalUnderlyingType
    }
}

impl PartialEq<TinyIntType> for DecimalType {
    fn eq(&self, other: &TinyIntType) -> bool {
        partial_eq_null!(self.is_null(), other.is_null());

        self.0 == other.0 as DecimalUnderlyingType
    }
}

impl PartialEq<Value> for DecimalType {
    fn eq(&self, other: &Value) -> bool {
        let other_type_id = other.get_db_type_id();
        assert!(Self::TYPE.check_comparable(&other_type_id));

        // TODO - add varchar support

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

impl PartialEq<DecimalUnderlyingType> for DecimalType {
    fn eq(&self, other: &DecimalUnderlyingType) -> bool {
        self.0 == *other
    }
}

impl PartialOrd for DecimalType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.is_null() && other.is_null() {
            return Some(Ordering::Equal);
        }

        self.0.partial_cmp(&other.0)
    }
}

impl PartialOrd<BigIntType> for DecimalType {
    fn partial_cmp(&self, other: &BigIntType) -> Option<Ordering> {
        if self.is_null() && other.is_null() {
            return Some(Ordering::Equal);
        }

        self.0.partial_cmp(&(other.0 as DecimalUnderlyingType))
    }
}

impl PartialOrd<IntType> for DecimalType {
    fn partial_cmp(&self, other: &IntType) -> Option<Ordering> {
        if self.is_null() && other.is_null() {
            return Some(Ordering::Equal);
        }

        self.0.partial_cmp(&(other.0 as DecimalUnderlyingType))
    }
}

impl PartialOrd<SmallIntType> for DecimalType {
    fn partial_cmp(&self, other: &SmallIntType) -> Option<Ordering> {
        if self.is_null() && other.is_null() {
            return Some(Ordering::Equal);
        }

        self.0.partial_cmp(&(other.0 as DecimalUnderlyingType))
    }
}

impl PartialOrd<TinyIntType> for DecimalType {
    fn partial_cmp(&self, other: &TinyIntType) -> Option<Ordering> {
        if self.is_null() && other.is_null() {
            return Some(Ordering::Equal);
        }

        self.0.partial_cmp(&(other.0 as DecimalUnderlyingType))
    }
}

impl PartialOrd<Value> for DecimalType {
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

impl PartialOrd<DecimalUnderlyingType> for DecimalType {
    fn partial_cmp(&self, other: &DecimalUnderlyingType) -> Option<Ordering> {
        self.0.partial_cmp(other)
    }
}

impl Eq for DecimalType {}

impl Ord for DecimalType {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.0 < other.0 {
            Ordering::Less
        } else if self.0 == other.0 {
            Ordering::Equal
        } else {
            Ordering::Greater
        }
    }
}

impl ComparisonDBTypeTrait for DecimalType {
    fn is_zero(&self) -> bool {
        self.0 == 0.0
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
