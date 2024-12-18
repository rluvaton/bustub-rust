use super::FixedSizeLinkedList;

pub struct FixedSizeLinkedListIter<'a, T: 'a> {
    list: &'a FixedSizeLinkedList<T>,
    length_passed: usize,
}

impl<'a, T: 'a> FixedSizeLinkedListIter<'a, T> {
    pub fn new(list: &'a FixedSizeLinkedList<T>) -> FixedSizeLinkedListIter<'a, T> {
        Self {
            length_passed: 0,
            list,
        }
    }
}

impl<'a, T: 'a> Iterator for FixedSizeLinkedListIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        // If the list is empty or we reached the end
        if self.length_passed >= self.list.length {
            return None;
        }

        let index = (self.list.front_index + self.length_passed) % self.list.capacity;

        self.length_passed += 1;

        self.list.data[index].as_ref()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.list.length - self.length_passed;

        (len, Some(len))
    }
}

impl<T> ExactSizeIterator for FixedSizeLinkedListIter<'_, T> {}
