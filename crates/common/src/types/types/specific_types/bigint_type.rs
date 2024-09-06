use std::any::Any;
use std::cmp::Ordering;
use std::fmt::{Debug, Display, Formatter};
use std::ops::{Add, Deref, Div, Mul, Rem, Sub};
use anyhow::anyhow;
use crate::types::{DBTypeId, DBTypeIdImpl, TypeIdTrait, Value, BUSTUB_I64_MAX, BUSTUB_I64_MIN, BUSTUB_I64_NULL, BUSTUB_VALUE_NULL};

#[derive(Copy, Debug)]
pub struct BigIntType {
    value: i64,
    len: u32,
}

impl BigIntType {
    pub fn new(value: i64) -> Self {
        BigIntType {
            value,
            len: if value == BUSTUB_I64_NULL { BUSTUB_VALUE_NULL } else { 0 },
        }
    }
}

impl Deref for BigIntType {
    type Target = i64;

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

impl From<i64> for BigIntType {
    fn from(value: i64) -> Self {
        BigIntType::new(value)
    }
}

impl From<&[u8]> for BigIntType {
    fn from(value: &[u8]) -> Self {
        // TODO - should we have type that indicate whether it's big int or other type?
        BigIntType::deserialize_from(value)
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

impl PartialEq<i64> for BigIntType {
    fn eq(&self, other: &i64) -> bool {
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

impl PartialOrd<i64> for BigIntType {
    fn partial_cmp(&self, other: &i64) -> Option<Ordering> {
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

impl Into<DBTypeIdImpl> for BigIntType {
    fn into(self) -> DBTypeIdImpl {
        DBTypeIdImpl::BIGINT(self)
    }
}

impl TypeIdTrait for BigIntType {
    const SIZE: u64 = size_of::<i64>() as u64;
    const NAME: &'static str = "BIGINT";

    fn get_type_id() -> DBTypeId {
        DBTypeId::BIGINT
    }


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

    fn is_zero(&self) -> bool {
        self.value == 0
    }

    fn is_inlined(&self) -> bool {
        true
    }

    fn get_data(&self) -> &[u8] {
        unimplemented!()
    }

    fn get_length(&self) -> u32 {
        unimplemented!()
    }

    fn get_data_from_slice(storage: &[u8]) -> &[u8] {
        unimplemented!()
    }

    fn to_string(&self) -> String {
        // TODO - what about null
        self.value.to_string()
    }

    fn serialize_to(&self, storage: &mut [u8]) {
        storage[0..Self::SIZE as usize].copy_from_slice(self.value.to_ne_bytes().as_slice())
    }

    fn deserialize_from(storage: &[u8]) -> Self {
        BigIntType::new(i64::from_ne_bytes(storage[..Self::SIZE as usize].try_into().unwrap()))
    }

    fn get_min_value() -> Self {
        Self::new(BUSTUB_I64_MIN)
    }

    fn get_max_value() -> Self {
        Self::new(BUSTUB_I64_MAX)
    }
}

#[cfg(test)]
mod test {
    use crate::types::{BigIntType, TypeIdTrait};

    #[test]
    fn basic_arithmetics_for_zero() {
        let numbers_i64: [i64; 201] = std::array::from_fn(|i| -100 + i as i64);
        let numbers: [BigIntType; 201] = std::array::from_fn(|i| (-100 + i as i64).into());

        // Make sure we created correctly
        for i in 0..201 {
            assert_eq!(numbers[i].value, numbers_i64[i]);
        }


        let zero = BigIntType::new(0);


        for number in numbers {
            let value = number.value;

            // 0 + i;
            assert_eq!(zero + number, number);
            assert_eq!(zero + number, value);

            // i + 0
            assert_eq!(number + zero, number);
            assert_eq!(number + zero, value);

            // 0 * i
            assert_eq!(zero * number, zero);
            assert_eq!(zero * number, 0);

            // i * 0
            assert_eq!(number * zero, zero);
            assert_eq!(number * zero, 0);
        }
    }

    #[test]
    fn basic_arithmetics() {
        let numbers_1_to_100: [BigIntType; 100] = std::array::from_fn(|i| (i as i64 + 1).into());

        // Validate all the numbers are correct
        for i in 0..100i64 {
            assert_eq!(numbers_1_to_100[i as usize], i + 1);
        }

        for a_index in 0..numbers_1_to_100.len() {
            let a = numbers_1_to_100[a_index];
            let a_value = (a_index as i64) + 1;

            for b_index in 0..numbers_1_to_100.len() {
                let b = numbers_1_to_100[b_index];
                let b_value = b_index as i64 + 1;

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
    fn basic_arithmetics_negative() {
        let numbers_minus100_to_1: [BigIntType; 100] = std::array::from_fn(|i| (-100 + i as i64).into());

        // Validate all the numbers are correct
        for i in 0..100i64 {
            assert_eq!(numbers_minus100_to_1[i as usize], -100 + i);
        }

        for a_index in 0..numbers_minus100_to_1.len() {
            let a = numbers_minus100_to_1[a_index];
            let a_value = -100 + (a_index as i64);

            for b_index in 0..numbers_minus100_to_1.len() {
                let b = numbers_minus100_to_1[b_index];
                let b_value = -100 + b_index as i64;

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
        let numbers_i64: [i64; 201] = std::array::from_fn(|i| -100 + i as i64);
        let numbers: [BigIntType; 201] = std::array::from_fn(|i| (-100 + i as i64).into());

        // Make sure we created correctly
        for i in 0..201 {
            assert_eq!(numbers[i].value, numbers_i64[i]);
        }

        for i in 0..201 {
            // =
            assert_eq!(numbers[i], BigIntType::new(numbers_i64[i]));
        }

        for n in numbers {
            // !=
            assert_ne!(n, BigIntType::new(200));
        }

        for n in numbers {
            // <
            assert!(n < BigIntType::new(n.value + 1), "{} < {}", n.value, n.value + 1);

            assert_eq!(n < BigIntType::new(n.value), false, "{} should not be less than {}", n.value, n.value);
            assert_eq!(n < BigIntType::new(n.value - 1), false, "{} should not be less than {}", n.value, n.value - 1);
        }

        for n in numbers {
            // <=
            assert!(n <= BigIntType::new(n.value), "{} <= {}", n.value, n.value);
            assert!(n <= BigIntType::new(n.value + 1), "{} <= {}", n.value, n.value + 1);

            assert_eq!(n <= BigIntType::new(n.value - 1), false, "{} should not be less than or equal to {}", n.value, n.value - 1);
        }

        for n in numbers {
            // >
            assert!(n > BigIntType::new(n.value - 1), "{} > {}", n.value, n.value - 1);

            assert_eq!(n > BigIntType::new(n.value), false, "{} should not be greater than {}", n.value, n.value);
            assert_eq!(n > BigIntType::new(n.value + 1), false, "{} should not be greater than {}", n.value, n.value + 1);
        }

        for n in numbers {
            // >=
            assert!(n >= BigIntType::new(n.value), "{} >= {}", n.value, n.value);

            assert!(n >= BigIntType::new(n.value - 1), "{} >= {}", n.value, n.value - 1);

            assert_eq!(n >= BigIntType::new(n.value + 1), false, "{} should not be greater than or equal to {}", n.value, n.value + 1);
        }
    }

    #[test]
    fn basic_serialize_deserialize() {
        let numbers_i64: [i64; 201] = std::array::from_fn(|i| -100 + i as i64);
        let numbers: [BigIntType; 201] = std::array::from_fn(|i| (-100 + i as i64).into());

        // Make sure we created correctly
        for i in 0..201 {
            assert_eq!(numbers[i].value, numbers_i64[i]);
        }

        for i in 0..numbers.len() {
            let number = numbers[i];
            let number_i64 = numbers_i64[i];

            let mut actual = [0u8; size_of::<i64>()];
            let expected = number_i64.to_ne_bytes();

            {
                numbers[i].serialize_to(&mut actual);
                assert_eq!(actual, expected, "serialize(BigIntType::new({})) == serialize({})", number_i64, number_i64);
            }

            {
                let deserialized = BigIntType::from(actual.as_slice());

                assert_eq!(deserialized, number, "BigIntType::from(serialize(BigIntType::new({})) == BigIntType::new({})", number_i64, number_i64);
                assert_eq!(deserialized, number_i64);
            }

            {
                let mut actual = [0u8; size_of::<i64>() * 2];
                actual[..size_of::<i64>()].copy_from_slice(expected.as_slice());
                let deserialized_from_larger = BigIntType::from(actual.as_slice());

                assert_eq!(deserialized_from_larger, number);
                assert_eq!(deserialized_from_larger, number_i64);
            }
        }
    }
}
