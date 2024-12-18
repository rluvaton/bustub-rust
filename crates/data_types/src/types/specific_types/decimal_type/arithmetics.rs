use crate::{run_on_numeric_impl, ArithmeticsDBTypeTrait, BigIntType, ComparisonDBTypeTrait, DBTypeId, DBTypeIdImpl, DecimalType, DecimalUnderlyingType, FormatDBTypeTrait, IntType, SmallIntType, TinyIntType, Value};
use error_utils::anyhow::anyhow;
use std::ops::{Add, Div, Mul, Rem, Sub};

impl Add for DecimalType {
    type Output = DecimalType;

    fn add(self, rhs: Self) -> Self::Output {
        DecimalType::new(self.0 + rhs.0)
    }
}

impl Add<BigIntType> for DecimalType {
    type Output = DecimalType;

    fn add(self, rhs: BigIntType) -> Self::Output {
        DecimalType::new(self.0 + rhs.0 as DecimalUnderlyingType)
    }
}

impl Add<IntType> for DecimalType {
    type Output = DecimalType;

    fn add(self, rhs: IntType) -> Self::Output {
        DecimalType::new(self.0 + rhs.0 as DecimalUnderlyingType)
    }
}

impl Add<SmallIntType> for DecimalType {
    type Output = DecimalType;

    fn add(self, rhs: SmallIntType) -> Self::Output {
        DecimalType::new(self.0 + rhs.0 as DecimalUnderlyingType)
    }
}

impl Add<TinyIntType> for DecimalType {
    type Output = DecimalType;

    fn add(self, rhs: TinyIntType) -> Self::Output {
        DecimalType::new(self.0 + rhs.0 as DecimalUnderlyingType)
    }
}

impl Add<Value> for DecimalType {
    type Output = Value;

    fn add(self, rhs: Value) -> Self::Output {
        let other_type_id = rhs.get_db_type_id();
        assert!(Self::TYPE.check_comparable(&other_type_id), "{} is not comparable to {}", Self::TYPE, other_type_id);

        Value::new(
            run_on_numeric_impl!(
                rhs.get_value(),
                rhs, (self + *rhs).into(),
                _ => unreachable!()
            )
        )
    }
}

impl Sub for DecimalType {
    type Output = DecimalType;

    fn sub(self, rhs: Self) -> Self::Output {
        DecimalType::new(self.0 - rhs.0)
    }
}

impl Sub<BigIntType> for DecimalType {
    type Output = DecimalType;

    fn sub(self, rhs: BigIntType) -> Self::Output {
        DecimalType::new(self.0 - rhs.0 as DecimalUnderlyingType)
    }
}

impl Sub<IntType> for DecimalType {
    type Output = DecimalType;

    fn sub(self, rhs: IntType) -> Self::Output {
        DecimalType::new(self.0 - rhs.0 as DecimalUnderlyingType)
    }
}

impl Sub<SmallIntType> for DecimalType {
    type Output = DecimalType;

    fn sub(self, rhs: SmallIntType) -> Self::Output {
        DecimalType::new(self.0 - rhs.0 as DecimalUnderlyingType)
    }
}

impl Sub<TinyIntType> for DecimalType {
    type Output = DecimalType;

    fn sub(self, rhs: TinyIntType) -> Self::Output {
        DecimalType::new(self.0 - rhs.0 as DecimalUnderlyingType)
    }
}

impl Sub<Value> for DecimalType {
    type Output = Value;

    fn sub(self, rhs: Value) -> Self::Output {
        let other_type_id = rhs.get_db_type_id();
        assert!(Self::TYPE.check_comparable(&other_type_id), "{} is not comparable to {}", Self::TYPE, other_type_id);

        Value::new(
            run_on_numeric_impl!(
                rhs.get_value(),
                rhs, (self - *rhs).into(),
                _ => unreachable!()
            )
        )
    }
}

impl Mul for DecimalType {
    type Output = DecimalType;

    fn mul(self, rhs: Self) -> Self::Output {
        DecimalType::new(self.0 * rhs.0)
    }
}

impl Mul<BigIntType> for DecimalType {
    type Output = DecimalType;

    fn mul(self, rhs: BigIntType) -> Self::Output {
        DecimalType::new(self.0 * rhs.0 as DecimalUnderlyingType)
    }
}

impl Mul<IntType> for DecimalType {
    type Output = DecimalType;

