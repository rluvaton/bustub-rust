use crate::{ArithmeticsDBTypeTrait, TimestampType, Value};
use error_utils::anyhow::anyhow;
use std::ops::{Add, Div, Mul, Rem, Sub};

impl Add for TimestampType {
    type Output = TimestampType;

    fn add(self, _rhs: TimestampType) -> Self::Output {
        panic!("Add is not available for timestamp")
    }
}

impl Add<Value> for TimestampType {
    type Output = Value;

    fn add(self, _rhs: Value) -> Self::Output {
        panic!("Add is not available for timestamp")
    }
}

impl Sub for TimestampType {
    type Output = TimestampType;

    fn sub(self, _rhs: TimestampType) -> Self::Output {
        panic!("Sub is not available for timestamp")
    }
}

impl Sub<Value> for TimestampType {
    type Output = Value;

    fn sub(self, _rhs: Value) -> Self::Output {
        panic!("Sub is not available for timestamp")
    }
}

impl Mul for TimestampType {
    type Output = TimestampType;

    fn mul(self, _rhs: Self) -> Self::Output {
        panic!("Mul is not available for timestamp")
    }
}

impl Mul<Value> for TimestampType {
    type Output = Value;

    fn mul(self, _rhs: Value) -> Self::Output {
        panic!("Mul is not available for timestamp")
    }
}

impl Div for TimestampType {
    type Output = TimestampType;

    fn div(self, _rhs: Self) -> Self::Output {
        panic!("Div is not available for timestamp")
    }
}

impl Div<Value> for TimestampType {
    type Output = Value;

    fn div(self, _rhs: Value) -> Self::Output {
        panic!("Div is not available for timestamp")
    }
}

impl Rem for TimestampType {
    type Output = TimestampType;

    fn rem(self, _rhs: Self) -> Self::Output {
        panic!("Rem is not available for timestamp")
    }
}

impl Rem<Value> for TimestampType {
    type Output = Value;

    fn rem(self, _rhs: Value) -> Self::Output {
        panic!("Rem is not available for timestamp")
    }
}

impl ArithmeticsDBTypeTrait for TimestampType {
    fn operate_null(&self, _rhs: &Value) -> error_utils::anyhow::Result<Value> {
        Err(anyhow!("operate null on boolean is not timestamp"))
    }
}
