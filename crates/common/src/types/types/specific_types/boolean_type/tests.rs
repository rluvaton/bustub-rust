
#[cfg(test)]
mod tests {
    use crate::types::{BooleanType, BooleanUnderlyingType, ConversionDBTypeTrait};


    #[test]
    fn basic_equality() {
        let v_true = BooleanType::from(true);
        let v_false = BooleanType::from(false);
        let v_null = BooleanType::from(None);

        assert_eq!(v_true, true);
        assert_eq!(v_true, BooleanType::TRUE);
        assert_ne!(v_true, false);
        assert_ne!(v_true, BooleanType::FALSE);
        assert_ne!(v_true, None);
        assert_ne!(v_true, BooleanType::NULL);

        assert_eq!(v_false, false);
        assert_eq!(v_false, BooleanType::FALSE);
        assert_ne!(v_false, true);
        assert_ne!(v_false, BooleanType::TRUE);
        assert_ne!(v_false, None);
        assert_ne!(v_false, BooleanType::NULL);

        assert_eq!(v_null, None);
        assert_eq!(v_null, BooleanType::NULL);
        assert_ne!(v_null, true);
        assert_ne!(v_null, BooleanType::TRUE);
        assert_ne!(v_null, false);
        assert_ne!(v_null, BooleanType::FALSE);
    }

    #[test]
    fn basic_compare() {
        let v_true = BooleanType::from(true);
        let v_false = BooleanType::from(false);
        let v_null = BooleanType::from(None);

        // ###################### > #####################
        // True
        assert_eq!(v_true > v_false, true, "true > false");
        assert_eq!(v_true > v_null, true, "true > null");

        assert_eq!(v_true > v_true, false, "true should not be greater than true");
        assert_eq!(v_false > v_true, false, "true > false");
        assert_eq!(v_null > v_true, false, "true > null");

        // False
        assert_eq!(v_false > v_null, true, "false > null");

        assert_eq!(v_false > v_false, false, "false should not be greater than false");
        assert_eq!(v_null > v_false, false, "false > null");

        // Null
        assert_eq!(v_null > v_null, false, "null should not be greater than null");


        // ###################### >= #####################
        // True
        assert_eq!(v_true >= v_false, true, "true >= false");
        assert_eq!(v_true >= v_null, true, "true >= null");
        assert_eq!(v_true >= v_true, true, "true >= true");

        assert_eq!(v_false >= v_true, false, "true should be greater than or equal to false");
        assert_eq!(v_null >= v_true, false, "true should be greater than or equal to null");

        // False
        assert_eq!(v_false >= v_false, false, "false >= false");
        assert_eq!(v_false >= v_null, true, "false >= null");

        assert_eq!(v_null >= v_false, false, "false should be greater than or equal to null");

        // Null
        assert_eq!(v_null >= v_null, true, "null >= null");


        // ###################### < #####################
        // True
        assert_eq!(v_true < v_true, false, "true should not be smaller than true");
        assert_eq!(v_true < v_false, false, "true should not be smaller than false");
        assert_eq!(v_true < v_null, false, "true should not be smaller than null");


        // False
        assert_eq!(v_false < v_true, true, "false < true");

        assert_eq!(v_false < v_false, false, "false should not be smaller than false");
        assert_eq!(v_false < v_null, false, "false should not be smaller than null");

        // Null
        assert_eq!(v_null < v_true, true, "null < true");
        assert_eq!(v_null < v_false, true, "null < false");

        assert_eq!(v_null < v_null, false, "null should not be smaller than null");


        // ###################### <= #####################
        // True
        assert_eq!(v_true <= v_true, true, "true <= true");
        assert_eq!(v_true <= v_false, false, "true should not be smaller than or equal to false");
        assert_eq!(v_true <= v_null, false, "true should not be smaller than or equal to null");


        // False
        assert_eq!(v_false <= v_true, true, "false <= true");
        assert_eq!(v_false <= v_false, true, "false <= false");

        assert_eq!(v_false <= v_null, false, "false should not be smaller than or equal to null");

        // Null
        assert_eq!(v_null <= v_true, true, "null <= true");
        assert_eq!(v_null <= v_false, true, "null <= false");

        assert_eq!(v_null <= v_null, false, "null <= null");

    }


