pub trait IsBitOn {
    fn is_bit_on(&self, n: usize) -> bool;
}

macro_rules! is_bit_om_impl {
    ($($t:ty)+) => ($(
        impl IsBitOn for $t {
            fn is_bit_on(&self, n: usize) -> bool {
                (self >> (n - 1)) & 1 == 1
            }
        }
    )+)
}

is_bit_om_impl! { u8 u16 u32 u64 }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_bit_on_u8() {
        let num: u8 = 0b10010000;

        assert_eq!(num.is_bit_on(1), false);
        assert_eq!(num.is_bit_on(2), false);
        assert_eq!(num.is_bit_on(3), false);
        assert_eq!(num.is_bit_on(4), false);
        assert_eq!(num.is_bit_on(5), true);
        assert_eq!(num.is_bit_on(6), false);
        assert_eq!(num.is_bit_on(7), false);
        assert_eq!(num.is_bit_on(8), true);
    }

    #[test]
    fn test_is_bit_on_u16() {
        let num: u16 = 0b1001111001110000;

        assert_eq!(num.is_bit_on(1), false);
        assert_eq!(num.is_bit_on(2), false);
        assert_eq!(num.is_bit_on(3), false);
        assert_eq!(num.is_bit_on(4), false);
        assert_eq!(num.is_bit_on(5), true);
        assert_eq!(num.is_bit_on(6), true);
        assert_eq!(num.is_bit_on(7), true);
        assert_eq!(num.is_bit_on(8), false);
        assert_eq!(num.is_bit_on(9), false);
        assert_eq!(num.is_bit_on(10), true);
        assert_eq!(num.is_bit_on(11), true);
        assert_eq!(num.is_bit_on(12), true);
        assert_eq!(num.is_bit_on(13), true);
        assert_eq!(num.is_bit_on(14), false);
        assert_eq!(num.is_bit_on(15), false);
        assert_eq!(num.is_bit_on(16), true);
    }

    #[test]
    fn test_is_bit_on_u32() {
        let num: u32 = 0b10011110000010001111111000000000;

        assert_eq!(num.is_bit_on(1), false);
        assert_eq!(num.is_bit_on(2), false);
        assert_eq!(num.is_bit_on(3), false);
        assert_eq!(num.is_bit_on(4), false);
        assert_eq!(num.is_bit_on(5), false);
        assert_eq!(num.is_bit_on(6), false);
        assert_eq!(num.is_bit_on(7), false);
        assert_eq!(num.is_bit_on(8), false);
        assert_eq!(num.is_bit_on(9), false);
        assert_eq!(num.is_bit_on(10), true);
        assert_eq!(num.is_bit_on(11), true);
        assert_eq!(num.is_bit_on(12), true);
        assert_eq!(num.is_bit_on(13), true);
        assert_eq!(num.is_bit_on(14), true);
        assert_eq!(num.is_bit_on(15), true);
        assert_eq!(num.is_bit_on(16), true);
        assert_eq!(num.is_bit_on(17), false);
        assert_eq!(num.is_bit_on(18), false);
        assert_eq!(num.is_bit_on(19), false);
        assert_eq!(num.is_bit_on(20), true);
        assert_eq!(num.is_bit_on(21), false);
        assert_eq!(num.is_bit_on(22), false);
        assert_eq!(num.is_bit_on(23), false);
        assert_eq!(num.is_bit_on(24), false);
        assert_eq!(num.is_bit_on(25), false);
        assert_eq!(num.is_bit_on(26), true);
        assert_eq!(num.is_bit_on(27), true);
        assert_eq!(num.is_bit_on(28), true);
        assert_eq!(num.is_bit_on(29), true);
        assert_eq!(num.is_bit_on(30), false);
        assert_eq!(num.is_bit_on(31), false);
        assert_eq!(num.is_bit_on(32), true);
    }

    #[test]
    fn test_is_bit_on_u64() {
        let num: u64 = 0b1001111000001000100000010000100110100001111111110000000000000000;

        assert_eq!(num.is_bit_on(1), false);
        assert_eq!(num.is_bit_on(2), false);
        assert_eq!(num.is_bit_on(3), false);
        assert_eq!(num.is_bit_on(4), false);
        assert_eq!(num.is_bit_on(5), false);
        assert_eq!(num.is_bit_on(6), false);
        assert_eq!(num.is_bit_on(7), false);
        assert_eq!(num.is_bit_on(8), false);
        assert_eq!(num.is_bit_on(9), false);
        assert_eq!(num.is_bit_on(10), false);
        assert_eq!(num.is_bit_on(11), false);
        assert_eq!(num.is_bit_on(12), false);
        assert_eq!(num.is_bit_on(13), false);
        assert_eq!(num.is_bit_on(14), false);
        assert_eq!(num.is_bit_on(15), false);
        assert_eq!(num.is_bit_on(16), false);
        assert_eq!(num.is_bit_on(17), true);
        assert_eq!(num.is_bit_on(18), true);
        assert_eq!(num.is_bit_on(19), true);
        assert_eq!(num.is_bit_on(20), true);
        assert_eq!(num.is_bit_on(21), true);
        assert_eq!(num.is_bit_on(22), true);
        assert_eq!(num.is_bit_on(23), true);
        assert_eq!(num.is_bit_on(24), true);
        assert_eq!(num.is_bit_on(25), true);
        assert_eq!(num.is_bit_on(26), false);
        assert_eq!(num.is_bit_on(27), false);
        assert_eq!(num.is_bit_on(28), false);
        assert_eq!(num.is_bit_on(29), false);
        assert_eq!(num.is_bit_on(30), true);
        assert_eq!(num.is_bit_on(31), false);
        assert_eq!(num.is_bit_on(32), true);
        assert_eq!(num.is_bit_on(33), true);
        assert_eq!(num.is_bit_on(34), false);
        assert_eq!(num.is_bit_on(35), false);
        assert_eq!(num.is_bit_on(36), true);
        assert_eq!(num.is_bit_on(37), false);
        assert_eq!(num.is_bit_on(38), false);
        assert_eq!(num.is_bit_on(39), false);
        assert_eq!(num.is_bit_on(40), false);
        assert_eq!(num.is_bit_on(41), true);
        assert_eq!(num.is_bit_on(42), false);
        assert_eq!(num.is_bit_on(43), false);
        assert_eq!(num.is_bit_on(44), false);
        assert_eq!(num.is_bit_on(45), false);
        assert_eq!(num.is_bit_on(46), false);
        assert_eq!(num.is_bit_on(47), false);
        assert_eq!(num.is_bit_on(48), true);
        assert_eq!(num.is_bit_on(49), false);
        assert_eq!(num.is_bit_on(50), false);
        assert_eq!(num.is_bit_on(51), false);
        assert_eq!(num.is_bit_on(52), true);
        assert_eq!(num.is_bit_on(53), false);
        assert_eq!(num.is_bit_on(54), false);
        assert_eq!(num.is_bit_on(55), false);
        assert_eq!(num.is_bit_on(56), false);
        assert_eq!(num.is_bit_on(57), false);
        assert_eq!(num.is_bit_on(58), true);
        assert_eq!(num.is_bit_on(59), true);
        assert_eq!(num.is_bit_on(60), true);
        assert_eq!(num.is_bit_on(61), true);
        assert_eq!(num.is_bit_on(62), false);
        assert_eq!(num.is_bit_on(63), false);
        assert_eq!(num.is_bit_on(64), true);
    }
}
