use crate::{ArithmeticsDBTypeTrait, BigIntType, DBTypeId, DecimalType, IntType, SmallIntType, TinyIntType, Value, VarcharType};
use error_utils::anyhow::anyhow;
use std::ops::{Add, Div, Mul, Rem, Sub};

impl Add for VarcharType {
    type Output = VarcharType;

    fn add(self, _rhs: Self) -> Self::Output {
        unreachable!("String concatenation is not supported at the moment, and this should be blocked in the planing phase");
    }
}

impl Add<Value> for VarcharType {
    type Output = Value;

    fn add(self, _rhs: Value) -> Self::Output {
        unreachable!("String concatenation is not supported at the moment, and this should be blocked in the planing phase");
    }
}

impl Sub for VarcharType {
    type Output = VarcharType;

    fn sub(self, _rhs: Self) -> Self::Output {
        unreachable!("Cant subtract strings and this should be blocked in the planing phase");
    }
}

impl Sub<Value> for VarcharType {
    type Output = Value;

    fn sub(self, _rhs: Value) -> Self::Output {
        unreachable!("Cant subtract varchar with anything and this should be blocked in the planing phase");

    }
}

impl Mul for VarcharType {
    type Output = BigIntType;

    fn mul(self, _rhs: Self) -> Self::Output {
        unreachable!("Cant multiply strings and this should be blocked in the planing phase");
    }
}

impl Mul<Value> for VarcharType {
    type Output = Value;

    fn mul(self, _rhs: Value) -> Self::Output {
        unreachable!("Cant multiply with varchar and this should be blocked in the planing phase");
    }
}

impl Div for VarcharType {
    type Output = VarcharType;

    fn div(self, _rhs: Self) -> Self::Output {
        unreachable!("Cant divide strings and this should be blocked in the planing phase");
    }
}

impl Div<Value> for VarcharType {
    type Output = Value;

    fn div(self, _rhs: Value) -> Self::Output {
        unreachable!("Cant divide with varchar and this should be blocked in the planing phase");
    }
}

impl Rem for VarcharType {
    type Output = VarcharType;

    fn rem(self, _rhs: Self) -> Self::Output {
        unreachable!("Cant do module with varchar and this should be blocked in the planing phase");
    }
}

impl Rem<Value> for VarcharType {
    type Output = Value;

    fn rem(self, _rhs: Value) -> Self::Output {
        unreachable!("Cant do module with varchar and this should be blocked in the planing phase");

    }
}

impl ArithmeticsDBTypeTrait for VarcharType {
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
