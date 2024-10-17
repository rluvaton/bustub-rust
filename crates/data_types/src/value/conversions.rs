use crate::{BigIntType, BigIntUnderlyingType, BooleanType, DecimalType, DecimalUnderlyingType, IntType, IntUnderlyingType, SmallIntType, SmallIntUnderlyingType, TimestampType, TimestampUnderlyingType, TinyIntType, TinyIntUnderlyingType, Value, VarcharType};

// Varchar

impl From<&str> for Value {
    fn from(value: &str) -> Self {
        VarcharType::from(value).into()
    }
}

impl From<Option<&str>> for Value {
    fn from(value: Option<&str>) -> Self {
        VarcharType::from(value).into()
    }
}

impl From<String> for Value {
    fn from(value: String) -> Self {
        VarcharType::from(value).into()
    }
}

impl From<Option<String>> for Value {
    fn from(value: Option<String>) -> Self {
        VarcharType::from(value).into()
    }
}

// Boolean


impl From<bool> for Value {
    fn from(value: bool) -> Self {
        BooleanType::from(value).into()
    }
}

impl From<Option<bool>> for Value {
    fn from(value: Option<bool>) -> Self {
        BooleanType::from(value).into()
    }
}

impl From<&Option<bool>> for Value {
    fn from(value: &Option<bool>) -> Self {
        BooleanType::from(value).into()
    }
}

// Tinyint

impl From<TinyIntUnderlyingType> for Value {
    fn from(value: TinyIntUnderlyingType) -> Self {
        TinyIntType::from(value).into()
    }
}

impl From<Option<TinyIntUnderlyingType>> for Value {
    fn from(value: Option<TinyIntUnderlyingType>) -> Self {
        TinyIntType::from(value).into()
    }
}

impl From<&Option<TinyIntUnderlyingType>> for Value {
    fn from(value: &Option<TinyIntUnderlyingType>) -> Self {
        TinyIntType::from(value).into()
    }
}

// Smallint


impl From<SmallIntUnderlyingType> for Value {
    fn from(value: SmallIntUnderlyingType) -> Self {
        SmallIntType::from(value).into()
    }
}

impl From<Option<SmallIntUnderlyingType>> for Value {
    fn from(value: Option<SmallIntUnderlyingType>) -> Self {
        SmallIntType::from(value).into()
    }
}

impl From<&Option<SmallIntUnderlyingType>> for Value {
    fn from(value: &Option<SmallIntUnderlyingType>) -> Self {
        SmallIntType::from(value).into()
    }
}

// int

impl From<IntUnderlyingType> for Value {
    fn from(value: IntUnderlyingType) -> Self {
        IntType::from(value).into()
    }
}

impl From<Option<IntUnderlyingType>> for Value {
    fn from(value: Option<IntUnderlyingType>) -> Self {
        IntType::from(value).into()
    }
}

impl From<&Option<IntUnderlyingType>> for Value {
    fn from(value: &Option<IntUnderlyingType>) -> Self {
        IntType::from(value).into()
    }
}

// Bigint

impl From<BigIntUnderlyingType> for Value {
    fn from(value: BigIntUnderlyingType) -> Self {
        BigIntType::from(value).into()
    }
}

impl From<Option<BigIntUnderlyingType>> for Value {
    fn from(value: Option<BigIntUnderlyingType>) -> Self {
        BigIntType::from(value).into()
    }
}

impl From<&Option<BigIntUnderlyingType>> for Value {
    fn from(value: &Option<BigIntUnderlyingType>) -> Self {
        BigIntType::from(value).into()
    }
}

// Decimal


impl From<DecimalUnderlyingType> for Value {
    fn from(value: DecimalUnderlyingType) -> Self {
        DecimalType::from(value).into()
    }
}

impl From<Option<DecimalUnderlyingType>> for Value {
    fn from(value: Option<DecimalUnderlyingType>) -> Self {
        DecimalType::from(value).into()
    }
}

impl From<&Option<DecimalUnderlyingType>> for Value {
    fn from(value: &Option<DecimalUnderlyingType>) -> Self {
        DecimalType::from(value).into()
    }
}

// Timestamp


impl From<TimestampUnderlyingType> for Value {
    fn from(value: TimestampUnderlyingType) -> Self {
        TimestampType::from(value).into()
    }
}

impl From<Option<TimestampUnderlyingType>> for Value {
    fn from(value: Option<TimestampUnderlyingType>) -> Self {
        TimestampType::from(value).into()
    }
}

impl From<&Option<TimestampUnderlyingType>> for Value {
    fn from(value: &Option<TimestampUnderlyingType>) -> Self {
        TimestampType::from(value).into()
    }
}
