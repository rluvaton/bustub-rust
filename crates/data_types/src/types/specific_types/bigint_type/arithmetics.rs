use std::ops::{Add, Div, Mul, Rem, Sub};
use anyhow::anyhow;
use crate::{run_on_numeric_impl, ArithmeticsDBTypeTrait, BigIntType, ComparisonDBTypeTrait, DBTypeId, DBTypeIdImpl, DecimalType, DecimalUnderlyingType, FormatDBTypeTrait, IntType, SmallIntType, TinyIntType, Value, BigIntUnderlyingType};

impl Add for BigIntType {
    type Output = BigIntType;

    fn add(self, rhs: Self) -> Self::Output {
        BigIntType::new(self.value + rhs.value)
    }
}

impl Add<DecimalType> for BigIntType {
    type Output = DecimalType;

    fn add(self, rhs: DecimalType) -> Self::Output {
        DecimalType::new(self.value as DecimalUnderlyingType + rhs.value)
    }
}

impl Add<IntType> for BigIntType {
    type Output = BigIntType;

    fn add(self, rhs: IntType) -> Self::Output {
        BigIntType::new(self.value + rhs.value as BigIntUnderlyingType)
    }
}

impl Add<SmallIntType> for BigIntType {
    type Output = BigIntType;

    fn add(self, rhs: SmallIntType) -> Self::Output {
        BigIntType::new(self.value + rhs.value as BigIntUnderlyingType)
    }
}

impl Add<TinyIntType> for BigIntType {
    type Output = BigIntType;

    fn add(self, rhs: TinyIntType) -> Self::Output {
        BigIntType::new(self.value + rhs.value as BigIntUnderlyingType)
    }
}

impl Add<Value> for BigIntType {
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

impl Sub for BigIntType {
    type Output = BigIntType;

    fn sub(self, rhs: Self) -> Self::Output {
        BigIntType::new(self.value - rhs.value)
    }
}

impl Sub<DecimalType> for BigIntType {
    type Output = DecimalType;

    fn sub(self, rhs: DecimalType) -> Self::Output {
        DecimalType::new(self.value as DecimalUnderlyingType - rhs.value)
    }
}

impl Sub<IntType> for BigIntType {
    type Output = BigIntType;

    fn sub(self, rhs: IntType) -> Self::Output {
        BigIntType::new(self.value - rhs.value as BigIntUnderlyingType)
    }
}

impl Sub<SmallIntType> for BigIntType {
    type Output = BigIntType;

    fn sub(self, rhs: SmallIntType) -> Self::Output {
        BigIntType::new(self.value - rhs.value as BigIntUnderlyingType)
    }
}

impl Sub<TinyIntType> for BigIntType {
    type Output = BigIntType;

    fn sub(self, rhs: TinyIntType) -> Self::Output {
        BigIntType::new(self.value - rhs.value as BigIntUnderlyingType)
    }
}

impl Sub<Value> for BigIntType {
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

impl Mul for BigIntType {
    type Output = BigIntType;

    fn mul(self, rhs: Self) -> Self::Output {
        BigIntType::new(self.value * rhs.value)
    }
}

impl Mul<DecimalType> for BigIntType {
    type Output = DecimalType;

    fn mul(self, rhs: DecimalType) -> Self::Output {
        DecimalType::new(self.value as DecimalUnderlyingType * rhs.value)
    }
}

impl Mul<IntType> for BigIntType {
    type Output = BigIntType;

    fn mul(self, rhs: IntType) -> Self::Output {
        BigIntType::new(self.value * rhs.value as BigIntUnderlyingType)
    }
}

impl Mul<SmallIntType> for BigIntType {
    type Output = BigIntType;

    fn mul(self, rhs: SmallIntType) -> Self::Output {
        BigIntType::new(self.value * rhs.value as BigIntUnderlyingType)
    }
}

impl Mul<TinyIntType> for BigIntType {
    type Output = BigIntType;

    fn mul(self, rhs: TinyIntType) -> Self::Output {
        BigIntType::new(self.value * rhs.value as BigIntUnderlyingType)
    }
}

impl Mul<Value> for BigIntType {
    type Output = Value;

    fn mul(self, rhs: Value) -> Self::Output {
        let other_type_id = rhs.get_db_type_id();
        assert!(Self::TYPE.check_comparable(&other_type_id));

        Value::new(
            run_on_numeric_impl!(
                rhs.get_value(),
                rhs, (self * *rhs).into(),
                    _ => unreachable!()
            )
        )
    }
}

impl Div for BigIntType {
    type Output = BigIntType;

    fn div(self, rhs: Self) -> Self::Output {
        BigIntType::new(self.value / rhs.value)
    }
}

impl Div<DecimalType> for BigIntType {
    type Output = DecimalType;

    fn div(self, rhs: DecimalType) -> Self::Output {
        DecimalType::new(self.value as DecimalUnderlyingType / rhs.value)
    }
}

impl Div<IntType> for BigIntType {
    type Output = BigIntType;

    fn div(self, rhs: IntType) -> Self::Output {
        BigIntType::new(self.value / rhs.value as BigIntUnderlyingType)
    }
}

impl Div<SmallIntType> for BigIntType {
    type Output = BigIntType;

    fn div(self, rhs: SmallIntType) -> Self::Output {
        BigIntType::new(self.value / rhs.value as BigIntUnderlyingType)
    }
}

impl Div<TinyIntType> for BigIntType {
    type Output = BigIntType;

    fn div(self, rhs: TinyIntType) -> Self::Output {
        BigIntType::new(self.value / rhs.value as BigIntUnderlyingType)
    }
}

impl Div<Value> for BigIntType {
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

impl Rem for BigIntType {
    type Output = BigIntType;

    fn rem(self, rhs: Self) -> Self::Output {
        BigIntType::new(self.value % rhs.value)
    }
}

// Decimal

impl Rem<DecimalType> for BigIntType {
    type Output = DecimalType;

    fn rem(self, rhs: DecimalType) -> Self::Output {
        DecimalType::new(self.value as DecimalUnderlyingType % rhs.value)
    }
}

impl Rem<IntType> for BigIntType {
    type Output = BigIntType;

    fn rem(self, rhs: IntType) -> Self::Output {
        BigIntType::new(self.value % rhs.value as BigIntUnderlyingType)
    }
}

impl Rem<SmallIntType> for BigIntType {
    type Output = BigIntType;

    fn rem(self, rhs: SmallIntType) -> Self::Output {
        BigIntType::new(self.value % rhs.value as BigIntUnderlyingType)
    }
}

impl Rem<TinyIntType> for BigIntType {
    type Output = BigIntType;

    fn rem(self, rhs: TinyIntType) -> Self::Output {
        BigIntType::new(self.value % rhs.value as BigIntUnderlyingType)
    }
}

impl Rem<Value> for BigIntType {
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

impl ArithmeticsDBTypeTrait for BigIntType {
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
