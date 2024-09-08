
#[cfg(test)]
mod tests {
    use crate::types::{SmallIntType, ConversionDBTypeTrait, DBTypeIdTrait, IntUnderlyingType, IntType};
    use super::super::SmallIntUnderlyingType;

    #[test]
    fn basic_arithmetics_for_zero() {
        let numbers_i16: [SmallIntUnderlyingType; 201] = std::array::from_fn(|i| -100 + i as SmallIntUnderlyingType);
        let numbers: [SmallIntType; 201] = std::array::from_fn(|i| (-100 + i as SmallIntUnderlyingType).into());

        // Make sure we created correctly
        for i in 0..201 {
            assert_eq!(numbers[i].value, numbers_i16[i]);
        }


        let zero = SmallIntType::new(0);


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
        let numbers_1_to_100: [SmallIntType; 100] = std::array::from_fn(|i| (i as SmallIntUnderlyingType + 1).into());

        // Validate all the numbers are correct
        for i in 0..100i16 {
            assert_eq!(numbers_1_to_100[i as usize], i + 1);
        }

        for a_index in 0..numbers_1_to_100.len() {
            let a = numbers_1_to_100[a_index];
            let a_value = (a_index as SmallIntUnderlyingType) + 1;

            for b_index in 0..numbers_1_to_100.len() {
                let b = numbers_1_to_100[b_index];
                let b_value = b_index as SmallIntUnderlyingType + 1;

                // a + b;
                assert_eq!((a + b).value, a_value + b_value);
                assert_eq!(a + b, SmallIntType::new(a_value + b_value));

                // a * b;
                assert_eq!((a * b).value, a_value * b_value);
                assert_eq!(a * b, SmallIntType::new(a_value * b_value));

                // a / b;
                assert_eq!((a / b).value, a_value / b_value);
                assert_eq!(a / b, SmallIntType::new(a_value / b_value));

                // a % b
                assert_eq!((a % b).value, a_value % b_value);
                assert_eq!(a % b, SmallIntType::new(a_value % b_value));
            }
        }
    }

    #[test]
    fn basic_arithmetics_negative() {
        let numbers_minus100_to_1: [SmallIntType; 100] = std::array::from_fn(|i| (-100 + i as SmallIntUnderlyingType).into());

        // Validate all the numbers are correct
        for i in 0..100i16 {
            assert_eq!(numbers_minus100_to_1[i as usize], -100 + i);
        }

        for a_index in 0..numbers_minus100_to_1.len() {
            let a = numbers_minus100_to_1[a_index];
            let a_value = -100 + (a_index as SmallIntUnderlyingType);

            for b_index in 0..numbers_minus100_to_1.len() {
                let b = numbers_minus100_to_1[b_index];
                let b_value = -100 + b_index as SmallIntUnderlyingType;

                // a + b;
                assert_eq!((a + b).value, a_value + b_value);
                assert_eq!(a + b, SmallIntType::new(a_value + b_value));

                // a * b;
                assert_eq!((a * b).value, a_value * b_value);
                assert_eq!(a * b, SmallIntType::new(a_value * b_value));

                // a / b;
                assert_eq!((a / b).value, a_value / b_value);
                assert_eq!(a / b, SmallIntType::new(a_value / b_value));

                // a % b
                assert_eq!((a % b).value, a_value % b_value);
                assert_eq!(a % b, SmallIntType::new(a_value % b_value));
            }
        }
    }

