pub trait GetNBits {
    /// Get N Most significant bits
    ///
    /// # Arguments
    ///
    /// * `n`: the number of bits to take from the left
    ///
    /// returns: Self
    ///
    /// # Examples
    ///
    /// ```
    /// use binary_utils::GetNBits;
    ///
    /// let num: u8 = 0b10010000;
    ///
    /// assert_eq!(num.get_n_msb_bits(1), 0b1);
    /// assert_eq!(num.get_n_msb_bits(2), 0b10);
    /// assert_eq!(num.get_n_msb_bits(3), 0b100);
    /// assert_eq!(num.get_n_msb_bits(4), 0b1001);
    /// assert_eq!(num.get_n_msb_bits(5), 0b10010);
    /// assert_eq!(num.get_n_msb_bits(6), 0b100100);
    /// assert_eq!(num.get_n_msb_bits(7), 0b1001000);
    /// assert_eq!(num.get_n_msb_bits(8), num);
    /// ```
    fn get_n_msb_bits(&self, n: u8) -> Self;

    /// Get N Most significant bits without checking if the number `0 < n < Self size of` (e.g. for u64 the number of bits is 64 so `0 < n < 64`)
    ///
    /// # Arguments
    ///
    /// * `n`: the number of bits to take from the left
    ///
    /// returns: Self
    ///
    /// # Examples
    ///
    /// ```
    /// use binary_utils::GetNBits;
    /// let num: u8 = 0b10010000;
    ///
    /// unsafe {
    ///     assert_eq!(num.get_n_msb_bits_unchecked(1), 0b1);
    ///     assert_eq!(num.get_n_msb_bits_unchecked(2), 0b10);
    ///     assert_eq!(num.get_n_msb_bits_unchecked(3), 0b100);
    ///     assert_eq!(num.get_n_msb_bits_unchecked(4), 0b1001);
    ///     assert_eq!(num.get_n_msb_bits_unchecked(5), 0b10010);
    ///     assert_eq!(num.get_n_msb_bits_unchecked(6), 0b100100);
    ///     assert_eq!(num.get_n_msb_bits_unchecked(7), 0b1001000);
    ///     assert_eq!(num.get_n_msb_bits_unchecked(8), num);
    /// }
    /// ```
    ///
    /// # Safety
    /// `n` must be in range `0 < n < Self size of`
    ///
    /// e.g. for u64 the number of bits is 64 so `0 < n < 64`
    ///
    unsafe fn get_n_msb_bits_unchecked(&self, n: u8) -> Self;

    /// Get N Least significant bits
    ///
    /// # Arguments
    ///
    /// * `n`: the number of bits to take from the right
    ///
    /// returns: Self
    ///
    /// # Examples
    ///
    /// ```
    /// use binary_utils::GetNBits;
    /// let num = 0b1010u8;
    ///
    /// assert_eq!(num.get_n_lsb_bits(1), 0b0);
    /// assert_eq!(num.get_n_lsb_bits(2), 0b10);
    /// assert_eq!(num.get_n_lsb_bits(3), 0b010);
    /// assert_eq!(num.get_n_lsb_bits(4), 0b1010);
    /// ```
    fn get_n_lsb_bits(&self, n: u8) -> Self;

    /// Get N Least significant bits without checking if the number `0 < n < Self size of` (e.g. for u64 the number of bits is 64 so `0 < n < 64`)
    ///
    /// # Arguments
    ///
    /// * `n`: the number of bits to take from the right
    ///
    /// returns: Self
    ///
    /// # Examples
    ///
    /// ```
    /// use binary_utils::GetNBits;
    /// let num = 0b1010u8;
    ///
    /// unsafe {
    ///     assert_eq!(num.get_n_lsb_bits_unchecked(1), 0b0);
    ///     assert_eq!(num.get_n_lsb_bits_unchecked(2), 0b10);
    ///     assert_eq!(num.get_n_lsb_bits_unchecked(3), 0b010);
    ///     assert_eq!(num.get_n_lsb_bits_unchecked(4), 0b1010);
    /// }
    /// ```
    ///
    /// # Safety
    /// `n` must be in range `0 < n < Self size of`
    ///
    /// e.g. for u64 the number of bits is 64 so `0 < n < 64`
    ///
    unsafe fn get_n_lsb_bits_unchecked(&self, n: u8) -> Self;
}

