use std::fmt::{Debug, Formatter};

pub struct FixedSizeLinkedList<T> {
    capacity: usize,
    length: usize,
    front_index: usize,
    back_index: usize,
    data: Vec<Option<T>>,
}

impl<T> FixedSizeLinkedList<T> {
    pub fn with_capacity(capacity: usize) -> Self {
        let mut data = Vec::with_capacity(capacity);
        for i in 0..capacity {
            data.insert(i, None);
        }

        Self {
            length: 0,
            capacity,
            front_index: 0,
            back_index: capacity - 1,
            data,
        }
    }

    #[inline]
    #[must_use]
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// Returns `true` if the `FixedSizeLinkedList` is empty.
    ///
    /// This operation should compute in *O*(1) time.
    ///
    /// # Examples
    ///
    /// ```
    /// use data_structures::FixedSizeLinkedList;
    ///
    /// let mut dl = FixedSizeLinkedList::with_capacity(10);
    /// assert!(dl.is_empty());
    ///
    /// dl.push_front("foo");
    /// assert!(!dl.is_empty());
    /// ```
    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    /// Returns `true` if the `FixedSizeLinkedList` is empty.
    ///
    /// This operation should compute in *O*(1) time.
    ///
    /// # Examples
    ///
    /// ```
    /// use data_structures::FixedSizeLinkedList;
    ///
    /// let mut dl = FixedSizeLinkedList::with_capacity(10);
    /// assert!(dl.is_empty());
    ///
    /// dl.push_front("foo");
    /// assert!(!dl.is_empty());
    /// ```
    #[inline]
    #[must_use]
    pub fn is_full(&self) -> bool {
        self.length == self.capacity
    }

    /// Returns the length of the `FixedSizeLinkedList`.
    ///
    /// This operation should compute in *O*(1) time.
    ///
    /// # Examples
    ///
    /// ```
    /// use data_structures::FixedSizeLinkedList;
    ///
    /// let mut dl = FixedSizeLinkedList::with_capacity(10);
    ///
    /// dl.push_front(2);
    /// assert_eq!(dl.len(), 1);
    ///
    /// dl.push_front(1);
    /// assert_eq!(dl.len(), 2);
    ///
    /// dl.push_back(3);
    /// assert_eq!(dl.len(), 3);
    /// ```
    #[inline]
    #[must_use]
    pub fn len(&self) -> usize {
        self.length
    }

    /// Keep all elements in `FixedSizeLinkedList` but start over (don't drop values but the values won't be found)
    ///
    /// this is useful for reusing memory with items that do not have custom drop logic
    ///
    /// This operation should compute in *O*(*1*) time.
    ///
    /// # Examples
    ///
    /// ```
    /// use data_structures::FixedSizeLinkedList;
    ///
    /// let mut dl = FixedSizeLinkedList::with_capacity(10);
    ///
    /// dl.push_front(2);
    /// dl.push_front(1);
    /// assert_eq!(dl.len(), 2);
    /// assert_eq!(dl.front(), Some(&1));
    ///
    /// dl.start_over();
    /// assert_eq!(dl.len(), 0);
    /// assert_eq!(dl.front(), None);
    /// ```
    #[inline]
    pub fn start_over(&mut self) {
        // If already empty nothing to do
        if self.is_empty() {
            return;
        }

        self.front_index = 0;
        self.back_index = self.capacity - 1;
        self.length = 0;
    }

