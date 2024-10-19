
#[cfg(test)]
mod tests {
    use super::super::TinyIntUnderlyingType;
    use crate::types::{ConversionDBTypeTrait, TinyIntType};

    #[test]
    fn basic_arithmetics_for_zero() {
        let numbers_i8: [TinyIntUnderlyingType; 81] = std::array::from_fn(|i| -40 + i as TinyIntUnderlyingType);
        let numbers: [TinyIntType; 81] = std::array::from_fn(|i| (-40 + i as TinyIntUnderlyingType).into());

        // Make sure we created correctly
        for i in 0..81 {
            assert_eq!(numbers[i].value, numbers_i8[i]);
        }

        let zero = TinyIntType::new(0);

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
        let numbers_1_to_10: [TinyIntType; 10] = std::array::from_fn(|i| (i as TinyIntUnderlyingType + 1).into());

        // Validate all the numbers are correct
        for i in 0..10i8 {
            assert_eq!(numbers_1_to_10[i as usize], i + 1);
        }

        for a_index in 0..numbers_1_to_10.len() {
            let a = numbers_1_to_10[a_index];
            let a_value = (a_index as TinyIntUnderlyingType) + 1;

            for b_index in 0..numbers_1_to_10.len() {
                let b = numbers_1_to_10[b_index];
                let b_value = b_index as TinyIntUnderlyingType + 1;

                // a + b;
                assert_eq!((a + b).value, a_value + b_value);
                assert_eq!(a + b, TinyIntType::new(a_value + b_value));

                // a * b;
                assert_eq!((a * b).value, a_value * b_value);
                assert_eq!(a * b, TinyIntType::new(a_value * b_value));

                // a / b;
                assert_eq!((a / b).value, a_value / b_value);
                assert_eq!(a / b, TinyIntType::new(a_value / b_value));

                // a % b
                assert_eq!((a % b).value, a_value % b_value);
                assert_eq!(a % b, TinyIntType::new(a_value % b_value));
            }
        }
    }

    #[test]
    fn basic_arithmetics_negative() {
        let numbers_minus10_to_1: [TinyIntType; 10] = std::array::from_fn(|i| (-10 + i as TinyIntUnderlyingType).into());

        // Validate all the numbers are correct
        for i in 0..10i8 {
            assert_eq!(numbers_minus10_to_1[i as usize], -10 + i);
        }

        for a_index in 0..numbers_minus10_to_1.len() {
            let a = numbers_minus10_to_1[a_index];
            let a_value = -10 + (a_index as TinyIntUnderlyingType);

            for b_index in 0..numbers_minus10_to_1.len() {
                let b = numbers_minus10_to_1[b_index];
                let b_value = -10 + b_index as TinyIntUnderlyingType;

                // a + b;
                assert_eq!((a + b).value, a_value + b_value);
                assert_eq!(a + b, TinyIntType::new(a_value + b_value));

                // a * b;
                assert_eq!((a * b).value, a_value * b_value);
                assert_eq!(a * b, TinyIntType::new(a_value * b_value));

                // a / b;
                assert_eq!((a / b).value, a_value / b_value);
                assert_eq!(a / b, TinyIntType::new(a_value / b_value));

                // a % b
                assert_eq!((a % b).value, a_value % b_value);
                assert_eq!(a % b, TinyIntType::new(a_value % b_value));
            }
        }
    }

