use crate::types::{SmallIntType, ComparisonDBTypeTrait, DBTypeIdImpl, FormatDBTypeTrait, Value, BUSTUB_I64_MAX, BUSTUB_I64_MIN, BUSTUB_I64_NULL, BUSTUB_I32_MIN, BUSTUB_I32_MAX, BUSTUB_I32_NULL, BUSTUB_I16_NULL, BUSTUB_I16_MIN, BUSTUB_I16_MAX, BigIntType, BigIntUnderlyingType, IntType, IntUnderlyingType, TinyIntType, DecimalType, DecimalUnderlyingType};
use std::cmp::Ordering;
use crate::run_on_numeric_impl;
use super::SmallIntUnderlyingType;

impl PartialEq for SmallIntType {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl PartialEq<DecimalType> for SmallIntType {
    fn eq(&self, other: &DecimalType) -> bool {
        self.value as DecimalUnderlyingType == other.value
    }
}

impl PartialEq<BigIntType> for SmallIntType {
    fn eq(&self, other: &BigIntType) -> bool {
        self.value as BigIntUnderlyingType == other.value
    }
}

impl PartialEq<IntType> for SmallIntType {
    fn eq(&self, other: &IntType) -> bool {
        self.value as IntUnderlyingType == other.value
    }
}

impl PartialEq<TinyIntType> for SmallIntType {
    fn eq(&self, other: &TinyIntType) -> bool {
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

impl PartialEq<SmallIntUnderlyingType> for SmallIntType {
    fn eq(&self, other: &SmallIntUnderlyingType) -> bool {
        self.value == *other
    }
}

impl PartialOrd for SmallIntType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl PartialOrd<DecimalType> for SmallIntType {
    fn partial_cmp(&self, other: &DecimalType) -> Option<Ordering> {
        (self.value as DecimalUnderlyingType).partial_cmp(&other.value)
    }
}

impl PartialOrd<BigIntType> for SmallIntType {
    fn partial_cmp(&self, other: &BigIntType) -> Option<Ordering> {
        (self.value as BigIntUnderlyingType).partial_cmp(&other.value)
    }
}

impl PartialOrd<IntType> for SmallIntType {
    fn partial_cmp(&self, other: &IntType) -> Option<Ordering> {
        (self.value as IntUnderlyingType).partial_cmp(&other.value)
    }
}

impl PartialOrd<TinyIntType> for SmallIntType {
    fn partial_cmp(&self, other: &TinyIntType) -> Option<Ordering> {
        self.value.partial_cmp(&(other.value as SmallIntUnderlyingType))
    }
}

impl PartialOrd<Value> for SmallIntType {
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

impl PartialOrd<SmallIntUnderlyingType> for SmallIntType {
    fn partial_cmp(&self, other: &SmallIntUnderlyingType) -> Option<Ordering> {
        self.value.partial_cmp(other)
    }
}

impl Eq for SmallIntType {}

impl Ord for SmallIntType {
    fn cmp(&self, other: &Self) -> Ordering {
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
