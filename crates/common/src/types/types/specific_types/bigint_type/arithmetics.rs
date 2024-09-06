use std::ops::{Add, Div, Mul, Rem, Sub};
use anyhow::anyhow;
use crate::types::{ArithmeticsDBTypeTrait, BigIntType, DBTypeId, Value, BUSTUB_I64_NULL};
impl Add for BigIntType {
    type Output = BigIntType;

    fn add(self, rhs: Self) -> Self::Output {
        BigIntType::new(self.value + rhs.value)
    }
}

impl Add<Value> for BigIntType {
    type Output = Value;

    fn add(self, rhs: Value) -> Self::Output {
        todo!()
    }
}

impl Sub for BigIntType {
    type Output = BigIntType;

    fn sub(self, rhs: Self) -> Self::Output {
        BigIntType::new(self.value - rhs.value)
    }
}

impl Sub<Value> for BigIntType {
    type Output = Value;

    fn sub(self, rhs: Value) -> Self::Output {
        todo!()
    }
}

impl Mul for BigIntType {
    type Output = BigIntType;

    fn mul(self, rhs: Self) -> Self::Output {
        BigIntType::new(self.value * rhs.value)
    }
}

impl Mul<Value> for BigIntType {
    type Output = Value;

    fn mul(self, rhs: Value) -> Self::Output {
        todo!()
    }
}

impl Div for BigIntType {
    type Output = BigIntType;

    fn div(self, rhs: Self) -> Self::Output {
        BigIntType::new(self.value / rhs.value)
    }
}

impl Div<Value> for BigIntType {
    type Output = Value;

    fn div(self, rhs: Value) -> Self::Output {
        todo!()
    }
}

impl Rem for BigIntType {
    type Output = BigIntType;

    fn rem(self, rhs: Self) -> Self::Output {
        BigIntType::new(self.value % rhs.value)
    }
}

impl Rem<Value> for BigIntType {
    type Output = Value;

    fn rem(self, rhs: Value) -> Self::Output {
        todo!()
    }
}

impl ArithmeticsDBTypeTrait for BigIntType {
    fn operate_null(&self, rhs: &Value) -> anyhow::Result<Value> {
        match rhs.get_db_type_id() {
            DBTypeId::TINYINT | DBTypeId::SMALLINT | DBTypeId::INTEGER | DBTypeId::BIGINT => {
                Ok(Value::new(Self::new(BUSTUB_I64_NULL).into()))
            }
            DBTypeId::DECIMAL => {
                // Ok(Value::new(DecimalType::new(BUSTUB_INT64_NULL)))
                todo!()
            }
            _ => Err(anyhow!("Type error"))
        }
    }
}