macro_rules! get_n_bits_impl {
    ($($t:ty)+) => ($(
        impl GetNBits for $t {
            fn get_n_msb_bits(&self, n: u8) -> Self {
                assert!(n <= <$t>::BITS as u8, "n ({}) n <= {}", n, <$t>::BITS);

                unsafe { self.get_n_msb_bits_unchecked(n) }
            }

            #[inline]
            unsafe fn get_n_msb_bits_unchecked(&self, n: u8) -> Self {
                if n == 0 {
                    return 0 as $t;
                }

                if n == <$t>::BITS as u8 {
                    return *self
                }
                let mask = (1 << n) - 1;
                (self >> (<$t>::BITS as u8 - n)) & mask
            }

            fn get_n_lsb_bits(&self, n: u8) -> Self {
                assert!(n <= <$t>::BITS as u8, "n ({}) n <= {}", n, <$t>::BITS);

                unsafe { self.get_n_lsb_bits_unchecked(n) }
            }

            #[inline]
            unsafe fn get_n_lsb_bits_unchecked(&self, n: u8) -> Self {
                if n == 0 {
                    return 0 as $t;
                }

                if n == <$t>::BITS as u8 {
                    return *self
                }
                let mask = (1 << n) - 1;
                self & mask
            }
        }
    )+)
}

get_n_bits_impl! { u8 u16 u32 u64 usize }


#[cfg(test)]
mod tests {
    use crate::GetNBits;