    /// Removes all elements from the `FixedSizeLinkedList`.
    ///
    /// This operation should compute in *O*(*n*) time.
    ///
    /// # Examples
    ///
    /// ```
    /// use data_structures::FixedSizeLinkedList;
    ///
    /// let mut dl = FixedSizeLinkedList::with_capacity(10);
    ///
    /// dl.push_front(2);
    /// dl.push_front(1);
    /// assert_eq!(dl.len(), 2);
    /// assert_eq!(dl.front(), Some(&1));
    ///
    /// dl.clear();
    /// assert_eq!(dl.len(), 0);
    /// assert_eq!(dl.front(), None);
    /// ```
    #[inline]
    pub fn clear(&mut self) where T: Clone {
        // If already empty nothing to do
        if self.is_empty() {
            return;
        }

        if self.front_index <= self.back_index {
            self.data[self.front_index..=self.back_index].fill(None);
        } else {
            // If back index is before front (we had a circle)
            self.data[0..=self.back_index].fill(None);
            self.data[self.front_index..].fill(None);
        }

        self.front_index = 0;
        self.back_index = self.capacity - 1;
        self.length = 0;
    }

    /// Provides a reference to the front element, or `None` if the list is
    /// empty.
    ///
    /// This operation should compute in *O*(1) time.
    ///
    /// # Examples
    ///
    /// ```
    /// use data_structures::FixedSizeLinkedList;
    ///
    /// let mut dl = FixedSizeLinkedList::with_capacity(10);
    /// assert_eq!(dl.front(), None);
    ///
    /// dl.push_front(1);
    /// assert_eq!(dl.front(), Some(&1));
    /// ```
    #[inline]
    #[must_use]
    pub fn front(&self) -> Option<&T> {
        if self.is_empty() {
            None
        } else {
            self.data[self.front_index].as_ref()
        }
    }

    /// Provides a mutable reference to the front element, or `None` if the list
    /// is empty.
    ///
    /// This operation should compute in *O*(1) time.
    ///
    /// # Examples
    ///
    /// ```
    /// use data_structures::FixedSizeLinkedList;
    ///
    /// let mut dl = FixedSizeLinkedList::with_capacity(10);
    /// assert_eq!(dl.front(), None);
    ///
    /// dl.push_front(1);
    /// assert_eq!(dl.front(), Some(&1));
    ///
    /// match dl.front_mut() {
    ///     None => {},
    ///     Some(x) => *x = 5,
    /// }
    /// assert_eq!(dl.front(), Some(&5));
    /// ```
    #[inline]
    #[must_use]
    pub fn front_mut(&mut self) -> Option<&mut T> {
        if self.is_empty() {
            None
        } else {
            self.data[self.front_index].as_mut()
        }
    }

    /// Provides a reference to the back element, or `None` if the list is
    /// empty.
    ///
    /// This operation should compute in *O*(1) time.
    ///
    /// # Examples
    ///
    /// ```
    /// use data_structures::FixedSizeLinkedList;
    ///
    /// let mut dl = FixedSizeLinkedList::with_capacity(10);
    /// assert_eq!(dl.back(), None);
    ///
    /// dl.push_back(1);
    /// assert_eq!(dl.back(), Some(&1));
    /// ```
    #[inline]
    #[must_use]
    pub fn back(&self) -> Option<&T> {
        if self.is_empty() {
            None
        } else {
            self.data[self.back_index].as_ref()
        }
    }

    /// Provides a mutable reference to the back element, or `None` if the list
    /// is empty.
    ///
    /// This operation should compute in *O*(1) time.
    ///
    /// # Examples
    ///
    /// ```
    /// use data_structures::FixedSizeLinkedList;
    ///
    /// let mut dl = FixedSizeLinkedList::with_capacity(10);
    /// assert_eq!(dl.back(), None);
    ///
    /// dl.push_back(1);
    /// assert_eq!(dl.back(), Some(&1));
    ///
    /// match dl.back_mut() {
    ///     None => {},
    ///     Some(x) => *x = 5,
    /// }
    /// assert_eq!(dl.back(), Some(&5));
    /// ```
    #[inline]
    pub fn back_mut(&mut self) -> Option<&mut T> {
        if self.is_empty() {
            None
        } else {
            self.data[self.back_index].as_mut()
        }
    }

