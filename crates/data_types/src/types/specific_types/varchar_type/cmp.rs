use crate::{partial_eq_null, run_on_numeric_impl, BigIntType, BigIntUnderlyingType, BooleanType, ComparisonDBTypeTrait, ConversionDBTypeTrait, DBTypeIdImpl, DecimalType, DecimalUnderlyingType, FormatDBTypeTrait, IntType, SmallIntType, TimestampType, TinyIntType, Value, VarcharType, VarcharUnderlyingType, BUSTUB_VALUE_NULL};
use std::cmp::Ordering;

impl PartialEq for VarcharType {
    fn eq(&self, other: &Self) -> bool {
        partial_eq_null!(self.is_null(), other.is_null());

        self.value == other.value
    }
}

impl PartialEq<TimestampType> for VarcharType {
    fn eq(&self, other: &TimestampType) -> bool {
        let other: VarcharType = other.into();

        self.eq(&other)
    }
}

impl PartialEq<DecimalType> for VarcharType {
    fn eq(&self, other: &DecimalType) -> bool {
        let other: VarcharType = other.into();

        self.eq(&other)
    }
}

impl PartialEq<BigIntType> for VarcharType {
    fn eq(&self, other: &BigIntType) -> bool {
        let other: VarcharType = other.into();

        self.eq(&other)
    }
}

impl PartialEq<IntType> for VarcharType {
    fn eq(&self, other: &IntType) -> bool {
        let other: VarcharType = other.into();

        self.eq(&other)
    }
}

impl PartialEq<SmallIntType> for VarcharType {
    fn eq(&self, other: &SmallIntType) -> bool {
        let other: VarcharType = other.into();

        self.eq(&other)
    }
}

impl PartialEq<TinyIntType> for VarcharType {
    fn eq(&self, other: &TinyIntType) -> bool {
        let other: VarcharType = other.into();

        self.eq(&other)
    }
}

impl PartialEq<BooleanType> for VarcharType {
    fn eq(&self, other: &BooleanType) -> bool {
        let other: VarcharType = other.into();

        self.eq(&other)
    }
}

impl PartialEq<Value> for VarcharType {
    fn eq(&self, other: &Value) -> bool {
        let other: VarcharType = other.into();

        self.eq(&other)
    }
}

impl PartialEq<Option<VarcharUnderlyingType>> for VarcharType {
    fn eq(&self, other: &Option<VarcharUnderlyingType>) -> bool {
        if let Some(other) = other {
            self.is_null() == false && other == self.value
        } else {
            self.is_null()
        }
    }
}

impl PartialEq<Option<&str>> for VarcharType {
    fn eq(&self, other: &Option<&str>) -> bool {
        if let Some(other) = other {
            self.is_null() == false && other == self.value.as_str()
        } else {
            self.is_null()
        }
    }
}

impl PartialOrd for VarcharType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.is_null() || other.is_null() {
            return self.len.partial_cmp(other.len)
        }
        self.value.partial_cmp(&other.value)
    }
}

impl PartialOrd<BigIntType> for VarcharType {
    fn partial_cmp(&self, other: &BigIntType) -> Option<Ordering> {
        let other: VarcharType = other.into();

        // This will compare nulls
        self.partial_cmp(&other)
    }
}

impl PartialOrd<IntType> for VarcharType {
    fn partial_cmp(&self, other: &IntType) -> Option<Ordering> {
        let other: VarcharType = other.into();

        // This will compare nulls
        self.partial_cmp(&other)
    }
}

impl PartialOrd<DecimalType> for VarcharType {
    fn partial_cmp(&self, other: &DecimalType) -> Option<Ordering> {
        let other: VarcharType = other.into();

        // This will compare nulls
        self.partial_cmp(&other)
    }
}

impl PartialOrd<SmallIntType> for VarcharType {
    fn partial_cmp(&self, other: &SmallIntType) -> Option<Ordering> {
        let other: VarcharType = other.into();

        // This will compare nulls
        self.partial_cmp(&other)
    }
}

impl PartialOrd<TinyIntType> for VarcharType {
    fn partial_cmp(&self, other: &TinyIntType) -> Option<Ordering> {
        let other: VarcharType = other.into();

        // This will compare nulls
        self.partial_cmp(&other)
    }
}

impl PartialOrd<BooleanType> for VarcharType {
    fn partial_cmp(&self, other: &BooleanType) -> Option<Ordering> {
        let other: VarcharType = other.into();

        // This will compare nulls
        self.partial_cmp(&other)
    }
}

impl PartialOrd<TimestampType> for VarcharType {
    fn partial_cmp(&self, other: &TimestampType) -> Option<Ordering> {
        let other: VarcharType = other.into();

        // This will compare nulls
        self.partial_cmp(&other)
    }
}

impl PartialOrd<Value> for VarcharType {
    fn partial_cmp(&self, other: &Value) -> Option<Ordering> {
        let other: VarcharType = other.into();

        // This will compare nulls
        self.partial_cmp(&other)
    }
}

impl Eq for VarcharType {}

impl Ord for VarcharType {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.is_null() || other.is_null() {
            return self.len.cmp(other.len)
        }
        self.value.cmp(&other.value)
    }
}

impl ComparisonDBTypeTrait for VarcharType {
    fn is_zero(&self) -> bool {
        unreachable!()
    }

    fn get_min_value() -> Self {
        unreachable!()
    }

    fn get_max_value() -> Self {
        unreachable!()
    }

    fn is_null(&self) -> bool {
        // When changing this we should also update serialize_to and `serialize_to` and `deserialize_from`
        self.len == BUSTUB_VALUE_NULL
    }
}
