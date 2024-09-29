use std::ops::Range;
use rand::seq::SliceRandom;
use rand::thread_rng;


/// Shuffle, implemented for range for easier testing
pub trait Shuffle<T> {
    fn shuffle(&mut self) -> Vec<T>;

    fn shuffle_with_seed(&mut self, rng: &mut (impl rand::SeedableRng + rand::RngCore)) -> Vec<T>;
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

            fn shuffle_with_seed(&mut self, rng: &mut (impl rand::SeedableRng + rand::RngCore)) -> Vec<$t> {
                let mut vec: Vec<$t> = self.collect();
                vec.shuffle(rng);

                vec
            }
        }
    )+)
}

shuffle_impl! { i8 u8 i16 u16 i32 u32 i64 u64 i128 u128 isize usize }


#[cfg(test)]
mod tests {
    use rand::SeedableRng;
    use rand_chacha::ChaChaRng;
    use crate::shuffle::Shuffle;

    #[test]
    fn should_shuffle_the_range() {
        let original: Vec<u32> = (0..10000).collect();
        let shuffled: Vec<u32> = (0..10000).shuffle();

        assert_ne!(original, shuffled);
    }

    #[test]
    fn should_shuffle_with_seed_the_range() {

        let mut rng = ChaChaRng::from_seed([0; 32]);

        let original: Vec<u32> = (0..10000).collect();

        // Shuffling with same seed and cloning should output the same result
        let shuffled_with_same_seed1: Vec<u32> = (0..10000).shuffle_with_seed(&mut rng.clone());
        let shuffled_with_same_seed2: Vec<u32> = (0..10000).shuffle_with_seed(&mut rng.clone());

        assert_ne!(original, shuffled_with_same_seed1);
        assert_eq!(shuffled_with_same_seed1, shuffled_with_same_seed2);
    }

    #[test]
    fn should_shuffle_again_will_produce_different_output() {
        let mut rng = ChaChaRng::from_seed([0; 32]);

        let original: Vec<u32> = (0..10000).collect();

        let shuffled_with_same_seed1: Vec<u32> = (0..10000).shuffle_with_seed(&mut rng);
        let shuffled_with_same_seed2: Vec<u32> = (0..10000).shuffle_with_seed(&mut rng);

        assert_ne!(shuffled_with_same_seed1, shuffled_with_same_seed2);
    }

    #[test]
    fn should_shuffle_with_different_seed_produce_different_output() {
        let original: Vec<u32> = (0..10000).collect();

        let shuffled_with_same_seed1: Vec<u32> = (0..10000).shuffle_with_seed(&mut ChaChaRng::from_seed([1; 32]));
        let shuffled_with_same_seed2: Vec<u32> = (0..10000).shuffle_with_seed(&mut ChaChaRng::from_seed([2; 32]));

        assert_ne!(shuffled_with_same_seed1, shuffled_with_same_seed2);
    }
}
