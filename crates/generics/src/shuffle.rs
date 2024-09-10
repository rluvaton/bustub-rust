use std::ops::Range;
use rand::seq::SliceRandom;
use rand::thread_rng;

/// Shuffle, implemented for range for easier testing
pub trait Shuffle<T> {
    fn shuffle(&mut self) -> Vec<T>;
}

impl Shuffle<u8> for Range<u8> {
    fn shuffle(&mut self) -> Vec<u8> {
        let mut vec: Vec<u8> = self.collect();
        vec.shuffle(&mut thread_rng());

        vec
    }
}

impl Shuffle<i8> for Range<i8> {
    fn shuffle(&mut self) -> Vec<i8> {
        let mut vec: Vec<i8> = self.collect();
        vec.shuffle(&mut thread_rng());

        vec
    }
}

impl Shuffle<u16> for Range<u16> {
    fn shuffle(&mut self) -> Vec<u16> {
        let mut vec: Vec<u16> = self.collect();
        vec.shuffle(&mut thread_rng());

        vec
    }
}

impl Shuffle<i16> for Range<i16> {
    fn shuffle(&mut self) -> Vec<i16> {
        let mut vec: Vec<i16> = self.collect();
        vec.shuffle(&mut thread_rng());

        vec
    }
}

impl Shuffle<u32> for Range<u32> {
    fn shuffle(&mut self) -> Vec<u32> {
        let mut vec: Vec<u32> = self.collect();
        vec.shuffle(&mut thread_rng());

        vec
    }
}

impl Shuffle<i32> for Range<i32> {
    fn shuffle(&mut self) -> Vec<i32> {
        let mut vec: Vec<i32> = self.collect();
        vec.shuffle(&mut thread_rng());

        vec
    }
}

impl Shuffle<u64> for Range<u64> {
    fn shuffle(&mut self) -> Vec<u64> {
        let mut vec: Vec<u64> = self.collect();
        vec.shuffle(&mut thread_rng());

        vec
    }
}

impl Shuffle<i64> for Range<i64> {
    fn shuffle(&mut self) -> Vec<i64> {
        let mut vec: Vec<i64> = self.collect();
        vec.shuffle(&mut thread_rng());

        vec
    }
}

impl Shuffle<u128> for Range<u128> {
    fn shuffle(&mut self) -> Vec<u128> {
        let mut vec: Vec<u128> = self.collect();
        vec.shuffle(&mut thread_rng());

        vec
    }
}

impl Shuffle<i128> for Range<i128> {
    fn shuffle(&mut self) -> Vec<i128> {
        let mut vec: Vec<i128> = self.collect();
        vec.shuffle(&mut thread_rng());

        vec
    }
}

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