    fn mul(self, rhs: IntType) -> Self::Output {
        DecimalType::new(self.0 * rhs.0 as DecimalUnderlyingType)
    }
}

impl Mul<SmallIntType> for DecimalType {
    type Output = DecimalType;

    fn mul(self, rhs: SmallIntType) -> Self::Output {
        DecimalType::new(self.0 * rhs.0 as DecimalUnderlyingType)
    }
}

impl Mul<TinyIntType> for DecimalType {
    type Output = DecimalType;

    fn mul(self, rhs: TinyIntType) -> Self::Output {
        DecimalType::new(self.0 * rhs.0 as DecimalUnderlyingType)
    }
}

impl Mul<Value> for DecimalType {
    type Output = Value;

    fn mul(self, rhs: Value) -> Self::Output {
        let other_type_id = rhs.get_db_type_id();
        assert!(Self::TYPE.check_comparable(&other_type_id), "{} is not comparable to {}", Self::TYPE, other_type_id);

        Value::new(
            run_on_numeric_impl!(
                rhs.get_value(),
                rhs, (self * *rhs).into(),
                _ => unreachable!()
            )
        )
    }
}

impl Div for DecimalType {
    type Output = DecimalType;

    fn div(self, rhs: Self) -> Self::Output {
        DecimalType::new(self.0 / rhs.0)
    }
}

impl Div<BigIntType> for DecimalType {
    type Output = DecimalType;

    fn div(self, rhs: BigIntType) -> Self::Output {
        DecimalType::new(self.0 / rhs.0 as DecimalUnderlyingType)
    }
}

impl Div<IntType> for DecimalType {
    type Output = DecimalType;

    fn div(self, rhs: IntType) -> Self::Output {
        DecimalType::new(self.0 / rhs.0 as DecimalUnderlyingType)
    }
}

impl Div<SmallIntType> for DecimalType {
    type Output = DecimalType;

    fn div(self, rhs: SmallIntType) -> Self::Output {
        DecimalType::new(self.0 / rhs.0 as DecimalUnderlyingType)
    }
}

impl Div<TinyIntType> for DecimalType {
    type Output = DecimalType;

    fn div(self, rhs: TinyIntType) -> Self::Output {
        DecimalType::new(self.0 / rhs.0 as DecimalUnderlyingType)
    }
}

impl Div<Value> for DecimalType {
    type Output = Value;

    fn div(self, rhs: Value) -> Self::Output {
        let other_type_id = rhs.get_db_type_id();
        assert!(Self::TYPE.check_comparable(&other_type_id), "{} is not comparable to {}", Self::TYPE, other_type_id);

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

impl Rem for DecimalType {
    type Output = DecimalType;

    fn rem(self, rhs: Self) -> Self::Output {
        DecimalType::new(self.0 % rhs.0)
    }
}

impl Rem<BigIntType> for DecimalType {
    type Output = DecimalType;

    fn rem(self, rhs: BigIntType) -> Self::Output {
        DecimalType::new(self.0 % rhs.0 as DecimalUnderlyingType)
    }
}

impl Rem<IntType> for DecimalType {
    type Output = DecimalType;

    fn rem(self, rhs: IntType) -> Self::Output {
        DecimalType::new(self.0 % rhs.0 as DecimalUnderlyingType)
    }
}

impl Rem<SmallIntType> for DecimalType {
    type Output = DecimalType;

    fn rem(self, rhs: SmallIntType) -> Self::Output {
        DecimalType::new(self.0 % rhs.0 as DecimalUnderlyingType)
    }
}

impl Rem<TinyIntType> for DecimalType {
    type Output = DecimalType;

    fn rem(self, rhs: TinyIntType) -> Self::Output {
        DecimalType::new(self.0 % rhs.0 as DecimalUnderlyingType)
    }
}

impl Rem<Value> for DecimalType {
    type Output = Value;

    fn rem(self, rhs: Value) -> Self::Output {
        let other_type_id = rhs.get_db_type_id();
        assert!(Self::TYPE.check_comparable(&other_type_id), "{} is not comparable to {}", Self::TYPE, other_type_id);

        Value::new(
            run_on_numeric_impl!(
                rhs.get_value(),
                rhs, (self % *rhs).into(),
                _ => unreachable!()
            )
        )
    }
}

impl ArithmeticsDBTypeTrait for DecimalType {
    fn operate_null(&self, rhs: &Value) -> error_utils::anyhow::Result<Value> {
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
