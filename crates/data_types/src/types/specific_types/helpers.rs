
#[macro_export]
macro_rules! assert_in_range {
    ($db_type:ident, $value:expr, $larger_size:ty) => {
        assert!($db_type::MIN as $larger_size <= $value as $larger_size && $value as $larger_size <= $db_type::MAX as $larger_size, "value {} is out of range of {} and {}", $value, $db_type::MIN, $db_type::MAX)
    };
}

#[macro_export]
macro_rules! return_error_on_out_of_range {
    ($db_type:ident, $value:expr, $larger_size:ty) => {
        if ($db_type::MIN as $larger_size > $value as $larger_size || $value as $larger_size > $db_type::MAX as $larger_size) {
            return Err(crate::types::errors::InnerNumericConversionError::OutOfRange {
                value: $value.to_string(),
                min: $db_type::MIN.to_string(),
                max: $db_type::MAX.to_string(),
            }.into())
        }
    };
}

#[macro_export]
macro_rules! partial_eq_null {
    ($lhs_null:expr, $rhs_null:expr) => {
        // If both null - true
        if $lhs_null && $rhs_null {
            return true;
        }

        // If only one of them is null - false
        if $lhs_null || $rhs_null {
            return false;
        }
    };
}

#[cfg(test)]
mod tests {

    #[test]
    fn assert_in_range() {
        struct AWithMaxMin {}

        impl AWithMaxMin {
            const MIN: i16 = i16::MIN;
            const MAX: i16 = i16::MAX;
        }

        assert_in_range!(AWithMaxMin, 0u8, i16);
        assert_in_range!(AWithMaxMin, u8::MIN, i16);
        assert_in_range!(AWithMaxMin, u8::MAX, i16);
        assert_in_range!(AWithMaxMin, i8::MIN, i16);
        assert_in_range!(AWithMaxMin, i8::MAX, i16);
        assert_in_range!(AWithMaxMin, i16::MIN, i16);
        assert_in_range!(AWithMaxMin, i16::MAX, i16);
        assert_in_range!(AWithMaxMin, i16::MIN as f64, i16);
        assert_in_range!(AWithMaxMin, i16::MAX as f64, i16);
        assert_in_range!(AWithMaxMin, 0i128, i16);
        assert_in_range!(AWithMaxMin, 0u128, i16);
    }

    #[should_panic]
    #[test]
    fn assert_not_in_range_fail_with_negative() {
        struct AWithMaxMin {}

        impl AWithMaxMin {
            const MIN: u16 = u16::MIN;
            const MAX: u16 = u16::MAX;
        }

        assert_in_range!(AWithMaxMin, -1i8, i32);
    }

    #[should_panic]
    #[test]
    fn assert_not_in_range_fail_with_larger() {
        struct AWithMaxMin {}

        impl AWithMaxMin {
            const MIN: u16 = u16::MIN;
            const MAX: u16 = u16::MAX;
        }

        assert_in_range!(AWithMaxMin, u32::MAX, u32);
    }
}
