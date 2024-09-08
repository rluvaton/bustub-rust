use crate::types::{ArithmeticsDBTypeTrait, DBTypeIdImpl, Value};
use crate::run_on_impl;
use std::ops::{Add, Div, Mul, Rem, Sub};

impl Add for Value {
    type Output = Value;

    fn add(self, rhs: Self) -> Self::Output {
        run_on_impl!(self.value, lhs, lhs + rhs)
    }
}

impl Sub for Value {
    type Output = Value;

    fn sub(self, rhs: Self) -> Self::Output {
        run_on_impl!(self.value, lhs, lhs - rhs)
    }
}

impl Mul for Value {
    type Output = Value;

    fn mul(self, rhs: Self) -> Self::Output {
        run_on_impl!(self.value, lhs, lhs * rhs)
    }
}

impl Div for Value {
    type Output = Value;

    fn div(self, rhs: Self) -> Self::Output {
        run_on_impl!(self.value, lhs, lhs / rhs)
    }
}

impl Rem for Value {
    type Output = Value;

    fn rem(self, rhs: Self) -> Self::Output {
        run_on_impl!(self.value, lhs, lhs % rhs)
    }
}

impl ArithmeticsDBTypeTrait for Value {
    fn operate_null(&self, rhs: &Value) -> anyhow::Result<Value> {
        run_on_impl!(self.value, lhs, lhs.operate_null(rhs))
    }
}
