use crate::{ArithmeticsDBTypeTrait, BooleanType, Value};
use anyhow::anyhow;
use std::ops::{Add, Div, Mul, Rem, Sub};

impl Add for BooleanType {
    type Output = BooleanType;

    fn add(self, _rhs: BooleanType) -> Self::Output {
        panic!("Add is not available for boolean")
    }
}

impl Add<Value> for BooleanType {
    type Output = Value;

    fn add(self, _rhs: Value) -> Self::Output {
        panic!("Add is not available for boolean")
    }
}

impl Sub for BooleanType {
    type Output = BooleanType;

    fn sub(self, _rhs: BooleanType) -> Self::Output {
        panic!("Sub is not available for boolean")
    }
}

impl Sub<Value> for BooleanType {
    type Output = Value;

    fn sub(self, _rhs: Value) -> Self::Output {
        panic!("Sub is not available for boolean")
    }
}

impl Mul for BooleanType {
    type Output = BooleanType;

    fn mul(self, _rhs: Self) -> Self::Output {
        panic!("Mul is not available for boolean")
    }
}

impl Mul<Value> for BooleanType {
    type Output = Value;

    fn mul(self, _rhs: Value) -> Self::Output {
        panic!("Mul is not available for boolean")
    }
}

impl Div for BooleanType {
    type Output = BooleanType;

    fn div(self, _rhs: Self) -> Self::Output {
        panic!("Div is not available for boolean")
    }
}

impl Div<Value> for BooleanType {
    type Output = Value;

    fn div(self, _rhs: Value) -> Self::Output {
        panic!("Div is not available for boolean")
    }
}

impl Rem for BooleanType {
    type Output = BooleanType;

    fn rem(self, _rhs: Self) -> Self::Output {
        panic!("Rem is not available for boolean")
    }
}

impl Rem<Value> for BooleanType {
    type Output = Value;

    fn rem(self, _rhs: Value) -> Self::Output {
        panic!("Rem is not available for boolean")
    }
}

impl ArithmeticsDBTypeTrait for BooleanType {
    fn operate_null(&self, rhs: &Value) -> anyhow::Result<Value> {
        Err(anyhow!("operate null on boolean is not available"))
    }
}
