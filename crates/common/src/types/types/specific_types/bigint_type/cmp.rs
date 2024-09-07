use crate::types::{BigIntType, ComparisonDBTypeTrait, DBTypeIdImpl, FormatDBTypeTrait, IntType, SmallIntType, Value, BUSTUB_I64_MAX, BUSTUB_I64_MIN, BUSTUB_I64_NULL};
use std::cmp::Ordering;
use crate::run_on_numeric_impl;
use super::BigIntUnderlyingType;

impl PartialEq for BigIntType {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl PartialEq<Value> for BigIntType {
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

impl PartialEq<IntType> for BigIntType {
    fn eq(&self, other: &IntType) -> bool {
        self.value == other.value as BigIntUnderlyingType
    }
}

impl PartialEq<SmallIntType> for BigIntType {
    fn eq(&self, other: &SmallIntType) -> bool {
        self.value == other.value as BigIntUnderlyingType
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
        self.value.partial_cmp(&(other.value as BigIntUnderlyingType))
    }
}

impl PartialOrd<SmallIntType> for BigIntType {
    fn partial_cmp(&self, other: &SmallIntType) -> Option<Ordering> {
        self.value.partial_cmp(&(other.value as BigIntUnderlyingType))
    }
}

impl PartialOrd<Value> for BigIntType {
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

impl PartialOrd<BigIntUnderlyingType> for BigIntType {
    fn partial_cmp(&self, other: &BigIntUnderlyingType) -> Option<Ordering> {
        self.value.partial_cmp(other)
    }
}

impl ComparisonDBTypeTrait for BigIntType {
    fn is_zero(&self) -> bool {
        self.value == 0
    }

    fn get_min_value() -> Self {
        Self::new(BUSTUB_I64_MIN)
    }

    fn get_max_value() -> Self {
        Self::new(BUSTUB_I64_MAX)
    }

    // TODO - this is not the same as the value
    fn is_null(&self) -> bool {
        self.value == BUSTUB_I64_NULL
    }
}
