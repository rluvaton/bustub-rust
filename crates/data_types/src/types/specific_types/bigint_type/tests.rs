
#[cfg(test)]
mod tests {
    use super::super::BigIntUnderlyingType;
    use crate::types::{BigIntType, ConversionDBTypeTrait};

    #[test]
    fn basic_arithmetics_for_zero() {
        let numbers_i64: [BigIntUnderlyingType; 201] = std::array::from_fn(|i| -100 + i as BigIntUnderlyingType);
        let numbers: [BigIntType; 201] = std::array::from_fn(|i| (-100 + i as BigIntUnderlyingType).into());

        // Make sure we created correctly
        for i in 0..201 {
            assert_eq!(numbers[i].0, numbers_i64[i]);
        }


        let zero = BigIntType::new(0);


        for number in numbers {
            let value = number.0;

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
        let numbers_1_to_100: [BigIntType; 100] = std::array::from_fn(|i| (i as BigIntUnderlyingType + 1).into());

        // Validate all the numbers are correct
        for i in 0..100i64 {
            assert_eq!(numbers_1_to_100[i as usize], i + 1);
        }

        for a_index in 0..numbers_1_to_100.len() {
            let a = numbers_1_to_100[a_index];
            let a_value = (a_index as BigIntUnderlyingType) + 1;

            for b_index in 0..numbers_1_to_100.len() {
                let b = numbers_1_to_100[b_index];
                let b_value = b_index as BigIntUnderlyingType + 1;

                // a + b;
                assert_eq!((a + b).0, a_value + b_value);
                assert_eq!(a + b, BigIntType::new(a_value + b_value));

                // a * b;
                assert_eq!((a * b).0, a_value * b_value);
                assert_eq!(a * b, BigIntType::new(a_value * b_value));

                // a / b;
                assert_eq!((a / b).0, a_value / b_value);
                assert_eq!(a / b, BigIntType::new(a_value / b_value));

                // a % b
                assert_eq!((a % b).0, a_value % b_value);
                assert_eq!(a % b, BigIntType::new(a_value % b_value));
            }
        }
    }

    #[test]
    fn basic_arithmetics_negative() {
        let numbers_minus100_to_1: [BigIntType; 100] = std::array::from_fn(|i| (-100 + i as BigIntUnderlyingType).into());

        // Validate all the numbers are correct
        for i in 0..100i64 {
            assert_eq!(numbers_minus100_to_1[i as usize], -100 + i);
        }

        for a_index in 0..numbers_minus100_to_1.len() {
            let a = numbers_minus100_to_1[a_index];
            let a_value = -100 + (a_index as BigIntUnderlyingType);

            for b_index in 0..numbers_minus100_to_1.len() {
                let b = numbers_minus100_to_1[b_index];
                let b_value = -100 + b_index as BigIntUnderlyingType;

                // a + b;
                assert_eq!((a + b).0, a_value + b_value);
                assert_eq!(a + b, BigIntType::new(a_value + b_value));

                // a * b;
                assert_eq!((a * b).0, a_value * b_value);
                assert_eq!(a * b, BigIntType::new(a_value * b_value));

                // a / b;
                assert_eq!((a / b).0, a_value / b_value);
                assert_eq!(a / b, BigIntType::new(a_value / b_value));

                // a % b
                assert_eq!((a % b).0, a_value % b_value);
                assert_eq!(a % b, BigIntType::new(a_value % b_value));
            }
        }
    }

    #[test]
    fn basic_cmp() {
        let numbers_i64: [BigIntUnderlyingType; 201] = std::array::from_fn(|i| -100 + i as BigIntUnderlyingType);
        let numbers: [BigIntType; 201] = std::array::from_fn(|i| (-100 + i as BigIntUnderlyingType).into());

        // Make sure we created correctly
        for i in 0..201 {
            assert_eq!(numbers[i].0, numbers_i64[i]);
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
            assert!(n < BigIntType::new(n.0 + 1), "{} < {}", n.0, n.0 + 1);

            assert_eq!(n < BigIntType::new(n.0), false, "{} should not be less than {}", n.0, n.0);
            assert_eq!(n < BigIntType::new(n.0 - 1), false, "{} should not be less than {}", n.0, n.0 - 1);
        }

        for n in numbers {
            // <=
            assert!(n <= BigIntType::new(n.0), "{} <= {}", n.0, n.0);
            assert!(n <= BigIntType::new(n.0 + 1), "{} <= {}", n.0, n.0 + 1);

            assert_eq!(n <= BigIntType::new(n.0 - 1), false, "{} should not be less than or equal to {}", n.0, n.0 - 1);
        }

        for n in numbers {
            // >
            assert!(n > BigIntType::new(n.0 - 1), "{} > {}", n.0, n.0 - 1);

            assert_eq!(n > BigIntType::new(n.0), false, "{} should not be greater than {}", n.0, n.0);
            assert_eq!(n > BigIntType::new(n.0 + 1), false, "{} should not be greater than {}", n.0, n.0 + 1);
        }

        for n in numbers {
            // >=
            assert!(n >= BigIntType::new(n.0), "{} >= {}", n.0, n.0);

            assert!(n >= BigIntType::new(n.0 - 1), "{} >= {}", n.0, n.0 - 1);

            assert_eq!(n >= BigIntType::new(n.0 + 1), false, "{} should not be greater than or equal to {}", n.0, n.0 + 1);
        }
    }

    #[test]
    fn basic_ordering_operations() {
        let numbers_i64: [BigIntUnderlyingType; 201] = std::array::from_fn(|i| -100 + i as BigIntUnderlyingType);
        let numbers: [BigIntType; 201] = std::array::from_fn(|i| (-100 + i as BigIntUnderlyingType).into());

        // Make sure we created correctly
        for i in 0..201 {
            assert_eq!(numbers[i].0, numbers_i64[i]);
        }

        {
            let mut numbers_i64_sorted = numbers_i64.clone();
            numbers_i64_sorted.sort();

            let mut numbers_sorted = numbers.clone();
            numbers_sorted.sort();

            let numbers_i64_parsed: [BigIntType; 201] = numbers_i64_sorted.map(|item| item.into());
            assert_eq!(numbers_i64_parsed, numbers_sorted);
        }

        {
            let max_number: BigIntType = numbers_i64.iter().max().expect("Must have max item").into();

            let max_db_type = numbers.iter().max().expect("Must have max item");

            assert_eq!(max_db_type, &max_number);
        }

        {
            let min_number: BigIntType = numbers_i64.iter().min().expect("Must have min item").into();

            let min_db_type = numbers.iter().min().expect("Must have min item");

            assert_eq!(min_db_type, &min_number);
        }
    }

    #[test]
    fn basic_serialize_deserialize() {
        let numbers_i64: [BigIntUnderlyingType; 201] = std::array::from_fn(|i| -100 + i as BigIntUnderlyingType);
        let numbers: [BigIntType; 201] = std::array::from_fn(|i| (-100 + i as BigIntUnderlyingType).into());

        // Make sure we created correctly
        for i in 0..201 {
            assert_eq!(numbers[i].0, numbers_i64[i]);
        }

        for i in 0..numbers.len() {
            let number = numbers[i];
            let number_i64 = numbers_i64[i];

            let mut actual = [0u8; size_of::<BigIntUnderlyingType>()];
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
                let mut actual = [0u8; size_of::<BigIntUnderlyingType>() * 2];
                actual[..size_of::<BigIntUnderlyingType>()].copy_from_slice(expected.as_slice());
                let deserialized_from_larger = BigIntType::from(actual.as_slice());

                assert_eq!(deserialized_from_larger, number);
                assert_eq!(deserialized_from_larger, number_i64);
            }
        }
    }
}
