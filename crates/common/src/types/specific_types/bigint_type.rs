use std::cmp::Ordering;
use std::fmt::{Debug, Display, Formatter};
use std::ops::{Add, Deref, Div, Mul, Rem, Sub};
use crate::types::{TypeId, TypeIdTrait, Value, BUSTUB_INT64_NULL, BUSTUB_VALUE_NULL};

#[derive(Copy, Debug)]
pub struct BigIntType {
    value: u64,
    len: u32,
}

impl BigIntType {
    pub fn new(value: u64) -> Self {
        BigIntType {
            value,
            len: if value == BUSTUB_INT64_NULL as u64 { BUSTUB_VALUE_NULL } else { 0 },
        }
    }
}

impl Deref for BigIntType {
    type Target = u64;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Clone for BigIntType {
    fn clone(&self) -> Self {
        BigIntType::new(self.value)
    }
}

impl Display for BigIntType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}


impl From<u64> for BigIntType {
    fn from(value: u64) -> Self {
        BigIntType::new(value)
    }
}

impl From<&[u8]> for BigIntType {
    fn from(value: &[u8]) -> Self {
        todo!()
    }
}

impl<'a> Into<&'a [u8]> for BigIntType {
    fn into(self) -> &'a [u8] {
        todo!()
    }
}

impl PartialEq for BigIntType {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl PartialEq<Value> for BigIntType {
    fn eq(&self, other: &Value) -> bool {
        todo!()
    }
}

impl PartialEq<u64> for BigIntType {
    fn eq(&self, other: &u64) -> bool {
        self.value == *other
    }
}

impl PartialOrd for BigIntType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl PartialOrd<Value> for BigIntType {
    fn partial_cmp(&self, other: &Value) -> Option<Ordering> {
        todo!()
    }
}

impl PartialOrd<u64> for BigIntType {
    fn partial_cmp(&self, other: &u64) -> Option<Ordering> {
        self.value.partial_cmp(other)
    }
}

impl Add for BigIntType {
    type Output = BigIntType;

    fn add(self, rhs: Self) -> Self::Output {
        BigIntType::new(self.value + rhs.value)
    }
}

impl Add<Value> for BigIntType {
    type Output = ();

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
    type Output = ();

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
    type Output = ();

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
    type Output = ();

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
    type Output = ();

    fn rem(self, rhs: Value) -> Self::Output {
        todo!()
    }
}

impl TypeIdTrait<'_> for BigIntType {
    fn get_type_id() -> TypeId {
        TypeId::BIGINT
    }

    fn operate_null(&self, rhs: &Self) -> Self {
        todo!()
    }

    fn is_zero(&self) -> bool {
        todo!()
    }

    fn is_inlined(&self) -> bool {
        todo!()
    }

    fn get_data(&self) -> &[u8] {
        todo!()
    }

    fn get_length(&self) -> u32 {
        todo!()
    }

    fn get_data_from_slice(storage: &[u8]) -> &[u8] {
        todo!()
    }

