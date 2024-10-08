use super::FixedSizeLinkedListWithoutOption;

pub struct FixedSizeLinkedListWithoutOptionIter<'a, T: 'a + Copy> {
    list: &'a FixedSizeLinkedListWithoutOption<T>,
    length_passed: usize,
}

impl<'a, T: 'a + Copy> FixedSizeLinkedListWithoutOptionIter<'a, T> {
    pub fn new(list: &'a FixedSizeLinkedListWithoutOption<T>) -> FixedSizeLinkedListWithoutOptionIter<'a, T> {
        Self {
            length_passed: 0,
            list,
        }
    }
}

impl<'a, T: 'a + Copy> Iterator for FixedSizeLinkedListWithoutOptionIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        // If the list is empty or we reached the end
        if self.length_passed >= self.list.length {
            return None;
        }

        let index = (self.list.front_index + self.length_passed) % self.list.capacity;

        self.length_passed += 1;

        Some(&self.list.data[index])
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.list.length - self.length_passed;

        (len, Some(len))
    }
}
