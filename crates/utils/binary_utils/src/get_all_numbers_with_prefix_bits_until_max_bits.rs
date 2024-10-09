use crate::GetNBits;
use std::ops::{Shl};

pub trait GetAllNumbersWithPrefixBitsUntilMaxBits {
    fn get_all_numbers_with_prefix_bits_until_max_bits(&self, prefix_bits: u8, max_bits: u8) -> impl Iterator<Item=Self>;
}


macro_rules! get_all_numbers_with_prefix_bit_until_max_bit_impl {
    ($($t:ty)+) => ($(
        impl GetAllNumbersWithPrefixBitsUntilMaxBits for $t {
            fn get_all_numbers_with_prefix_bits_until_max_bits(&self, prefix_bits: u8, max_bits: u8) -> impl Iterator<Item=$t> {
                assert!(prefix_bits <= <$t>::BITS as u8 && max_bits <= <$t>::BITS as u8, "prefix bits and max bits must not overflow");

                let size = max_bits - prefix_bits;
                let size = 2u64.pow(size as u32) as $t;

                let start = self.get_n_lsb_bits(prefix_bits);

                (0..size).map(move |value_to_append| {
                    // Shift value to append by prefix_bits
                    start | (value_to_append.shl(prefix_bits))
                }).into_iter()
            }

        }
    )+)
}

get_all_numbers_with_prefix_bit_until_max_bit_impl! { u8 u16 u32 u64 usize }


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_all_numbers() {
        let n = 0b0011101usize;
        assert_eq!(
            n.get_all_numbers_with_prefix_bits_until_max_bits(
                // start from 1101
                4,

                // end in 1111101
                7,
            ).collect::<Vec<_>>(),
            vec![
                0b0001101,
                0b0011101,
                0b0101101,
                0b0111101,
                0b1001101,
                0b1011101,
                0b1101101,
                0b1111101,
            ]
        )
    }

}
