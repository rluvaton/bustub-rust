use crate::{ArithmeticsDBTypeTrait, DBTypeIdImpl, Value};
use crate::run_on_impl;
use std::ops::{Add, AddAssign, Div, Mul, Rem, Sub};

impl Add for Value {
    type Output = Value;

    fn add(self, rhs: Self) -> Self::Output {
        run_on_impl!(self.value, lhs, lhs + rhs)
    }
}

impl AddAssign for Value {
    fn add_assign(&mut self, rhs: Self) {
        // TODO - remove clone
        *self = run_on_impl!(self.clone().value, lhs, lhs + rhs)
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
    fn operate_null(&self, rhs: &Value) -> error_utils::anyhow::Result<Value> {
        run_on_impl!(&self.value, lhs, lhs.operate_null(rhs))
    }
}
