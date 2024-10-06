#[cfg(test)]
mod tests {
    use crate::types::{ConversionDBTypeTrait, DecimalType, DecimalUnderlyingType};

    #[test]
    fn basic_arithmetics_for_zero() {
        // Divided in the tests by 17 as it's prime and it would create floating number
        let numbers_f64: [DecimalUnderlyingType; 201] = std::array::from_fn(|i| (-100.0 + i as DecimalUnderlyingType) / 17.0);
        let numbers: [DecimalType; 201] = std::array::from_fn(|i| ((-100.0 + i as DecimalUnderlyingType) / 17.0).into());

        // Make sure we created correctly
        for i in 0..201 {
            assert_eq!(numbers[i].value, numbers_f64[i]);
        }


        let zero = DecimalType::new(0.0);

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
            assert_eq!(zero * number, 0.0);

            // i * 0
            assert_eq!(number * zero, zero);
            assert_eq!(number * zero, 0.0);
        }
    }

    #[test]
    fn basic_arithmetics() {
        // Divided in the tests by 17 as it's prime and it would create floating number
        let numbers_1_to_100: [DecimalType; 100] = std::array::from_fn(|i| ((i as DecimalUnderlyingType + 1.0) / 17.0).into());

        // Validate all the numbers are correct
        for i in 0..100 {
            assert_eq!(numbers_1_to_100[i as usize], (i as f64 + 1.0) / 17.0);
        }

        for a_index in 0..numbers_1_to_100.len() {
            let a = numbers_1_to_100[a_index];
            let a_value = ((a_index as DecimalUnderlyingType) + 1.0) / 17.0;

            for b_index in 0..numbers_1_to_100.len() {
                let b = numbers_1_to_100[b_index];
                let b_value = (b_index as DecimalUnderlyingType + 1.0) / 17.0;

                // a + b;
                assert_eq!((a + b).value, a_value + b_value);
                assert_eq!(a + b, DecimalType::new(a_value + b_value));

                // a * b;
                assert_eq!((a * b).value, a_value * b_value);
                assert_eq!(a * b, DecimalType::new(a_value * b_value));

                // a / b;
                assert_eq!((a / b).value, a_value / b_value);
                assert_eq!(a / b, DecimalType::new(a_value / b_value));

                // a % b
                assert_eq!((a % b).value, a_value % b_value);
                assert_eq!(a % b, DecimalType::new(a_value % b_value));
            }
        }
    }

    #[test]
    fn basic_arithmetics_negative() {
        // Divided in the tests by 17 as it's prime and it would create floating number
        let numbers_minus100_to_1: [DecimalType; 100] = std::array::from_fn(|i| ((-100.0 + i as DecimalUnderlyingType) / 17.0).into());

        // Validate all the numbers are correct
        for i in 0..100 {
            assert_eq!(numbers_minus100_to_1[i as usize], (-100.0 + i as f64) / 17.0);
        }

        for a_index in 0..numbers_minus100_to_1.len() {
            let a = numbers_minus100_to_1[a_index];
            let a_value = (-100.0 + (a_index as DecimalUnderlyingType)) / 17.0;

            for b_index in 0..numbers_minus100_to_1.len() {
                let b = numbers_minus100_to_1[b_index];
                let b_value = (-100.0 + b_index as DecimalUnderlyingType) / 17.0;

                // a + b;
                assert_eq!((a + b).value, a_value + b_value);
                assert_eq!(a + b, DecimalType::new(a_value + b_value));

                // a * b;
                assert_eq!((a * b).value, a_value * b_value);
                assert_eq!(a * b, DecimalType::new(a_value * b_value));

                // a / b;
                assert_eq!((a / b).value, a_value / b_value);
                assert_eq!(a / b, DecimalType::new(a_value / b_value));

                // a % b
                assert_eq!((a % b).value, a_value % b_value);
                assert_eq!(a % b, DecimalType::new(a_value % b_value));
            }
        }
    }

    #[test]
    fn basic_cmp() {
        // Divided in the tests by 17 as it's prime and it would create floating number
        let numbers_f64: [DecimalUnderlyingType; 201] = std::array::from_fn(|i| (-100.0 + i as DecimalUnderlyingType) / 17.0);
        let numbers: [DecimalType; 201] = std::array::from_fn(|i| ((-100.0 + i as DecimalUnderlyingType) / 17.0).into());

        // Make sure we created correctly
        for i in 0..201 {
            assert_eq!(numbers[i].value, numbers_f64[i]);
        }

        for i in 0..201 {
            // =
            assert_eq!(numbers[i], DecimalType::new(numbers_f64[i]));
        }

        for n in numbers {
            // !=
            assert_ne!(n, DecimalType::new(200.0));
        }

        for n in numbers {
            // <
            assert!(n < DecimalType::new(n.value + 1.0), "{} < {}", n.value, n.value + 1.0);

            assert_eq!(n < DecimalType::new(n.value), false, "{} should not be less than {}", n.value, n.value);
            assert_eq!(n < DecimalType::new(n.value - 1.0), false, "{} should not be less than {}", n.value, n.value - 1.0);
        }

        for n in numbers {
            // <=
            assert!(n <= DecimalType::new(n.value), "{} <= {}", n.value, n.value);
            assert!(n <= DecimalType::new(n.value + 1.0), "{} <= {}", n.value, n.value + 1.0);

            assert_eq!(n <= DecimalType::new(n.value - 1.0), false, "{} should not be less than or equal to {}", n.value, n.value - 1.0);
        }

        for n in numbers {
            // >
            assert!(n > DecimalType::new(n.value - 1.0), "{} > {}", n.value, n.value - 1.0);

            assert_eq!(n > DecimalType::new(n.value), false, "{} should not be greater than {}", n.value, n.value);
            assert_eq!(n > DecimalType::new(n.value + 1.0), false, "{} should not be greater than {}", n.value, n.value + 1.0);
        }

        for n in numbers {
            // >=
            assert!(n >= DecimalType::new(n.value), "{} >= {}", n.value, n.value);

            assert!(n >= DecimalType::new(n.value - 1.0), "{} >= {}", n.value, n.value - 1.0);

            assert_eq!(n >= DecimalType::new(n.value + 1.0), false, "{} should not be greater than or equal to {}", n.value, n.value + 1.0);
        }
    }

    #[test]
    fn basic_ordering_operations() {
        // Divided in the tests by 17 as it's prime and it would create floating number
        let numbers_f64: [DecimalUnderlyingType; 201] = std::array::from_fn(|i| (-100.0 + i as DecimalUnderlyingType) / 17.0);
        let numbers: [DecimalType; 201] = std::array::from_fn(|i| ((-100.0 + i as DecimalUnderlyingType) / 17.0).into());

        // Make sure we created correctly
        for i in 0..201 {
            assert_eq!(numbers[i].value, numbers_f64[i]);
        }

        {
            let mut numbers_f64_sorted = numbers_f64.clone();
            numbers_f64_sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());

            let mut numbers_sorted = numbers.clone();
            numbers_sorted.sort();

            let numbers_f64_parsed: [DecimalType; 201] = numbers_f64_sorted.map(|item| item.into());
            assert_eq!(numbers_f64_parsed, numbers_sorted);
        }

        {
            let max_number: DecimalType = numbers_f64.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).expect("Must have max item").into();

            let max_db_type = numbers.iter().max().expect("Must have max item");

            assert_eq!(max_db_type, &max_number);
        }

        {
            let min_number: DecimalType = numbers_f64.iter().min_by(|a, b| a.partial_cmp(b).unwrap()).expect("Must have min item").into();

            let min_db_type = numbers.iter().min().expect("Must have min item");

            assert_eq!(min_db_type, &min_number);
        }
    }

    #[test]
    fn basic_serialize_deserialize() {
        // Divided in the tests by 17 as it's prime and it would create floating number
        let numbers_f64: [DecimalUnderlyingType; 201] = std::array::from_fn(|i| (-100.0 + i as DecimalUnderlyingType) / 17.0);
        let numbers: [DecimalType; 201] = std::array::from_fn(|i| ((-100.0 + i as DecimalUnderlyingType) / 17.0).into());

        // Make sure we created correctly
        for i in 0..201 {
            assert_eq!(numbers[i].value, numbers_f64[i]);
        }

        for i in 0..numbers.len() {
            let number = numbers[i];
            let number_f64 = numbers_f64[i];

            let mut actual = [0u8; size_of::<DecimalUnderlyingType>()];
            let expected = number_f64.to_ne_bytes();

            {
                numbers[i].serialize_to(&mut actual);
                assert_eq!(actual, expected, "serialize(DecimalType::new({})) == serialize({})", number_f64, number_f64);
            }

            {
                let deserialized = DecimalType::from(actual.as_slice());

                assert_eq!(deserialized, number, "DecimalType::from(serialize(DecimalType::new({})) == DecimalType::new({})", number_f64, number_f64);
                assert_eq!(deserialized, number_f64);
            }

            {
                let mut actual = [0u8; size_of::<DecimalUnderlyingType>() * 2];
                actual[..size_of::<DecimalUnderlyingType>()].copy_from_slice(expected.as_slice());
                let deserialized_from_larger = DecimalType::from(actual.as_slice());

                assert_eq!(deserialized_from_larger, number);
                assert_eq!(deserialized_from_larger, number_f64);
            }
        }
    }
}
