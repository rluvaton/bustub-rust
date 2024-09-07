use crate::types::{BigIntType, BigIntUnderlyingType, BooleanType, ComparisonDBTypeTrait, DBTypeIdImpl, DecimalType, DecimalUnderlyingType, FormatDBTypeTrait, IntType, IntUnderlyingType, SmallIntType, TinyIntType, Value, BUSTUB_DECIMAL_MAX, BUSTUB_DECIMAL_MIN, BUSTUB_DECIMAL_NULL, BUSTUB_I16_MAX, BUSTUB_I16_MIN, BUSTUB_I16_NULL, BUSTUB_I32_MAX, BUSTUB_I32_MIN, BUSTUB_I32_NULL};
use std::cmp::Ordering;
use crate::run_on_numeric_impl;

impl PartialEq for DecimalType {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl PartialEq<BigIntType> for DecimalType {
    fn eq(&self, other: &BigIntType) -> bool {
        self.value == other.value as DecimalUnderlyingType
    }
}

impl PartialEq<IntType> for DecimalType {
    fn eq(&self, other: &IntType) -> bool {
        self.value == other.value as DecimalUnderlyingType
    }
}

impl PartialEq<SmallIntType> for DecimalType {
    fn eq(&self, other: &SmallIntType) -> bool {
        self.value == other.value as DecimalUnderlyingType
    }
}

impl PartialEq<TinyIntType> for DecimalType {
    fn eq(&self, other: &TinyIntType) -> bool {
        self.value == other.value as DecimalUnderlyingType
    }
}

impl PartialEq<Value> for DecimalType {
    fn eq(&self, other: &Value) -> bool {
        let other_type_id = other.get_db_type_id();
        assert!(Self::TYPE.check_comparable(&other_type_id));

        run_on_numeric_impl!(
            other.get_value(),
            rhs, self.eq(rhs),
            _ => unreachable!()
        )
        //
        // match other_type_id {
        //     DBTypeId::TINYINT => {
        //         todo!()
        //     }
        //     DBTypeId::SMALLINT => {
        //         todo!()
        //     }
        //     DBTypeId::INTEGER => {
        //         todo!()
        //     }
        //     DBTypeId::BIGINT => unsafe {
        //         self.value.eq(&other.get_as_bigint_unchecked().value)
        //     },
        //     DBTypeId::DECIMAL => {
        //         todo!()
        //     }
        //     DBTypeId::VARCHAR => unsafe {
        //         let r_value = other.try_cast_as(DBTypeId::BIGINT).expect("Should be able to change to bigint");
        //
        //         self.value.eq(&r_value.get_as_bigint_unchecked().value)
        //     }
        //     // TODO - panic?
        //     _ => panic!("Type error")
        // }
    }
}

impl PartialEq<DecimalUnderlyingType> for DecimalType {
    fn eq(&self, other: &DecimalUnderlyingType) -> bool {
        self.value == *other
    }
}

impl PartialOrd for DecimalType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl PartialOrd<BigIntType> for DecimalType {
    fn partial_cmp(&self, other: &BigIntType) -> Option<Ordering> {
        self.value.partial_cmp(&(other.value as DecimalUnderlyingType))
    }
}

impl PartialOrd<IntType> for DecimalType {
    fn partial_cmp(&self, other: &IntType) -> Option<Ordering> {
        self.value.partial_cmp(&(other.value as DecimalUnderlyingType))
    }
}

impl PartialOrd<SmallIntType> for DecimalType {
    fn partial_cmp(&self, other: &SmallIntType) -> Option<Ordering> {
        self.value.partial_cmp(&(other.value as DecimalUnderlyingType))
    }
}

impl PartialOrd<TinyIntType> for DecimalType {
    fn partial_cmp(&self, other: &TinyIntType) -> Option<Ordering> {
        self.value.partial_cmp(&(other.value as DecimalUnderlyingType))
    }
}

impl PartialOrd<Value> for DecimalType {
    fn partial_cmp(&self, other: &Value) -> Option<Ordering> {
        let other_type_id = other.get_db_type_id();
        assert!(Self::TYPE.check_comparable(&other_type_id));

        run_on_numeric_impl!(
            other.get_value(),
            rhs, self.partial_cmp(rhs),

            _ => unreachable!()
        )

        //
        // match other_type_id {
        //     DBTypeId::TINYINT => {
        //         todo!()
        //     }
        //     DBTypeId::SMALLINT => {
        //         todo!()
        //     }
        //     DBTypeId::INTEGER => {
        //         todo!()
        //     }
        //     DBTypeId::BIGINT => unsafe {
        //         self.value.partial_cmp(&other.get_as_bigint_unchecked().value)
        //     },
        //     DBTypeId::DECIMAL => {
        //         todo!()
        //     }
        //     DBTypeId::VARCHAR => unsafe {
        //         let r_value = other.try_cast_as(DBTypeId::BIGINT).ok()?;
        //
        //         self.value.partial_cmp(&r_value.get_as_bigint_unchecked().value)
        //     }
        //     // TODO - panic?
        //     _ => None
        // }
    }
}

impl PartialOrd<DecimalUnderlyingType> for DecimalType {
    fn partial_cmp(&self, other: &DecimalUnderlyingType) -> Option<Ordering> {
        self.value.partial_cmp(other)
    }
}

impl Eq for DecimalType {}

impl Ord for DecimalType {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.value < other.value {
            Ordering::Less
        } else if self.value == other.value {
            Ordering::Equal
        } else {
            Ordering::Greater
        }
    }
}

impl ComparisonDBTypeTrait for DecimalType {
    fn is_zero(&self) -> bool {
        self.value == 0.0
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