    #[test]
    fn get_n_msb_bits_u64() {
        let num: u64 = 0b1001111000001000100000010000100110100001111111110000000000000000;

        assert_eq!(num.get_n_msb_bits(0), 0b0);
        assert_eq!(num.get_n_msb_bits(1), 0b1);
        assert_eq!(num.get_n_msb_bits(2), 0b10);
        assert_eq!(num.get_n_msb_bits(3), 0b100);
        assert_eq!(num.get_n_msb_bits(4), 0b1001);
        assert_eq!(num.get_n_msb_bits(5), 0b10011);
        assert_eq!(num.get_n_msb_bits(6), 0b100111);
        assert_eq!(num.get_n_msb_bits(7), 0b1001111);
        assert_eq!(num.get_n_msb_bits(8), 0b10011110);
        assert_eq!(num.get_n_msb_bits(9), 0b100111100);
        assert_eq!(num.get_n_msb_bits(10), 0b1001111000);
        assert_eq!(num.get_n_msb_bits(11), 0b10011110000);
        assert_eq!(num.get_n_msb_bits(12), 0b100111100000);
        assert_eq!(num.get_n_msb_bits(13), 0b1001111000001);
        assert_eq!(num.get_n_msb_bits(14), 0b10011110000010);
        assert_eq!(num.get_n_msb_bits(15), 0b100111100000100);
        assert_eq!(num.get_n_msb_bits(16), 0b1001111000001000);
        assert_eq!(num.get_n_msb_bits(17), 0b10011110000010001);
        assert_eq!(num.get_n_msb_bits(18), 0b100111100000100010);
        assert_eq!(num.get_n_msb_bits(19), 0b1001111000001000100);
        assert_eq!(num.get_n_msb_bits(20), 0b10011110000010001000);
        assert_eq!(num.get_n_msb_bits(21), 0b100111100000100010000);
        assert_eq!(num.get_n_msb_bits(22), 0b1001111000001000100000);
        assert_eq!(num.get_n_msb_bits(23), 0b10011110000010001000000);
        assert_eq!(num.get_n_msb_bits(24), 0b100111100000100010000001);
        assert_eq!(num.get_n_msb_bits(25), 0b1001111000001000100000010);
        assert_eq!(num.get_n_msb_bits(26), 0b10011110000010001000000100);
        assert_eq!(num.get_n_msb_bits(27), 0b100111100000100010000001000);
        assert_eq!(num.get_n_msb_bits(28), 0b1001111000001000100000010000);
        assert_eq!(num.get_n_msb_bits(29), 0b10011110000010001000000100001);
        assert_eq!(num.get_n_msb_bits(30), 0b100111100000100010000001000010);
        assert_eq!(num.get_n_msb_bits(31), 0b1001111000001000100000010000100);
        assert_eq!(num.get_n_msb_bits(32), 0b10011110000010001000000100001001);
        assert_eq!(num.get_n_msb_bits(33), 0b100111100000100010000001000010011);
        assert_eq!(num.get_n_msb_bits(34), 0b1001111000001000100000010000100110);
        assert_eq!(num.get_n_msb_bits(35), 0b10011110000010001000000100001001101);
        assert_eq!(num.get_n_msb_bits(36), 0b100111100000100010000001000010011010);
        assert_eq!(num.get_n_msb_bits(37), 0b1001111000001000100000010000100110100);
        assert_eq!(num.get_n_msb_bits(38), 0b10011110000010001000000100001001101000);
        assert_eq!(num.get_n_msb_bits(39), 0b100111100000100010000001000010011010000);
        assert_eq!(num.get_n_msb_bits(40), 0b1001111000001000100000010000100110100001);
        assert_eq!(num.get_n_msb_bits(41), 0b10011110000010001000000100001001101000011);
        assert_eq!(num.get_n_msb_bits(42), 0b100111100000100010000001000010011010000111);
        assert_eq!(num.get_n_msb_bits(43), 0b1001111000001000100000010000100110100001111);
        assert_eq!(num.get_n_msb_bits(44), 0b10011110000010001000000100001001101000011111);
        assert_eq!(num.get_n_msb_bits(45), 0b100111100000100010000001000010011010000111111);
        assert_eq!(num.get_n_msb_bits(46), 0b1001111000001000100000010000100110100001111111);
        assert_eq!(num.get_n_msb_bits(47), 0b10011110000010001000000100001001101000011111111);
        assert_eq!(num.get_n_msb_bits(48), 0b100111100000100010000001000010011010000111111111);
        assert_eq!(num.get_n_msb_bits(49), 0b1001111000001000100000010000100110100001111111110);
        assert_eq!(num.get_n_msb_bits(50), 0b10011110000010001000000100001001101000011111111100);
        assert_eq!(num.get_n_msb_bits(51), 0b100111100000100010000001000010011010000111111111000);
        assert_eq!(num.get_n_msb_bits(52), 0b1001111000001000100000010000100110100001111111110000);
        assert_eq!(num.get_n_msb_bits(53), 0b10011110000010001000000100001001101000011111111100000);
        assert_eq!(num.get_n_msb_bits(54), 0b100111100000100010000001000010011010000111111111000000);
        assert_eq!(num.get_n_msb_bits(55), 0b1001111000001000100000010000100110100001111111110000000);
        assert_eq!(num.get_n_msb_bits(56), 0b10011110000010001000000100001001101000011111111100000000);
        assert_eq!(num.get_n_msb_bits(57), 0b100111100000100010000001000010011010000111111111000000000);
        assert_eq!(num.get_n_msb_bits(58), 0b1001111000001000100000010000100110100001111111110000000000);
        assert_eq!(num.get_n_msb_bits(59), 0b10011110000010001000000100001001101000011111111100000000000);
        assert_eq!(num.get_n_msb_bits(60), 0b100111100000100010000001000010011010000111111111000000000000);
        assert_eq!(num.get_n_msb_bits(61), 0b1001111000001000100000010000100110100001111111110000000000000);
        assert_eq!(num.get_n_msb_bits(62), 0b10011110000010001000000100001001101000011111111100000000000000);
        assert_eq!(num.get_n_msb_bits(63), 0b100111100000100010000001000010011010000111111111000000000000000);
        assert_eq!(num.get_n_msb_bits(64), num);
    }

    #[should_panic]
    #[test]
    fn get_n_msb_bits_u64_should_panic_out_of_range_64() {
        1u8.get_n_msb_bits(64);
    }

    #[should_panic]
    #[test]
    fn get_n_msb_bits_u64_should_panic_out_of_range_65() {
        1u8.get_n_msb_bits(65);
    }

