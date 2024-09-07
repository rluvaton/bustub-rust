#[macro_export]
macro_rules! assert_in_range {
    ($db_type:ident, $value:expr, $larger_size:ty) => {
        assert!($db_type::MIN as $larger_size <= $value as $larger_size && $value as $larger_size <= $db_type::MAX as $larger_size, "value {} is out of range of {} and {}", $value, $db_type::MIN, $db_type::MAX)
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