    #[test]
    fn basic_cmp() {
        let numbers_i16: [SmallIntUnderlyingType; 201] = std::array::from_fn(|i| -100 + i as SmallIntUnderlyingType);
        let numbers: [SmallIntType; 201] = std::array::from_fn(|i| (-100 + i as SmallIntUnderlyingType).into());

        // Make sure we created correctly
        for i in 0..201 {
            assert_eq!(numbers[i].value, numbers_i16[i]);
        }

        for i in 0..201 {
            // =
            assert_eq!(numbers[i], SmallIntType::new(numbers_i16[i]));
        }

        for n in numbers {
            // !=
            assert_ne!(n, SmallIntType::new(200));
        }

        for n in numbers {
            // <
            assert!(n < SmallIntType::new(n.value + 1), "{} < {}", n.value, n.value + 1);

            assert_eq!(n < SmallIntType::new(n.value), false, "{} should not be less than {}", n.value, n.value);
            assert_eq!(n < SmallIntType::new(n.value - 1), false, "{} should not be less than {}", n.value, n.value - 1);
        }

        for n in numbers {
            // <=
            assert!(n <= SmallIntType::new(n.value), "{} <= {}", n.value, n.value);
            assert!(n <= SmallIntType::new(n.value + 1), "{} <= {}", n.value, n.value + 1);

            assert_eq!(n <= SmallIntType::new(n.value - 1), false, "{} should not be less than or equal to {}", n.value, n.value - 1);
        }

        for n in numbers {
            // >
            assert!(n > SmallIntType::new(n.value - 1), "{} > {}", n.value, n.value - 1);

            assert_eq!(n > SmallIntType::new(n.value), false, "{} should not be greater than {}", n.value, n.value);
            assert_eq!(n > SmallIntType::new(n.value + 1), false, "{} should not be greater than {}", n.value, n.value + 1);
        }

        for n in numbers {
            // >=
            assert!(n >= SmallIntType::new(n.value), "{} >= {}", n.value, n.value);

            assert!(n >= SmallIntType::new(n.value - 1), "{} >= {}", n.value, n.value - 1);

            assert_eq!(n >= SmallIntType::new(n.value + 1), false, "{} should not be greater than or equal to {}", n.value, n.value + 1);
        }
    }

    #[test]
    fn basic_ordering_operations() {
        let numbers_i16: [SmallIntUnderlyingType; 201] = std::array::from_fn(|i| -100 + i as SmallIntUnderlyingType);
        let numbers: [SmallIntType; 201] = std::array::from_fn(|i| (-100 + i as SmallIntUnderlyingType).into());

        // Make sure we created correctly
        for i in 0..201 {
            assert_eq!(numbers[i].value, numbers_i16[i]);
        }

        {
            let mut numbers_i16_sorted = numbers_i16.clone();
            numbers_i16_sorted.sort();

            let mut numbers_sorted = numbers.clone();
            numbers_sorted.sort();

            let numbers_i16_parsed: [SmallIntType; 201] = numbers_i16_sorted.map(|item| item.into());
            assert_eq!(numbers_i16_parsed, numbers_sorted);
        }

        {
            let max_number: SmallIntType = numbers_i16.iter().max().expect("Must have max item").into();

            let max_db_type = numbers.iter().max().expect("Must have max item");

            assert_eq!(max_db_type, &max_number);
        }

        {
            let min_number: SmallIntType = numbers_i16.iter().min().expect("Must have min item").into();

            let min_db_type = numbers.iter().min().expect("Must have min item");

            assert_eq!(min_db_type, &min_number);
        }
    }

    #[test]
    fn basic_serialize_deserialize() {
        let numbers_i16: [SmallIntUnderlyingType; 201] = std::array::from_fn(|i| -100 + i as SmallIntUnderlyingType);
        let numbers: [SmallIntType; 201] = std::array::from_fn(|i| (-100 + i as SmallIntUnderlyingType).into());

        // Make sure we created correctly
        for i in 0..201 {
            assert_eq!(numbers[i].value, numbers_i16[i]);
        }

        for i in 0..numbers.len() {
            let number = numbers[i];
            let number_i16 = numbers_i16[i];

            let mut actual = [0u8; size_of::<SmallIntUnderlyingType>()];
            let expected = number_i16.to_ne_bytes();

            {
                numbers[i].serialize_to(&mut actual);
                assert_eq!(actual, expected, "serialize(SmallIntType::new({})) == serialize({})", number_i16, number_i16);
            }

            {
                let deserialized = SmallIntType::from(actual.as_slice());

                assert_eq!(deserialized, number, "SmallIntType::from(serialize(SmallIntType::new({})) == SmallIntType::new({})", number_i16, number_i16);
                assert_eq!(deserialized, number_i16);
            }

            {
                let mut actual = [0u8; size_of::<SmallIntUnderlyingType>() * 2];
                actual[..size_of::<SmallIntUnderlyingType>()].copy_from_slice(expected.as_slice());
                let deserialized_from_larger = SmallIntType::from(actual.as_slice());

                assert_eq!(deserialized_from_larger, number);
                assert_eq!(deserialized_from_larger, number_i16);
            }
        }
    }
}