    fn to_string(&self) -> String {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use crate::types::BigIntType;

    #[test]
    fn basic_arithmetics_for_zero() {
        let numbers_0_to_100: [BigIntType; 101] = std::array::from_fn(|i| (i as u64).into());

        // Validate all the numbers are correct
        for i in 0..=100u64 {
            assert_eq!(numbers_0_to_100[i as usize], i);
        }

        // 0

        for index in 0..numbers_0_to_100.len() {
            let value = index as u64;

            // 0 + i;
            assert_eq!(numbers_0_to_100[0] + numbers_0_to_100[index], numbers_0_to_100[index]);
            assert_eq!(numbers_0_to_100[0] + numbers_0_to_100[index], value);

            // i + 0
            assert_eq!(numbers_0_to_100[index] + numbers_0_to_100[0], numbers_0_to_100[index]);
            assert_eq!(numbers_0_to_100[index] + numbers_0_to_100[0], value);

            // 0 * i
            assert_eq!(numbers_0_to_100[0] * numbers_0_to_100[index], numbers_0_to_100[0]);
            assert_eq!(numbers_0_to_100[0] * numbers_0_to_100[index], 0);

            // i * 0
            assert_eq!(numbers_0_to_100[index] * numbers_0_to_100[0], numbers_0_to_100[0]);
            assert_eq!(numbers_0_to_100[index] * numbers_0_to_100[0], 0);
        }
    }

    #[test]
    fn basic_arithmetics() {
        let numbers_1_to_100: [BigIntType; 100] = std::array::from_fn(|i| (i as u64 + 1).into());

        // Validate all the numbers are correct
        for i in 0..100u64 {
            assert_eq!(numbers_1_to_100[i as usize], i + 1);
        }

        for a_index in 0..numbers_1_to_100.len() {
            let a = numbers_1_to_100[a_index];
            let a_value = (a_index as u64) + 1;

            for b_index in 0..numbers_1_to_100.len() {
                let b = numbers_1_to_100[b_index];
                let b_value = b_index as u64 + 1;

                // a + b;
                assert_eq!((a + b).value, a_value + b_value);
                assert_eq!(a + b, BigIntType::new(a_value + b_value));

                // a * b;
                assert_eq!((a * b).value, a_value * b_value);
                assert_eq!(a * b, BigIntType::new(a_value * b_value));

                // a / b;
                assert_eq!((a / b).value, a_value / b_value);
                assert_eq!(a / b, BigIntType::new(a_value / b_value));

                // a % b
                assert_eq!((a % b).value, a_value % b_value);
                assert_eq!(a % b, BigIntType::new(a_value % b_value));
            }
        }
    }

    #[test]
    fn basic_cmp() {
        let numbers_0_to_100_u64: [u64; 101] = std::array::from_fn(|i| i as u64);
        let numbers_0_to_100: [BigIntType; 101] = std::array::from_fn(|i| (i as u64).into());

        // Make sure we created correctly
        for i in 0..=100 {
            assert_eq!(numbers_0_to_100[i].value, numbers_0_to_100_u64[i]);
        }

        for i in 0..=100 {
            // =
            assert_eq!(numbers_0_to_100[i], BigIntType::new(numbers_0_to_100_u64[i]));
        }

        for n in numbers_0_to_100 {
            // !=
            assert_ne!(n, BigIntType::new(200));
        }

        for n in numbers_0_to_100 {
            // <
            assert!(n < BigIntType::new(n.value + 1), "{} < {}", n.value, n.value + 1);

            assert_eq!(n < BigIntType::new(n.value), false, "{} should not be less than {}", n.value, n.value );
            if n.value > 0 {
                assert_eq!(n < BigIntType::new(n.value - 1), false, "{} should not be less than {}", n.value, n.value - 1);
            }
        }

        for n in numbers_0_to_100 {
            // <=
            assert!(n <= BigIntType::new(n.value), "{} <= {}", n.value, n.value);
            assert!(n <= BigIntType::new(n.value + 1), "{} <= {}", n.value, n.value + 1);

            if n.value > 0 {
                assert_eq!(n <= BigIntType::new(n.value - 1), false, "{} should not be less than or equal to {}", n.value, n.value - 1);
            }
        }

        for n in numbers_0_to_100 {
            // >
            if n.value > 0 {
                assert!(n > BigIntType::new(n.value - 1), "{} > {}", n.value, n.value - 1);
            }

            assert_eq!(n > BigIntType::new(n.value), false, "{} should not be greater than {}", n.value, n.value);
            assert_eq!(n > BigIntType::new(n.value + 1), false, "{} should not be greater than {}", n.value, n.value + 1);
        }

        for n in numbers_0_to_100 {
            // >=
            assert!(n >= BigIntType::new(n.value), "{} >= {}", n.value, n.value);

            if n.value > 0 {
                assert!(n >= BigIntType::new(n.value - 1), "{} >= {}", n.value, n.value - 1);
            }

            assert_eq!(n >= BigIntType::new(n.value + 1), false, "{} should not be greater than or equal to {}", n.value, n.value + 1);
        }
    }
}
