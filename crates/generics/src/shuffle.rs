use std::ops::Range;
use rand::seq::SliceRandom;
use rand::thread_rng;

/// Shuffle, implemented for range for easier testing
pub trait Shuffle<T> {
    fn shuffle(&mut self) -> Vec<T>;
}



// Once Step is stable we can do the following
// impl<T> Shuffle<T> for Range<T> where [T]: SliceRandom {
//     fn shuffle(&mut self) -> Vec<T> {
//         let mut vec: Vec<T> = self.collect();
//         vec.shuffle(&mut thread_rng());
//
//         vec
//     }
// }

macro_rules! shuffle_impl {
    ($($t:ty)+) => ($(
        impl Shuffle<$t> for Range<$t> {
            fn shuffle(&mut self) -> Vec<$t> {
                let mut vec: Vec<$t> = self.collect();
                vec.shuffle(&mut thread_rng());

                vec
            }
        }
    )+)
}

shuffle_impl! { i8 u8 i16 u16 i32 u32 i64 u64 i128 u128 isize usize }


#[cfg(test)]
mod tests {
    use crate::shuffle::Shuffle;

    #[test]
    fn should_shuffle_the_range() {
        let original: Vec<u32> = (0..10000).collect();
        let shuffled: Vec<u32> = (0..10000).shuffle();

        assert_ne!(original, shuffled);
    }
}