    #[test]
    fn basic_ordering_operations() {
        let db_type: [BooleanType; 3] = [Some(true).into(), Some(false).into(), None.into()];
        let numbers = [BooleanType::TRUE, BooleanType::FALSE, BooleanType::NULL];

        // Make sure we created correctly
        for i in 0..3 {
            assert_eq!(db_type[i].value, numbers[i]);
        }

        {
            let numbers_i8_sorted = numbers.clone();
            numbers_i8_sorted.sort();

            let db_type_sorted = db_type.clone();
            db_type_sorted.sort();

            let numbers_i8_parsed: [BooleanType; 3] = numbers_i8_sorted.map(|item| item.into());
            assert_eq!(numbers_i8_parsed, db_type_sorted);
        }

        {
            let max_number = numbers.iter().max().expect("Must have max item");

            let max_db_type = db_type.iter().max().expect("Must have max item");

            assert_eq!(max_db_type, max_number.into());
        }

        {
            let min_number = numbers.iter().min().expect("Must have min item");

            let min_db_type = db_type.iter().min().expect("Must have min item");

            assert_eq!(min_db_type, min_number.into());
        }
    }

    #[test]
    fn basic_serialize_deserialize() {

        let v_true = BooleanType::from(true);
        let v_false = BooleanType::from(false);
        let v_null = BooleanType::from(None);

        assert_eq!(v_true, true);
        assert_eq!(v_true, BooleanType::TRUE);
        assert_ne!(v_true, false);
        assert_ne!(v_true, BooleanType::FALSE);
        assert_ne!(v_true, None);
        assert_ne!(v_true, BooleanType::NULL);

        assert_eq!(v_false, false);
        assert_eq!(v_false, BooleanType::FALSE);
        assert_ne!(v_false, true);
        assert_ne!(v_false, BooleanType::TRUE);
        assert_ne!(v_false, None);
        assert_ne!(v_false, BooleanType::NULL);

        assert_eq!(v_null, None);
        assert_eq!(v_null, BooleanType::NULL);
        assert_ne!(v_null, true);
        assert_ne!(v_null, BooleanType::TRUE);
        assert_ne!(v_null, false);
        assert_ne!(v_null, BooleanType::FALSE);

        let expected: [BooleanUnderlyingType; 3] = [BooleanType::FALSE, BooleanType::TRUE, BooleanType::NULL];
        let db_type: [BooleanType; 3] = [BooleanType::new(BooleanType::FALSE), BooleanType::new(BooleanType::TRUE), BooleanType::new(BooleanType::NULL)];

        // Make sure we created correctly
        for i in 0..expected.len() {
            assert_eq!(db_type[i].value, expected[i]);
        }

        for i in 0..db_type.len() {
            let db_type_value = db_type[i];
            let expected_value = expected[i];

            let mut actual = [0u8; size_of::<BooleanUnderlyingType>()];
            let expected = expected_value.to_ne_bytes();

            {
                db_type[i].serialize_to(&mut actual);
                assert_eq!(actual, expected, "serialize(TinyIntType::new({})) == serialize({})", expected_value, expected_value);
            }

            {
                let deserialized = BooleanType::from(actual.as_slice());

                assert_eq!(deserialized, db_type_value, "BooleanType::from(serialize(BooleanType::new({})) == BooleanType::new({})", expected_value, expected_value);
                assert_eq!(deserialized, expected_value);
            }

            {
                let mut actual = [0u8; size_of::<BooleanUnderlyingType>() * 2];
                actual[..size_of::<BooleanUnderlyingType>()].copy_from_slice(expected.as_slice());
                let deserialized_from_larger = BooleanType::from(actual.as_slice());

                assert_eq!(deserialized_from_larger, db_type_value);
                assert_eq!(deserialized_from_larger, expected_value);
            }
        }
    }
}