    #[test]
    fn get_n_lsb_bits_u64() {
        let num: u64 = 0b1001111000001000100000010000100110100001111111110000000000000000;

        assert_eq!(num.get_n_lsb_bits(0), 0b0);
        assert_eq!(num.get_n_lsb_bits(1), 0b0);
        assert_eq!(num.get_n_lsb_bits(2), 0b00);
        assert_eq!(num.get_n_lsb_bits(3), 0b000);
        assert_eq!(num.get_n_lsb_bits(4), 0b0000);
        assert_eq!(num.get_n_lsb_bits(5), 0b00000);
        assert_eq!(num.get_n_lsb_bits(6), 0b000000);
        assert_eq!(num.get_n_lsb_bits(7), 0b0000000);
        assert_eq!(num.get_n_lsb_bits(8), 0b00000000);
        assert_eq!(num.get_n_lsb_bits(9), 0b000000000);
        assert_eq!(num.get_n_lsb_bits(10), 0b0000000000);
        assert_eq!(num.get_n_lsb_bits(11), 0b00000000000);
        assert_eq!(num.get_n_lsb_bits(12), 0b000000000000);
        assert_eq!(num.get_n_lsb_bits(13), 0b0000000000000);
        assert_eq!(num.get_n_lsb_bits(14), 0b00000000000000);
        assert_eq!(num.get_n_lsb_bits(15), 0b000000000000000);
        assert_eq!(num.get_n_lsb_bits(16), 0b0000000000000000);
        assert_eq!(num.get_n_lsb_bits(17), 0b10000000000000000);
        assert_eq!(num.get_n_lsb_bits(18), 0b110000000000000000);
        assert_eq!(num.get_n_lsb_bits(19), 0b1110000000000000000);
        assert_eq!(num.get_n_lsb_bits(20), 0b11110000000000000000);
        assert_eq!(num.get_n_lsb_bits(21), 0b111110000000000000000);
        assert_eq!(num.get_n_lsb_bits(22), 0b1111110000000000000000);
        assert_eq!(num.get_n_lsb_bits(23), 0b11111110000000000000000);
        assert_eq!(num.get_n_lsb_bits(24), 0b111111110000000000000000);
        assert_eq!(num.get_n_lsb_bits(25), 0b1111111110000000000000000);
        assert_eq!(num.get_n_lsb_bits(26), 0b01111111110000000000000000);
        assert_eq!(num.get_n_lsb_bits(27), 0b001111111110000000000000000);
        assert_eq!(num.get_n_lsb_bits(28), 0b0001111111110000000000000000);
        assert_eq!(num.get_n_lsb_bits(29), 0b00001111111110000000000000000);
        assert_eq!(num.get_n_lsb_bits(30), 0b100001111111110000000000000000);
        assert_eq!(num.get_n_lsb_bits(31), 0b0100001111111110000000000000000);
        assert_eq!(num.get_n_lsb_bits(32), 0b10100001111111110000000000000000);
        assert_eq!(num.get_n_lsb_bits(33), 0b110100001111111110000000000000000);
        assert_eq!(num.get_n_lsb_bits(34), 0b0110100001111111110000000000000000);
        assert_eq!(num.get_n_lsb_bits(35), 0b00110100001111111110000000000000000);
        assert_eq!(num.get_n_lsb_bits(36), 0b100110100001111111110000000000000000);
        assert_eq!(num.get_n_lsb_bits(37), 0b0100110100001111111110000000000000000);
        assert_eq!(num.get_n_lsb_bits(38), 0b00100110100001111111110000000000000000);
        assert_eq!(num.get_n_lsb_bits(39), 0b000100110100001111111110000000000000000);
        assert_eq!(num.get_n_lsb_bits(40), 0b0000100110100001111111110000000000000000);
        assert_eq!(num.get_n_lsb_bits(41), 0b10000100110100001111111110000000000000000);
        assert_eq!(num.get_n_lsb_bits(42), 0b010000100110100001111111110000000000000000);
        assert_eq!(num.get_n_lsb_bits(43), 0b0010000100110100001111111110000000000000000);
        assert_eq!(num.get_n_lsb_bits(44), 0b00010000100110100001111111110000000000000000);
        assert_eq!(num.get_n_lsb_bits(45), 0b000010000100110100001111111110000000000000000);
        assert_eq!(num.get_n_lsb_bits(46), 0b0000010000100110100001111111110000000000000000);
        assert_eq!(num.get_n_lsb_bits(47), 0b00000010000100110100001111111110000000000000000);
        assert_eq!(num.get_n_lsb_bits(48), 0b100000010000100110100001111111110000000000000000);
        assert_eq!(num.get_n_lsb_bits(49), 0b0100000010000100110100001111111110000000000000000);
        assert_eq!(num.get_n_lsb_bits(50), 0b00100000010000100110100001111111110000000000000000);
        assert_eq!(num.get_n_lsb_bits(51), 0b000100000010000100110100001111111110000000000000000);
        assert_eq!(num.get_n_lsb_bits(52), 0b1000100000010000100110100001111111110000000000000000);
        assert_eq!(num.get_n_lsb_bits(53), 0b01000100000010000100110100001111111110000000000000000);
        assert_eq!(num.get_n_lsb_bits(54), 0b001000100000010000100110100001111111110000000000000000);
        assert_eq!(num.get_n_lsb_bits(55), 0b0001000100000010000100110100001111111110000000000000000);
        assert_eq!(num.get_n_lsb_bits(56), 0b00001000100000010000100110100001111111110000000000000000);
        assert_eq!(num.get_n_lsb_bits(57), 0b000001000100000010000100110100001111111110000000000000000);
        assert_eq!(num.get_n_lsb_bits(58), 0b1000001000100000010000100110100001111111110000000000000000);
        assert_eq!(num.get_n_lsb_bits(59), 0b11000001000100000010000100110100001111111110000000000000000);
        assert_eq!(num.get_n_lsb_bits(60), 0b111000001000100000010000100110100001111111110000000000000000);
        assert_eq!(num.get_n_lsb_bits(61), 0b1111000001000100000010000100110100001111111110000000000000000);
        assert_eq!(num.get_n_lsb_bits(62), 0b01111000001000100000010000100110100001111111110000000000000000);
        assert_eq!(num.get_n_lsb_bits(63), 0b001111000001000100000010000100110100001111111110000000000000000);
        assert_eq!(num.get_n_lsb_bits(64), num);
    }

