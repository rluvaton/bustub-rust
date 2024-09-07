use std::ops::{Add, Div, Mul, Rem, Sub};
use anyhow::anyhow;
use crate::run_on_numeric_impl;
use crate::types::{ArithmeticsDBTypeTrait, SmallIntType, ComparisonDBTypeTrait, DBTypeId, DBTypeIdImpl, FormatDBTypeTrait, Value, BUSTUB_I32_NULL, BigIntType, BigIntUnderlyingType, BUSTUB_I16_NULL, IntType, IntUnderlyingType, TinyIntType, SmallIntUnderlyingType, DecimalType, DecimalUnderlyingType};

impl Add for SmallIntType {
    type Output = SmallIntType;

    fn add(self, rhs: Self) -> Self::Output {
        SmallIntType::new(self.value + rhs.value)
    }
}

impl Add<DecimalType> for SmallIntType {
    type Output = DecimalType;

    fn add(self, rhs: DecimalType) -> Self::Output {
        DecimalType::new(self.value as DecimalUnderlyingType + rhs.value)
    }
}

impl Add<BigIntType> for SmallIntType {
    type Output = BigIntType;

    fn add(self, rhs: BigIntType) -> Self::Output {
        BigIntType::new(self.value as BigIntUnderlyingType + rhs.value)
    }
}

impl Add<IntType> for SmallIntType {
    type Output = IntType;

    fn add(self, rhs: IntType) -> Self::Output {
        IntType::new(self.value as IntUnderlyingType + rhs.value)
    }
}

impl Add<TinyIntType> for SmallIntType {
    type Output = SmallIntType;

    fn add(self, rhs: TinyIntType) -> Self::Output {
        SmallIntType::new(self.value + rhs.value as SmallIntUnderlyingType)
    }
}

impl Add<Value> for SmallIntType {
    type Output = Value;

    fn add(self, rhs: Value) -> Self::Output {
        let other_type_id = rhs.get_db_type_id();
        assert!(Self::TYPE.check_comparable(&other_type_id));

        Value::new(
            run_on_numeric_impl!(
                rhs.get_value(),
                rhs, (self + *rhs).into(),
                _ => unreachable!()
            )
        )
    }
}

impl Sub for SmallIntType {
    type Output = SmallIntType;

    fn sub(self, rhs: Self) -> Self::Output {
        SmallIntType::new(self.value - rhs.value)
    }
}

impl Sub<DecimalType> for SmallIntType {
    type Output = DecimalType;

    fn sub(self, rhs: DecimalType) -> Self::Output {
        DecimalType::new(self.value as DecimalUnderlyingType - rhs.value)
    }
}

impl Sub<BigIntType> for SmallIntType {
    type Output = BigIntType;

    fn sub(self, rhs: BigIntType) -> Self::Output {
        BigIntType::new(self.value as BigIntUnderlyingType - rhs.value)
    }
}

impl Sub<IntType> for SmallIntType {
    type Output = IntType;

    fn sub(self, rhs: IntType) -> Self::Output {
        IntType::new(self.value as IntUnderlyingType - rhs.value)
    }
}

impl Sub<TinyIntType> for SmallIntType {
    type Output = SmallIntType;

    fn sub(self, rhs: TinyIntType) -> Self::Output {
        SmallIntType::new(self.value - rhs.value as SmallIntUnderlyingType)
    }
}

impl Sub<Value> for SmallIntType {
    type Output = Value;

    fn sub(self, rhs: Value) -> Self::Output {
        let other_type_id = rhs.get_db_type_id();
        assert!(Self::TYPE.check_comparable(&other_type_id));

        Value::new(
            run_on_numeric_impl!(
                rhs.get_value(),
                rhs, (self - *rhs).into(),
                _ => unreachable!()
            )
        )
    }
}

impl Mul for SmallIntType {
    type Output = SmallIntType;

    fn mul(self, rhs: Self) -> Self::Output {
        SmallIntType::new(self.value * rhs.value)
    }
}

impl Mul<DecimalType> for SmallIntType {
    type Output = DecimalType;

    fn mul(self, rhs: DecimalType) -> Self::Output {
        DecimalType::new(self.value as DecimalUnderlyingType * rhs.value)
    }
}

impl Mul<BigIntType> for SmallIntType {
    type Output = BigIntType;

    fn mul(self, rhs: BigIntType) -> Self::Output {
        BigIntType::new(self.value as BigIntUnderlyingType * rhs.value)
    }
}

impl Mul<IntType> for SmallIntType {
    type Output = IntType;

    fn mul(self, rhs: IntType) -> Self::Output {
        IntType::new(self.value as IntUnderlyingType * rhs.value)
    }
}

