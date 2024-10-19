mod list;
mod iter;

pub use list::FixedSizeLinkedListWithoutOption;
pub use iter::FixedSizeLinkedListWithoutOptionIter;

#[cfg(test)]
mod tests {
    use crate::double_ended_list::traits::tests_utils;
    use crate::FixedSizeLinkedList;

    #[test]
    fn all() {
        let capacity = 5;

        tests_utils::all(capacity, FixedSizeLinkedList::<isize>::with_capacity(capacity))
    }

    #[test]
    fn random() {
        let capacity = 13;

        tests_utils::random(capacity, FixedSizeLinkedList::<isize>::with_capacity(capacity), |_: &FixedSizeLinkedList<isize>| {})
    }
}