    #[test]
    fn basic_cmp() {
        let numbers_i8: [TinyIntUnderlyingType; 21] = std::array::from_fn(|i| -10 + i as TinyIntUnderlyingType);
        let numbers: [TinyIntType; 21] = std::array::from_fn(|i| (-10 + i as TinyIntUnderlyingType).into());

        // Make sure we created correctly
        for i in 0..21 {
            assert_eq!(numbers[i].value, numbers_i8[i]);
        }

        for i in 0..21 {
            // =
            assert_eq!(numbers[i], TinyIntType::new(numbers_i8[i]));
        }

        for n in numbers {
            // !=
            assert_ne!(n, TinyIntType::new(123));
        }

        for n in numbers {
            // <
            assert!(n < TinyIntType::new(n.value + 1), "{} < {}", n.value, n.value + 1);

            assert_eq!(n < TinyIntType::new(n.value), false, "{} should not be less than {}", n.value, n.value);
            assert_eq!(n < TinyIntType::new(n.value - 1), false, "{} should not be less than {}", n.value, n.value - 1);
        }

        for n in numbers {
            // <=
            assert!(n <= TinyIntType::new(n.value), "{} <= {}", n.value, n.value);
            assert!(n <= TinyIntType::new(n.value + 1), "{} <= {}", n.value, n.value + 1);

            assert_eq!(n <= TinyIntType::new(n.value - 1), false, "{} should not be less than or equal to {}", n.value, n.value - 1);
        }

        for n in numbers {
            // >
            assert!(n > TinyIntType::new(n.value - 1), "{} > {}", n.value, n.value - 1);

            assert_eq!(n > TinyIntType::new(n.value), false, "{} should not be greater than {}", n.value, n.value);
            assert_eq!(n > TinyIntType::new(n.value + 1), false, "{} should not be greater than {}", n.value, n.value + 1);
        }

        for n in numbers {
            // >=
            assert!(n >= TinyIntType::new(n.value), "{} >= {}", n.value, n.value);

            assert!(n >= TinyIntType::new(n.value - 1), "{} >= {}", n.value, n.value - 1);

            assert_eq!(n >= TinyIntType::new(n.value + 1), false, "{} should not be greater than or equal to {}", n.value, n.value + 1);
        }
    }

    #[test]
    fn basic_ordering_operations() {
        let numbers_i8: [TinyIntUnderlyingType; 21] = std::array::from_fn(|i| -10 + i as TinyIntUnderlyingType);
        let numbers: [TinyIntType; 21] = std::array::from_fn(|i| (-10 + i as TinyIntUnderlyingType).into());

        // Make sure we created correctly
        for i in 0..21 {
            assert_eq!(numbers[i].value, numbers_i8[i]);
        }

        {
            let mut numbers_i8_sorted = numbers_i8.clone();
            numbers_i8_sorted.sort();

            let mut numbers_sorted = numbers.clone();
            numbers_sorted.sort();

            let numbers_i8_parsed: [TinyIntType; 21] = numbers_i8_sorted.map(|item| item.into());
            assert_eq!(numbers_i8_parsed, numbers_sorted);
        }

        {
            let max_number: TinyIntType = numbers_i8.iter().max().expect("Must have max item").into();

            let max_db_type = numbers.iter().max().expect("Must have max item");

            assert_eq!(max_db_type, &max_number);
        }

        {
            let min_number: TinyIntType = numbers_i8.iter().min().expect("Must have min item").into();

            let min_db_type = numbers.iter().min().expect("Must have min item");

            assert_eq!(min_db_type, &min_number);
        }
    }

    #[test]
    fn basic_serialize_deserialize() {
        let numbers_i8: [TinyIntUnderlyingType; 21] = std::array::from_fn(|i| -10 + i as TinyIntUnderlyingType);
        let numbers: [TinyIntType; 21] = std::array::from_fn(|i| (-10 + i as TinyIntUnderlyingType).into());

        // Make sure we created correctly
        for i in 0..21 {
            assert_eq!(numbers[i].value, numbers_i8[i]);
        }

        for i in 0..numbers.len() {
            let number = numbers[i];
            let number_i8 = numbers_i8[i];

            let mut actual = [0u8; size_of::<TinyIntUnderlyingType>()];
            let expected = number_i8.to_ne_bytes();

            {
                numbers[i].serialize_to(&mut actual);
                assert_eq!(actual, expected, "serialize(TinyIntType::new({})) == serialize({})", number_i8, number_i8);
            }

            {
                let deserialized = TinyIntType::from(actual.as_slice());

                assert_eq!(deserialized, number, "TinyIntType::from(serialize(TinyIntType::new({})) == TinyIntType::new({})", number_i8, number_i8);
                assert_eq!(deserialized, number_i8);
            }

            {
                let mut actual = [0u8; size_of::<TinyIntUnderlyingType>() * 2];
                actual[..size_of::<TinyIntUnderlyingType>()].copy_from_slice(expected.as_slice());
                let deserialized_from_larger = TinyIntType::from(actual.as_slice());

                assert_eq!(deserialized_from_larger, number);
                assert_eq!(deserialized_from_larger, number_i8);
            }
        }
    }
}