impl Mul<TinyIntType> for SmallIntType {
    type Output = SmallIntType;

    fn mul(self, rhs: TinyIntType) -> Self::Output {
        SmallIntType::new(self.value * rhs.value as SmallIntUnderlyingType)
    }
}

impl Mul<Value> for SmallIntType {
    type Output = Value;

    fn mul(self, rhs: Value) -> Self::Output {
        let other_type_id = rhs.get_db_type_id();
        assert!(Self::TYPE.check_comparable(&other_type_id));


        Value::new(
            run_on_numeric_impl!(
                rhs.get_value(),
                rhs, (self * (*rhs)).into(),
                _ => unreachable!()
            )
        )
    }
}

impl Div for SmallIntType {
    type Output = SmallIntType;

    fn div(self, rhs: Self) -> Self::Output {
        SmallIntType::new(self.value / rhs.value)
    }
}

impl Div<DecimalType> for SmallIntType {
    type Output = DecimalType;

    fn div(self, rhs: DecimalType) -> Self::Output {
        DecimalType::new(self.value as DecimalUnderlyingType / rhs.value)
    }
}

impl Div<BigIntType> for SmallIntType {
    type Output = BigIntType;

    fn div(self, rhs: BigIntType) -> Self::Output {
        BigIntType::new(self.value as BigIntUnderlyingType / rhs.value)
    }
}

impl Div<IntType> for SmallIntType {
    type Output = IntType;

    fn div(self, rhs: IntType) -> Self::Output {
        IntType::new(self.value as IntUnderlyingType / rhs.value)
    }
}

impl Div<TinyIntType> for SmallIntType {
    type Output = SmallIntType;

    fn div(self, rhs: TinyIntType) -> Self::Output {
        SmallIntType::new(self.value / rhs.value as SmallIntUnderlyingType)
    }
}

impl Div<Value> for SmallIntType {
    type Output = Value;

    fn div(self, rhs: Value) -> Self::Output {
        let other_type_id = rhs.get_db_type_id();
        assert!(Self::TYPE.check_comparable(&other_type_id));

        if self.is_zero() && rhs.is_zero() {
            panic!("Division by zero on right-hand side");
        }

        Value::new(
            run_on_numeric_impl!(
                rhs.get_value(),
                rhs, (self / *rhs).into(),
                _ => unreachable!()
            )
        )
    }
}

impl Rem for SmallIntType {
    type Output = SmallIntType;

    fn rem(self, rhs: Self) -> Self::Output {
        SmallIntType::new(self.value % rhs.value)
    }
}

impl Rem<DecimalType> for SmallIntType {
    type Output = DecimalType;

    fn rem(self, rhs: DecimalType) -> Self::Output {
        DecimalType::new(self.value as DecimalUnderlyingType % rhs.value)
    }
}

impl Rem<BigIntType> for SmallIntType {
    type Output = BigIntType;

    fn rem(self, rhs: BigIntType) -> Self::Output {
        BigIntType::new(self.value as BigIntUnderlyingType % rhs.value)
    }
}

impl Rem<IntType> for SmallIntType {
    type Output = IntType;

    fn rem(self, rhs: IntType) -> Self::Output {
        IntType::new(self.value as IntUnderlyingType % rhs.value)
    }
}

impl Rem<TinyIntType> for SmallIntType {
    type Output = SmallIntType;

    fn rem(self, rhs: TinyIntType) -> Self::Output {
        SmallIntType::new(self.value / rhs.value as SmallIntUnderlyingType)
    }
}

impl Rem<Value> for SmallIntType {
    type Output = Value;

    fn rem(self, rhs: Value) -> Self::Output {
        let other_type_id = rhs.get_db_type_id();
        assert!(Self::TYPE.check_comparable(&other_type_id));

        Value::new(
            run_on_numeric_impl!(
                rhs.get_value(),
                rhs, (self % *rhs).into(),
                _ => unreachable!()
            )
        )
    }
}

impl ArithmeticsDBTypeTrait for SmallIntType {
    fn operate_null(&self, rhs: &Value) -> anyhow::Result<Value> {
        match rhs.get_db_type_id() {
            DBTypeId::TINYINT => Ok(Value::new(TinyIntType::default().into())),
            DBTypeId::SMALLINT => Ok(Value::new(SmallIntType::default().into())),
            DBTypeId::INT => Ok(Value::new(IntType::default().into())),
            DBTypeId::BIGINT => Ok(Value::new(BigIntType::default().into())),
            DBTypeId::DECIMAL => Ok(Value::new(DecimalType::default().into())),
            _ => Err(anyhow!("Type error"))
        }
    }
}