    #[should_panic]
    #[test]
    fn get_n_lsb_bits_u64_should_panic_out_of_range_64() {
        1u8.get_n_lsb_bits(64);
    }

    #[should_panic]
    #[test]
    fn get_n_lsb_bits_u64_should_panic_out_of_range_65() {
        1u8.get_n_lsb_bits(65);
    }

    #[should_panic]
    #[test]
    fn get_n_lsb_bits_u64_should_panic_out_of_range_0() {
        1u8.get_n_lsb_bits(0);
    }

    #[test]
    fn get_n_msb_bits_u32() {
        let num: u32 = 0b10011110000010001111111000000000;

        assert_eq!(num.get_n_msb_bits(0), 0b0);
        assert_eq!(num.get_n_msb_bits(1), 0b1);
        assert_eq!(num.get_n_msb_bits(2), 0b10);
        assert_eq!(num.get_n_msb_bits(3), 0b100);
        assert_eq!(num.get_n_msb_bits(4), 0b1001);
        assert_eq!(num.get_n_msb_bits(5), 0b10011);
        assert_eq!(num.get_n_msb_bits(6), 0b100111);
        assert_eq!(num.get_n_msb_bits(7), 0b1001111);
        assert_eq!(num.get_n_msb_bits(8), 0b10011110);
        assert_eq!(num.get_n_msb_bits(9), 0b100111100);
        assert_eq!(num.get_n_msb_bits(10), 0b1001111000);
        assert_eq!(num.get_n_msb_bits(11), 0b10011110000);
        assert_eq!(num.get_n_msb_bits(12), 0b100111100000);
        assert_eq!(num.get_n_msb_bits(13), 0b1001111000001);
        assert_eq!(num.get_n_msb_bits(14), 0b10011110000010);
        assert_eq!(num.get_n_msb_bits(15), 0b100111100000100);
        assert_eq!(num.get_n_msb_bits(16), 0b1001111000001000);
        assert_eq!(num.get_n_msb_bits(17), 0b10011110000010001);
        assert_eq!(num.get_n_msb_bits(18), 0b100111100000100011);
        assert_eq!(num.get_n_msb_bits(19), 0b1001111000001000111);
        assert_eq!(num.get_n_msb_bits(20), 0b10011110000010001111);
        assert_eq!(num.get_n_msb_bits(21), 0b100111100000100011111);
        assert_eq!(num.get_n_msb_bits(22), 0b1001111000001000111111);
        assert_eq!(num.get_n_msb_bits(23), 0b10011110000010001111111);
        assert_eq!(num.get_n_msb_bits(24), 0b100111100000100011111110);
        assert_eq!(num.get_n_msb_bits(25), 0b1001111000001000111111100);
        assert_eq!(num.get_n_msb_bits(26), 0b10011110000010001111111000);
        assert_eq!(num.get_n_msb_bits(27), 0b100111100000100011111110000);
        assert_eq!(num.get_n_msb_bits(28), 0b1001111000001000111111100000);
        assert_eq!(num.get_n_msb_bits(29), 0b10011110000010001111111000000);
        assert_eq!(num.get_n_msb_bits(30), 0b100111100000100011111110000000);
        assert_eq!(num.get_n_msb_bits(31), 0b1001111000001000111111100000000);
        assert_eq!(num.get_n_msb_bits(32), num);
    }

