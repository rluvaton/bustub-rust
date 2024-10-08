mod iter;
mod list;

pub use list::FixedSizeLinkedListSlice;
pub use iter::FixedSizeLinkedListSliceIter;


#[cfg(test)]
mod tests {
    use crate::double_ended_list::traits::tests_utils;
    use crate::FixedSizeLinkedListSlice;

    #[test]
    fn all() {
        let capacity = 5;

        let mut underlying = [None; 100];

        tests_utils::all(capacity, FixedSizeLinkedListSlice::<isize>::new(&mut underlying[5..10]))
    }

    #[test]
    fn random() {
        const CAPACITY: usize = 13;

        let mut underlying = [None; 100];

        let (before, after) = underlying.as_mut_slice().split_at_mut(11);
        let (part, after) = after.as_mut().split_at_mut(CAPACITY);

        tests_utils::random(CAPACITY, FixedSizeLinkedListSlice::<isize>::new(part), |list: &FixedSizeLinkedListSlice<isize>| {
            assert_eq!(before, [None; 11]);
            assert_eq!(after, [None; 100 - 11 - CAPACITY]);
        });

    }
}
