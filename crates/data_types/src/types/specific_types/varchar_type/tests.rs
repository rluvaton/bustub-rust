
#[cfg(test)]
mod tests {
    use crate::types::{BigIntType, ConversionDBTypeTrait, DBTypeIdTrait};
    use crate::{ComparisonDBTypeTrait, VarcharType};

    // #[test]
    // fn basic_cmp() {
    //     let numbers_i64: [VarcharType; 201] = std::array::from_fn(|i| -100 + i);
    //     let numbers: [BigIntType; 201] = std::array::from_fn(|i| (-100 + i as BigIntUnderlyingType).into());
    //
    //     // Make sure we created correctly
    //     for i in 0..201 {
    //         assert_eq!(numbers[i].value, numbers_i64[i]);
    //     }
    //
    //     for i in 0..201 {
    //         // =
    //         assert_eq!(numbers[i], BigIntType::new(numbers_i64[i]));
    //     }
    //
    //     for n in numbers {
    //         // !=
    //         assert_ne!(n, BigIntType::new(200));
    //     }
    //
    //     for n in numbers {
    //         // <
    //         assert!(n < BigIntType::new(n.value + 1), "{} < {}", n.value, n.value + 1);
    //
    //         assert_eq!(n < BigIntType::new(n.value), false, "{} should not be less than {}", n.value, n.value);
    //         assert_eq!(n < BigIntType::new(n.value - 1), false, "{} should not be less than {}", n.value, n.value - 1);
    //     }
    //
    //     for n in numbers {
    //         // <=
    //         assert!(n <= BigIntType::new(n.value), "{} <= {}", n.value, n.value);
    //         assert!(n <= BigIntType::new(n.value + 1), "{} <= {}", n.value, n.value + 1);
    //
    //         assert_eq!(n <= BigIntType::new(n.value - 1), false, "{} should not be less than or equal to {}", n.value, n.value - 1);
    //     }
    //
    //     for n in numbers {
    //         // >
    //         assert!(n > BigIntType::new(n.value - 1), "{} > {}", n.value, n.value - 1);
    //
    //         assert_eq!(n > BigIntType::new(n.value), false, "{} should not be greater than {}", n.value, n.value);
    //         assert_eq!(n > BigIntType::new(n.value + 1), false, "{} should not be greater than {}", n.value, n.value + 1);
    //     }
    //
    //     for n in numbers {
    //         // >=
    //         assert!(n >= BigIntType::new(n.value), "{} >= {}", n.value, n.value);
    //
    //         assert!(n >= BigIntType::new(n.value - 1), "{} >= {}", n.value, n.value - 1);
    //
    //         assert_eq!(n >= BigIntType::new(n.value + 1), false, "{} should not be greater than or equal to {}", n.value, n.value + 1);
    //     }
    // }

    // #[test]
    // fn basic_ordering_operations() {
        // let numbers_i64: [BigIntUnderlyingType; 201] = std::array::from_fn(|i| -100 + i as BigIntUnderlyingType);
        // let numbers: [BigIntType; 201] = std::array::from_fn(|i| (-100 + i as BigIntUnderlyingType).into());
        //
        // // Make sure we created correctly
        // for i in 0..201 {
        //     assert_eq!(numbers[i].value, numbers_i64[i]);
        // }
        //
        // {
        //     let mut numbers_i64_sorted = numbers_i64.clone();
        //     numbers_i64_sorted.sort();
        //
        //     let mut numbers_sorted = numbers.clone();
        //     numbers_sorted.sort();
        //
        //     let numbers_i64_parsed: [BigIntType; 201] = numbers_i64_sorted.map(|item| item.into());
        //     assert_eq!(numbers_i64_parsed, numbers_sorted);
        // }
        //
        // {
        //     let max_number: BigIntType = numbers_i64.iter().max().expect("Must have max item").into();
        //
        //     let max_db_type = numbers.iter().max().expect("Must have max item");
        //
        //     assert_eq!(max_db_type, &max_number);
        // }
        //
        // {
        //     let min_number: BigIntType = numbers_i64.iter().min().expect("Must have min item").into();
        //
        //     let min_db_type = numbers.iter().min().expect("Must have min item");
        //
        //     assert_eq!(min_db_type, &min_number);
        // }
    // }

    #[test]
    fn serialize_deserialize_null() {
        let mut storage = [0u8; 10000];
        let null = VarcharType::from(None);

        assert_eq!(null.is_null(), true);

        null.serialize_to(&mut storage);

        let actual = VarcharType::deserialize_from(&storage);
        assert_eq!(actual, null);

        assert_eq!(actual.is_null(), true);
    }

    #[test]
    fn basic_serialize_deserialize() {

        let mut storage = [0u8; 10000];
        let value = VarcharType::from("Hello world");

        assert_eq!(value.is_null(), false);

        value.serialize_to(&mut storage);

        let actual = VarcharType::deserialize_from(&storage);
        assert_eq!(actual, value);

        assert_eq!(actual.is_null(), false);

        assert_eq!(actual.as_string(), "Hello world".to_string());

    }
}