    #[test]
    fn get_n_lsb_bits_u32() {
        let num: u32 = 0b10011110000010001111111000000000;

        assert_eq!(num.get_n_lsb_bits(0), 0b0);
        assert_eq!(num.get_n_lsb_bits(1), 0b0);
        assert_eq!(num.get_n_lsb_bits(2), 0b00);
        assert_eq!(num.get_n_lsb_bits(3), 0b000);
        assert_eq!(num.get_n_lsb_bits(4), 0b0000);
        assert_eq!(num.get_n_lsb_bits(5), 0b00000);
        assert_eq!(num.get_n_lsb_bits(6), 0b000000);
        assert_eq!(num.get_n_lsb_bits(7), 0b0000000);
        assert_eq!(num.get_n_lsb_bits(8), 0b00000000);
        assert_eq!(num.get_n_lsb_bits(9), 0b000000000);
        assert_eq!(num.get_n_lsb_bits(10), 0b1000000000);
        assert_eq!(num.get_n_lsb_bits(11), 0b11000000000);
        assert_eq!(num.get_n_lsb_bits(12), 0b111000000000);
        assert_eq!(num.get_n_lsb_bits(13), 0b1111000000000);
        assert_eq!(num.get_n_lsb_bits(14), 0b11111000000000);
        assert_eq!(num.get_n_lsb_bits(15), 0b111111000000000);
        assert_eq!(num.get_n_lsb_bits(16), 0b1111111000000000);
        assert_eq!(num.get_n_lsb_bits(17), 0b01111111000000000);
        assert_eq!(num.get_n_lsb_bits(18), 0b001111111000000000);
        assert_eq!(num.get_n_lsb_bits(19), 0b0001111111000000000);
        assert_eq!(num.get_n_lsb_bits(20), 0b10001111111000000000);
        assert_eq!(num.get_n_lsb_bits(21), 0b010001111111000000000);
        assert_eq!(num.get_n_lsb_bits(22), 0b0010001111111000000000);
        assert_eq!(num.get_n_lsb_bits(23), 0b00010001111111000000000);
        assert_eq!(num.get_n_lsb_bits(24), 0b000010001111111000000000);
        assert_eq!(num.get_n_lsb_bits(25), 0b0000010001111111000000000);
        assert_eq!(num.get_n_lsb_bits(26), 0b10000010001111111000000000);
        assert_eq!(num.get_n_lsb_bits(27), 0b110000010001111111000000000);
        assert_eq!(num.get_n_lsb_bits(28), 0b1110000010001111111000000000);
        assert_eq!(num.get_n_lsb_bits(29), 0b11110000010001111111000000000);
        assert_eq!(num.get_n_lsb_bits(30), 0b011110000010001111111000000000);
        assert_eq!(num.get_n_lsb_bits(31), 0b0011110000010001111111000000000);
        assert_eq!(num.get_n_lsb_bits(32), num);
    }

