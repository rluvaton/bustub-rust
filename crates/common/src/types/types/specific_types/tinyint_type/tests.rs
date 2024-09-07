
#[cfg(test)]
mod tests {
    use crate::types::{TinyIntType, ConversionDBTypeTrait};
    use super::super::TinyIntUnderlyingType;

    #[test]
    fn basic_arithmetics_for_zero() {
        let numbers_i16: [TinyIntUnderlyingType; 201] = std::array::from_fn(|i| -100 + i as TinyIntUnderlyingType);
        let numbers: [TinyIntType; 201] = std::array::from_fn(|i| (-100 + i as TinyIntUnderlyingType).into());

        // Make sure we created correctly
        for i in 0..201 {
            assert_eq!(numbers[i].value, numbers_i16[i]);
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
        let numbers_1_to_100: [TinyIntType; 100] = std::array::from_fn(|i| (i as TinyIntUnderlyingType + 1).into());

        // Validate all the numbers are correct
        for i in 0..100i8 {
            assert_eq!(numbers_1_to_100[i as usize], i + 1);
        }

        for a_index in 0..numbers_1_to_100.len() {
            let a = numbers_1_to_100[a_index];
            let a_value = (a_index as TinyIntUnderlyingType) + 1;

            for b_index in 0..numbers_1_to_100.len() {
                let b = numbers_1_to_100[b_index];
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
        let numbers_minus100_to_1: [TinyIntType; 100] = std::array::from_fn(|i| (-100 + i as TinyIntUnderlyingType).into());

        // Validate all the numbers are correct
        for i in 0..100i8 {
            assert_eq!(numbers_minus100_to_1[i as usize], -100 + i);
        }

        for a_index in 0..numbers_minus100_to_1.len() {
            let a = numbers_minus100_to_1[a_index];
            let a_value = -100 + (a_index as TinyIntUnderlyingType);

            for b_index in 0..numbers_minus100_to_1.len() {
                let b = numbers_minus100_to_1[b_index];
                let b_value = -100 + b_index as TinyIntUnderlyingType;

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
        let numbers_i16: [TinyIntUnderlyingType; 201] = std::array::from_fn(|i| -100 + i as TinyIntUnderlyingType);
        let numbers: [TinyIntType; 201] = std::array::from_fn(|i| (-100 + i as TinyIntUnderlyingType).into());

        // Make sure we created correctly
        for i in 0..201 {
            assert_eq!(numbers[i].value, numbers_i16[i]);
        }

        for i in 0..201 {
            // =
            assert_eq!(numbers[i], TinyIntType::new(numbers_i16[i]));
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
    fn basic_serialize_deserialize() {
        let numbers_i16: [TinyIntUnderlyingType; 201] = std::array::from_fn(|i| -100 + i as TinyIntUnderlyingType);
        let numbers: [TinyIntType; 201] = std::array::from_fn(|i| (-100 + i as TinyIntUnderlyingType).into());

        // Make sure we created correctly
        for i in 0..201 {
            assert_eq!(numbers[i].value, numbers_i16[i]);
        }

        for i in 0..numbers.len() {
            let number = numbers[i];
            let number_i16 = numbers_i16[i];

            let mut actual = [0u8; size_of::<TinyIntUnderlyingType>()];
            let expected = number_i16.to_ne_bytes();

            {
                numbers[i].serialize_to(&mut actual);
                assert_eq!(actual, expected, "serialize(TinyIntType::new({})) == serialize({})", number_i16, number_i16);
            }

            {
                let deserialized = TinyIntType::from(actual.as_slice());

                assert_eq!(deserialized, number, "TinyIntType::from(serialize(TinyIntType::new({})) == TinyIntType::new({})", number_i16, number_i16);
                assert_eq!(deserialized, number_i16);
            }

            {
                let mut actual = [0u8; size_of::<TinyIntUnderlyingType>() * 2];
                actual[..size_of::<TinyIntUnderlyingType>()].copy_from_slice(expected.as_slice());
                let deserialized_from_larger = TinyIntType::from(actual.as_slice());

                assert_eq!(deserialized_from_larger, number);
                assert_eq!(deserialized_from_larger, number_i16);
            }
        }
    }
}