    /// Adds an element first in the list. return true if was successful
    ///
    /// This operation should compute in *O*(1) time.
    ///
    /// # Examples
    ///
    /// ```
    /// use data_structures::FixedSizeLinkedList;
    ///
    /// let mut dl = FixedSizeLinkedList::with_capacity(10);
    ///
    /// dl.push_front(2);
    /// assert_eq!(dl.front().unwrap(), &2);
    ///
    /// dl.push_front(1);
    /// assert_eq!(dl.front().unwrap(), &1);
    /// ```
    pub fn push_front(&mut self, item: T) -> bool {
        if self.is_full() {
            false
        } else {
            // Going 1 index back and rotate if reached 0
            let prev_index = (self.front_index + self.capacity - 1) % self.capacity;

            self.data[prev_index].replace(item);

            self.front_index = prev_index;
            self.length += 1;

            true
        }
    }

    /// Removes the first element and returns it, or `None` if the list is
    /// empty.
    ///
    /// This operation should compute in *O*(1) time.
    ///
    /// # Examples
    ///
    /// ```
    /// use data_structures::FixedSizeLinkedList;
    ///
    /// let mut d = FixedSizeLinkedList::with_capacity(10);
    /// assert_eq!(d.pop_front(), None);
    ///
    /// d.push_front(1);
    /// d.push_front(3);
    /// assert_eq!(d.pop_front(), Some(3));
    /// assert_eq!(d.pop_front(), Some(1));
    /// assert_eq!(d.pop_front(), None);
    /// ```
    pub fn pop_front(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            let next_index = self.front_index;
            self.front_index = (self.front_index + 1) % self.capacity;

            self.length -= 1;
            self.data[next_index].take()
        }
    }

    /// Appends an element to the back of a list, return true if was successful
    ///
    /// This operation should compute in *O*(1) time.
    ///
    /// # Examples
    ///
    /// ```
    /// use data_structures::FixedSizeLinkedList;
    ///
    /// let mut d = FixedSizeLinkedList::with_capacity(10);
    /// d.push_back(1);
    /// d.push_back(3);
    /// assert_eq!(3, *d.back().unwrap());
    /// ```
    pub fn push_back(&mut self, item: T) -> bool {
        if self.is_full() {
            false
        } else {
            let next_index = (self.back_index + 1) % self.capacity;

            self.data[next_index].replace(item);

            self.back_index = next_index;
            self.length += 1;

            true
        }
    }

    /// Appends an element to the back of a list. and if the list is full remove the first item and return it
    ///
    /// This operation should compute in *O*(1) time.
    ///
    /// # Examples
    ///
    /// ```
    /// use data_structures::FixedSizeLinkedList;
    ///
    /// let mut d = FixedSizeLinkedList::with_capacity(10);
    /// d.push_back(1);
    /// d.push_back(3);
    /// assert_eq!(3, *d.back().unwrap());
    /// ```
    pub fn push_back_rotate(&mut self, item: T) -> Option<T> {
        if self.is_full() {
            let front = self.data[self.front_index].replace(item);

            self.back_index = self.front_index;
            self.front_index = (self.front_index + 1) % self.capacity;

            front
        } else {
            // No need for rotate, same as push_back
            let next_index = (self.back_index + 1) % self.capacity;

            self.data[next_index] = Some(item);

            self.back_index = next_index;
            self.length += 1;


            None
        }
    }

    /// Removes the last element from a list and returns it, or `None` if
    /// it is empty.
    ///
    /// This operation should compute in *O*(1) time.
    ///
    /// # Examples
    ///
    /// ```
    /// use data_structures::FixedSizeLinkedList;
    ///
    /// let mut d = FixedSizeLinkedList::with_capacity(10);
    /// assert_eq!(d.pop_back(), None);
    /// d.push_back(1);
    /// d.push_back(3);
    /// assert_eq!(d.pop_back(), Some(3));
    /// ```
    pub fn pop_back(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            let prev_index = self.back_index;
            self.back_index = (self.back_index + self.capacity - 1) % self.capacity;

            self.length -= 1;
            self.data[prev_index].take()
        }
    }
}