    #[test]
    fn get_n_msb_bits_u16() {
        let num: u16 = 0b1001111001110000;

        assert_eq!(num.get_n_msb_bits(0), 0b0);
        assert_eq!(num.get_n_msb_bits(1), 0b1);
        assert_eq!(num.get_n_msb_bits(2), 0b10);
        assert_eq!(num.get_n_msb_bits(3), 0b100);
        assert_eq!(num.get_n_msb_bits(4), 0b1001);
        assert_eq!(num.get_n_msb_bits(5), 0b10011);
        assert_eq!(num.get_n_msb_bits(6), 0b100111);
        assert_eq!(num.get_n_msb_bits(7), 0b1001111);
        assert_eq!(num.get_n_msb_bits(8), 0b10011110);
        assert_eq!(num.get_n_msb_bits(9), 0b100111100);
        assert_eq!(num.get_n_msb_bits(10), 0b1001111001);
        assert_eq!(num.get_n_msb_bits(11), 0b10011110011);
        assert_eq!(num.get_n_msb_bits(12), 0b100111100111);
        assert_eq!(num.get_n_msb_bits(13), 0b1001111001110);
        assert_eq!(num.get_n_msb_bits(14), 0b10011110011100);
        assert_eq!(num.get_n_msb_bits(15), 0b100111100111000);
        assert_eq!(num.get_n_msb_bits(16), num);
    }

    #[test]
    fn get_n_lsb_bits_u16() {
        let num: u16 = 0b1001111001110000;

        assert_eq!(num.get_n_lsb_bits(0), 0b0);
        assert_eq!(num.get_n_lsb_bits(1), 0b0);
        assert_eq!(num.get_n_lsb_bits(2), 0b00);
        assert_eq!(num.get_n_lsb_bits(3), 0b000);
        assert_eq!(num.get_n_lsb_bits(4), 0b0000);
        assert_eq!(num.get_n_lsb_bits(5), 0b10000);
        assert_eq!(num.get_n_lsb_bits(6), 0b110000);
        assert_eq!(num.get_n_lsb_bits(7), 0b1110000);
        assert_eq!(num.get_n_lsb_bits(8), 0b01110000);
        assert_eq!(num.get_n_lsb_bits(9), 0b001110000);
        assert_eq!(num.get_n_lsb_bits(10), 0b1001110000);
        assert_eq!(num.get_n_lsb_bits(11), 0b11001110000);
        assert_eq!(num.get_n_lsb_bits(12), 0b111001110000);
        assert_eq!(num.get_n_lsb_bits(13), 0b1111001110000);
        assert_eq!(num.get_n_lsb_bits(14), 0b01111001110000);
        assert_eq!(num.get_n_lsb_bits(15), 0b001111001110000);
        assert_eq!(num.get_n_lsb_bits(16), num);
    }

    #[test]
    fn get_n_msb_bits_u8() {
        let num: u8 = 0b10010000;

        assert_eq!(num.get_n_msb_bits(0), 0b0);
        assert_eq!(num.get_n_msb_bits(1), 0b1);
        assert_eq!(num.get_n_msb_bits(2), 0b10);
        assert_eq!(num.get_n_msb_bits(3), 0b100);
        assert_eq!(num.get_n_msb_bits(4), 0b1001);
        assert_eq!(num.get_n_msb_bits(5), 0b10010);
        assert_eq!(num.get_n_msb_bits(6), 0b100100);
        assert_eq!(num.get_n_msb_bits(7), 0b1001000);
        assert_eq!(num.get_n_msb_bits(8), num);
    }

    #[test]
    fn get_n_lsb_bits_u8() {
        let num: u8 = 0b10010000;

        assert_eq!(num.get_n_lsb_bits(0), 0b0);
        assert_eq!(num.get_n_lsb_bits(1), 0b0);
        assert_eq!(num.get_n_lsb_bits(2), 0b00);
        assert_eq!(num.get_n_lsb_bits(3), 0b000);
        assert_eq!(num.get_n_lsb_bits(4), 0b0000);
        assert_eq!(num.get_n_lsb_bits(5), 0b10000);
        assert_eq!(num.get_n_lsb_bits(6), 0b010000);
        assert_eq!(num.get_n_lsb_bits(7), 0b0010000);
        assert_eq!(num.get_n_lsb_bits(8), num);
    }
}