impl<T: Clone> Clone for FixedSizeLinkedList<T> {
    fn clone(&self) -> Self {
        Self {
            length: self.length,
            capacity: self.capacity,
            front_index: self.front_index,
            back_index: self.back_index,
            data: self.data.clone(),
        }
    }
}

// Implementing send if the item is implementing send
unsafe impl<T: Send> Send for FixedSizeLinkedList<T> {
}

impl<T: Debug> Debug for FixedSizeLinkedList<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // TODO - fix this
        write!(f, "{:?}", self.data)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::{HashMap, LinkedList};
    use std::time::Duration;
    use rand::{thread_rng, Rng, SeedableRng};
    use rand::distributions::{Distribution, Standard};
    use crate::FixedSizeLinkedList;
    use rand_chacha::ChaChaRng;


    // All list operation
    #[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, PartialOrd, Ord)]
    enum Operation {
        PushFront,
        PushBack,
        PushRotate,
        PopFront,
        PopBack,
        PushFrontUntilFull,
        PushBackUntilFull,
        PushRotateUntilFullCircle,
        PopFrontUntilEmpty,
        PopBackUntilEmpty,
        StartOver,
        Clear,
    }

    impl Distribution<Operation> for Standard {
        fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Operation {
            match rng.gen_range(0..=26) {
                00 | 01 | 02 => Operation::PushFront,
                03 | 04 | 05 => Operation::PushBack,
                06 | 07 | 08 => Operation::PushRotate,
                09 | 10 | 11 => Operation::PopFront,
                12 | 13 | 14 => Operation::PopBack,
                15 | 16 => Operation::PushFrontUntilFull,
                17 | 18 => Operation::PushBackUntilFull,
                19 | 20 => Operation::PushRotateUntilFullCircle,
                21 | 22 => Operation::PopFrontUntilEmpty,
                23 | 24 => Operation::PopBackUntilEmpty,
                25 => Operation::StartOver,
                26 => Operation::Clear,
                _ => unreachable!()
            }
        }
    }

    fn rotate_in_actual_linked_list<T>(list: &mut LinkedList<T>, item: T, expected_length: usize) -> Option<T> {
        assert!(list.len() <= expected_length, "List cannot have more than expected length");

        if list.len() < expected_length {
            list.push_back(item);

            return None;
        }

        let front = list.pop_front();
        list.push_back(item);

        front
    }

    #[test]
    fn all() {
        const CAPACITY: usize = 5;

        let mut rng = rand::thread_rng();

        let mut list = FixedSizeLinkedList::<i16>::with_capacity(5);
        let mut helper_list = LinkedList::<i16>::new();

        assert_eq!(list.is_full(), false, "List should not be full after init");
        assert_eq!(list.is_empty(), true, "List should be empty after init");
        assert_eq!(list.len(), 0, "List is empty after init");
        assert_eq!(list.capacity(), CAPACITY);

        for i in 0..CAPACITY {
            let item  = rng.gen();

            helper_list.push_back(item);
            assert_eq!(list.push_back(item), true, "should insert for iteration {}", i);

            assert_eq!(list.front(), helper_list.front(), "should be the same for iteration {}", i);
            assert_eq!(list.back(), helper_list.back(), "should be the same for iteration {}", i);
            assert_eq!(list.is_full(), helper_list.len() == CAPACITY, "should be the same for iteration {}", i);
            assert_eq!(list.is_empty(), helper_list.is_empty(), "should be the same for iteration {}", i);
            assert_eq!(list.len(), helper_list.len(), "should be the same for iteration {}", i);
            assert_eq!(list.capacity(), CAPACITY, "should be the same for iteration {}", i);
        }

        {
            let item = rng.gen();

            assert_eq!(list.push_back(item), false, "should not insert");

            assert_eq!(list.front(), helper_list.front());
            assert_eq!(list.back(), helper_list.back());
            assert_eq!(list.is_full(), true);
            assert_eq!(list.is_empty(), false);
            assert_eq!(list.len(), CAPACITY);
            assert_eq!(list.capacity(), CAPACITY);
        }

        {
            let item = rng.gen();

            assert_eq!(list.push_front(item), false, "should not insert");

            assert_eq!(list.front(), helper_list.front());
            assert_eq!(list.back(), helper_list.back());
            assert_eq!(list.is_full(), true);
            assert_eq!(list.is_empty(), false);
            assert_eq!(list.len(), CAPACITY);
            assert_eq!(list.capacity(), CAPACITY);
        }

        for _ in 0..CAPACITY {
            let front = helper_list.pop_front();
            assert_eq!(list.pop_front(), front, "should pop front");

            assert_eq!(list.front(), helper_list.front());
            assert_eq!(list.back(), helper_list.back());
            assert_eq!(list.is_full(), helper_list.len() == CAPACITY);
            assert_eq!(list.is_empty(), helper_list.is_empty());
            assert_eq!(list.len(), helper_list.len());
            assert_eq!(list.capacity(), CAPACITY);
        }
        {
            assert_eq!(list.pop_front(), None, "should not pop");

            assert_eq!(list.front(), None);
            assert_eq!(list.back(), None);
            assert_eq!(list.is_full(), false);
            assert_eq!(list.is_empty(), true);
            assert_eq!(list.len(), 0);
            assert_eq!(list.capacity(), CAPACITY);
        }

        {
            assert_eq!(list.pop_back(), None, "should not pop");

            assert_eq!(list.front(), None);
            assert_eq!(list.back(), None);
            assert_eq!(list.is_full(), false);
            assert_eq!(list.is_empty(), true);
            assert_eq!(list.len(), 0);
            assert_eq!(list.capacity(), CAPACITY);
        }

        for _ in 0..CAPACITY {
            let item = rng.gen();

            helper_list.push_front(item);
            assert_eq!(list.push_front(item), true, "should insert");

            assert_eq!(list.front(), helper_list.front());
            assert_eq!(list.back(), helper_list.back());
            assert_eq!(list.is_full(), helper_list.len() == CAPACITY);
            assert_eq!(list.is_empty(), helper_list.is_empty());
            assert_eq!(list.len(), helper_list.len());
            assert_eq!(list.capacity(), CAPACITY);
        }

        {
            let item = rng.gen();

            assert_eq!(list.push_back(item), false, "should not insert");

            assert_eq!(list.front(), helper_list.front());
            assert_eq!(list.back(), helper_list.back());
            assert_eq!(list.is_full(), true);
            assert_eq!(list.is_empty(), false);
            assert_eq!(list.len(), CAPACITY);
            assert_eq!(list.capacity(), CAPACITY);
        }

        {
            let item = rng.gen();

            assert_eq!(list.push_front(item), false, "should not insert");

            assert_eq!(list.front(), helper_list.front());
            assert_eq!(list.back(), helper_list.back());
            assert_eq!(list.is_full(), true);
            assert_eq!(list.is_empty(), false);
            assert_eq!(list.len(), CAPACITY);
            assert_eq!(list.capacity(), CAPACITY);
        }

        for _ in 0..CAPACITY {
            let back = helper_list.pop_back();
            assert_eq!(list.pop_back(), back, "should pop back");

            assert_eq!(list.front(), helper_list.front());
            assert_eq!(list.back(), helper_list.back());
            assert_eq!(list.is_full(), helper_list.len() == CAPACITY);
            assert_eq!(list.is_empty(), helper_list.is_empty());
            assert_eq!(list.len(), helper_list.len());
            assert_eq!(list.capacity(), CAPACITY);
        }

        {
            assert_eq!(list.pop_front(), None, "should not pop");

            assert_eq!(list.front(), None);
            assert_eq!(list.back(), None);
            assert_eq!(list.is_full(), false);
            assert_eq!(list.is_empty(), true);
            assert_eq!(list.len(), 0);
            assert_eq!(list.capacity(), CAPACITY);
        }
        {
            assert_eq!(list.pop_back(), None, "should not pop");

            assert_eq!(list.front(), None);
            assert_eq!(list.back(), None);
            assert_eq!(list.is_full(), false);
            assert_eq!(list.is_empty(), true);
            assert_eq!(list.len(), 0);
            assert_eq!(list.capacity(), CAPACITY);
        }


        for i in 0..CAPACITY {
            let item = rng.gen();

            if i % 2 == 0 {
                helper_list.push_back(item);
                assert_eq!(list.push_back(item), true, "should push back");
            } else {
                helper_list.push_front(item);
                assert_eq!(list.push_front(item), true, "should push front");
            }

            assert_eq!(list.front(), helper_list.front());
            assert_eq!(list.back(), helper_list.back());
            assert_eq!(list.is_full(), helper_list.len() == CAPACITY);
            assert_eq!(list.is_empty(), helper_list.is_empty());
            assert_eq!(list.len(), helper_list.len());
            assert_eq!(list.capacity(), CAPACITY);
        }

        for i in 0..CAPACITY {
            if i % 2 == 0 {
                let back = helper_list.pop_back();
                assert_eq!(list.pop_back(), back, "should pop back");
            } else {
                let front = helper_list.pop_front();
                assert_eq!(list.pop_front(), front, "should pop front");
            }

            assert_eq!(list.front(), helper_list.front());
            assert_eq!(list.back(), helper_list.back());
            assert_eq!(list.is_full(), helper_list.len() == CAPACITY);
            assert_eq!(list.is_empty(), helper_list.is_empty());
            assert_eq!(list.len(), helper_list.len());
            assert_eq!(list.capacity(), CAPACITY);
        }


        for i in 0..CAPACITY {
            let item = rng.gen();

            helper_list.push_back(item);
            assert_eq!(list.push_back_rotate(item), None, "should push back rotate when still left");

            assert_eq!(list.front(), helper_list.front());
            assert_eq!(list.back(), helper_list.back());
            assert_eq!(list.is_full(), helper_list.len() == CAPACITY);
            assert_eq!(list.is_empty(), helper_list.is_empty());
            assert_eq!(list.len(), helper_list.len());
            assert_eq!(list.capacity(), CAPACITY);
        }


        for _ in 0..(CAPACITY * 5) {
            let item = rng.gen();


            // Manual rotate in the helper list
            let front = rotate_in_actual_linked_list(&mut helper_list, item, CAPACITY);
            assert_eq!(list.push_back_rotate(item), front, "should push back and rotate");

            assert_eq!(list.front(), helper_list.front());
            assert_eq!(list.back(), helper_list.back());
            assert_eq!(list.is_full(), helper_list.len() == CAPACITY);
            assert_eq!(list.is_empty(), helper_list.is_empty());
            assert_eq!(list.len(), helper_list.len());
            assert_eq!(list.capacity(), CAPACITY);
        }
    }

    #[test]
    fn rotate() {
        const CAPACITY: usize = 5;

        let mut list = FixedSizeLinkedList::with_capacity(5);
        let mut helper_list = LinkedList::new();

        assert_eq!(list.is_full(), false, "List is not full after init");
        assert_eq!(list.is_empty(), true, "List is empty after init");
        assert_eq!(list.len(), 0, "List is empty after init");
        assert_eq!(list.capacity(), 5);

        for i in 0..CAPACITY {
            helper_list.push_back(i);
            assert_eq!(list.push_back(i), true, "should insert");

            assert_eq!(list.front(), helper_list.front());
            assert_eq!(list.back(), helper_list.back());
            assert_eq!(list.is_full(), helper_list.len() == CAPACITY);
            assert_eq!(list.is_empty(), helper_list.is_empty());
            assert_eq!(list.len(), helper_list.len());
            assert_eq!(list.capacity(), CAPACITY);
        }

        assert_eq!(list.front_index, 0);
        assert_eq!(list.back_index, CAPACITY - 1);

        {
            let item = 5;
            let front = rotate_in_actual_linked_list(&mut helper_list, item, CAPACITY);
            assert_eq!(list.push_back_rotate(item), front, "should insert");

            assert_eq!(list.front_index, 1);
            assert_eq!(list.back_index, 0);

            assert_eq!(list.front(), helper_list.front());
            assert_eq!(list.back(), helper_list.back());
            assert_eq!(list.is_full(), helper_list.len() == CAPACITY);
            assert_eq!(list.is_empty(), helper_list.is_empty());
            assert_eq!(list.len(), helper_list.len());
            assert_eq!(list.capacity(), CAPACITY);
        }

        {
            let item = 6;
            let front = rotate_in_actual_linked_list(&mut helper_list, item, CAPACITY);
            assert_eq!(list.push_back_rotate(item), front, "should insert");

            assert_eq!(list.front_index, 2);
            assert_eq!(list.back_index, 1);

            assert_eq!(list.front(), helper_list.front());
            assert_eq!(list.back(), helper_list.back());
            assert_eq!(list.is_full(), helper_list.len() == CAPACITY);
            assert_eq!(list.is_empty(), helper_list.is_empty());
            assert_eq!(list.len(), helper_list.len());
            assert_eq!(list.capacity(), CAPACITY);
        }

        {
            let item = 7;
            let front = rotate_in_actual_linked_list(&mut helper_list, item, CAPACITY);
            assert_eq!(list.push_back_rotate(item), front, "should insert");

            assert_eq!(list.front_index, 3);
            assert_eq!(list.back_index, 2);

            assert_eq!(list.front(), helper_list.front());
            assert_eq!(list.back(), helper_list.back());
            assert_eq!(list.is_full(), helper_list.len() == CAPACITY);
            assert_eq!(list.is_empty(), helper_list.is_empty());
            assert_eq!(list.len(), helper_list.len());
            assert_eq!(list.capacity(), CAPACITY);
        }

        {
            assert_eq!(list.pop_back(), helper_list.pop_back(), "should pop_back");

            assert_eq!(list.front_index, 3);
            assert_eq!(list.back_index, 1);

            assert_eq!(list.front(), helper_list.front());
            assert_eq!(list.back(), helper_list.back());
            assert_eq!(list.is_full(), helper_list.len() == CAPACITY);
            assert_eq!(list.is_empty(), helper_list.is_empty());
            assert_eq!(list.len(), helper_list.len());
            assert_eq!(list.capacity(), CAPACITY);
        }

        {
            assert_eq!(list.pop_back(), helper_list.pop_back(), "should pop_back");

            assert_eq!(list.front_index, 3);
            assert_eq!(list.back_index, 0);

            assert_eq!(list.front(), helper_list.front());
            assert_eq!(list.back(), helper_list.back());
            assert_eq!(list.is_full(), helper_list.len() == CAPACITY);
            assert_eq!(list.is_empty(), helper_list.is_empty());
            assert_eq!(list.len(), helper_list.len());
            assert_eq!(list.capacity(), CAPACITY);
        }

        {
            assert_eq!(list.pop_back(), helper_list.pop_back(), "should pop_back");

            assert_eq!(list.front_index, 3);
            assert_eq!(list.back_index, CAPACITY - 1);

            assert_eq!(list.front(), helper_list.front());
            assert_eq!(list.back(), helper_list.back());
            assert_eq!(list.is_full(), helper_list.len() == CAPACITY);
            assert_eq!(list.is_empty(), helper_list.is_empty());
            assert_eq!(list.len(), helper_list.len());
            assert_eq!(list.capacity(), CAPACITY);
        }

        {
            assert_eq!(list.pop_back(), helper_list.pop_back(), "should pop_back");

            assert_eq!(list.front_index, 3);
            assert_eq!(list.back_index, CAPACITY - 2);

            assert_eq!(list.front(), helper_list.front());
            assert_eq!(list.back(), helper_list.back());
            assert_eq!(list.is_full(), helper_list.len() == CAPACITY);
            assert_eq!(list.is_empty(), helper_list.is_empty());
            assert_eq!(list.len(), helper_list.len());
            assert_eq!(list.capacity(), CAPACITY);
        }
    }

    #[test]
    fn random() {
        let shuffle_seed: u64 = thread_rng().gen();
        println!("Seed used: {}", shuffle_seed);
        let mut rng = ChaChaRng::seed_from_u64(shuffle_seed);

        const CAPACITY: usize = 13;

        let mut list = FixedSizeLinkedList::<isize>::with_capacity(CAPACITY);
        let mut helper_list = LinkedList::<isize>::new();

        assert_eq!(list.is_full(), false, "List is not full after init");
        assert_eq!(list.is_empty(), true, "List is empty after init");
        assert_eq!(list.len(), 0, "List is empty after init");
        assert_eq!(list.capacity(), CAPACITY);

        for _ in 0..CAPACITY * 10000 {
            let op = rng.sample(Standard);

            match op {
                Operation::PushFront => {
                    let item = rng.gen();

                    let pushed = list.push_front(item);
                    assert_eq!(pushed, helper_list.len() != CAPACITY);
                    if pushed {
                        helper_list.push_front(item);
                    }
                }
                Operation::PushBack => {
                    let item = rng.gen();

                    let pushed = list.push_back(item);
                    assert_eq!(pushed, helper_list.len() != CAPACITY);
                    if pushed {
                        helper_list.push_back(item);
                    }
                }
                Operation::PushRotate => {
                    let item = rng.gen();

                    let front = rotate_in_actual_linked_list(&mut helper_list, item, CAPACITY);
                    assert_eq!(list.push_back_rotate(item), front);
                }
                Operation::PopFront => {
                    let front = helper_list.pop_front();
                    assert_eq!(list.pop_front(), front);
                }
                Operation::PopBack => {
                    let back = helper_list.pop_back();
                    assert_eq!(list.pop_back(), back);
                }
                Operation::PushFrontUntilFull => {
                    for _ in 0..CAPACITY {
                        if list.is_full() {
                            break;
                        }
                        let item = rng.gen();

                        helper_list.push_front(item);
                        assert_eq!(list.push_front(item), true);
                    }
                }
                Operation::PushBackUntilFull => {
                    for _ in 0..CAPACITY {
                        if list.is_full() {
                            break;
                        }
                        let item = rng.gen();

                        helper_list.push_back(item);
                        assert_eq!(list.push_back(item), true);
                    }
                }
                Operation::PushRotateUntilFullCircle => {
                    for _ in 0..CAPACITY {
                        let item = rng.gen();

                        let front = rotate_in_actual_linked_list(&mut helper_list, item, CAPACITY);
                        assert_eq!(list.push_back_rotate(item), front);
                    }
                }
                Operation::PopFrontUntilEmpty => {
                    for _ in 0..CAPACITY {
                        if list.is_empty() {
                            break;
                        }

                        let front = helper_list.pop_front();
                        assert_eq!(list.pop_front(), front);
                    }
                }
                Operation::PopBackUntilEmpty => {
                    for _ in 0..CAPACITY {
                        if list.is_empty() {
                            break;
                        }

                        let front = helper_list.pop_back();
                        assert_eq!(list.pop_back(), front);
                    }
                },

                Operation::Clear => {
                    list.clear();
                    helper_list.clear();
                }

                Operation::StartOver => {
                    list.start_over();
                    helper_list.clear();
                }
            }

            assert_eq!(list.front(), helper_list.front());
            assert_eq!(list.back(), helper_list.back());
            assert_eq!(list.is_full(), helper_list.len() == CAPACITY);
            assert_eq!(list.is_empty(), helper_list.is_empty());
            assert_eq!(list.len(), helper_list.len());
            assert_eq!(list.capacity(), CAPACITY);
        }
    }
}
