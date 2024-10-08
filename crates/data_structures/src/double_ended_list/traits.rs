use std::fmt::Debug;


pub trait DoubleEndedList<T> {
    type Iter<'a>: Iterator<Item=&'a T>
    where
        Self: 'a,
        T: 'a;

    #[inline]
    #[must_use]
    fn capacity(&self) -> usize;

    /// Returns `true` if the `FixedSizeLinkedList` is empty.
    ///
    /// This operation should compute in *O*(1) time.
    ///
    /// # Examples
    ///
    /// ```
    /// use data_structures::{DoubleEndedList, FixedSizeLinkedList};
    ///
    /// let mut dl = FixedSizeLinkedList::with_capacity(10);
    /// assert!(dl.is_empty());
    ///
    /// dl.push_front("foo");
    /// assert!(!dl.is_empty());
    /// ```
    #[inline]
    #[must_use]
    fn is_empty(&self) -> bool;

    /// Returns `true` if the `FixedSizeLinkedList` is empty.
    ///
    /// This operation should compute in *O*(1) time.
    ///
    /// # Examples
    ///
    /// ```
    /// use data_structures::{DoubleEndedList, FixedSizeLinkedList};
    ///
    /// let mut dl = FixedSizeLinkedList::with_capacity(10);
    /// assert!(dl.is_empty());
    ///
    /// dl.push_front("foo");
    /// assert!(!dl.is_empty());
    /// ```
    #[inline]
    #[must_use]
    fn is_full(&self) -> bool;

    /// Returns the length of the `FixedSizeLinkedList`.
    ///
    /// This operation should compute in *O*(1) time.
    ///
    /// # Examples
    ///
    /// ```
    /// use data_structures::{DoubleEndedList, FixedSizeLinkedList};
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
    fn len(&self) -> usize;

    /// Keep all elements in `FixedSizeLinkedList` but start over (don't drop values but the values won't be found)
    ///
    /// this is useful for reusing memory with items that do not have custom drop logic
    ///
    /// This operation should compute in *O*(*1*) time.
    ///
    /// # Examples
    ///
    /// ```
    /// use data_structures::{DoubleEndedList, FixedSizeLinkedList};
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
    fn start_over(&mut self);

    /// Removes all elements from the `FixedSizeLinkedList`.
    ///
    /// This operation should compute in *O*(*n*) time.
    ///
    /// # Examples
    ///
    /// ```
    /// use data_structures::{DoubleEndedList, FixedSizeLinkedList};
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
    fn clear(&mut self)
    where
        T: Clone;

    /// Provides a reference to the front element, or `None` if the list is
    /// empty.
    ///
    /// This operation should compute in *O*(1) time.
    ///
    /// # Examples
    ///
    /// ```
    /// use data_structures::{DoubleEndedList, FixedSizeLinkedList};
    ///
    /// let mut dl = FixedSizeLinkedList::with_capacity(10);
    /// assert_eq!(dl.front(), None);
    ///
    /// dl.push_front(1);
    /// assert_eq!(dl.front(), Some(&1));
    /// ```
    #[inline]
    #[must_use]
    fn front(&self) -> Option<&T>;

    /// Provides a mutable reference to the front element, or `None` if the list
    /// is empty.
    ///
    /// This operation should compute in *O*(1) time.
    ///
    /// # Examples
    ///
    /// ```
    /// use data_structures::{DoubleEndedList, FixedSizeLinkedList};
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
    fn front_mut(&mut self) -> Option<&mut T>;

    /// Provides a reference to the back element, or `None` if the list is
    /// empty.
    ///
    /// This operation should compute in *O*(1) time.
    ///
    /// # Examples
    ///
    /// ```
    /// use data_structures::{DoubleEndedList, FixedSizeLinkedList};
    ///
    /// let mut dl = FixedSizeLinkedList::with_capacity(10);
    /// assert_eq!(dl.back(), None);
    ///
    /// dl.push_back(1);
    /// assert_eq!(dl.back(), Some(&1));
    /// ```
    #[inline]
    #[must_use]
    fn back(&self) -> Option<&T>;

    /// Provides a mutable reference to the back element, or `None` if the list
    /// is empty.
    ///
    /// This operation should compute in *O*(1) time.
    ///
    /// # Examples
    ///
    /// ```
    /// use data_structures::{DoubleEndedList, FixedSizeLinkedList};
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
    fn back_mut(&mut self) -> Option<&mut T>;

    /// Adds an element first in the list. return true if was successful
    ///
    /// This operation should compute in *O*(1) time.
    ///
    /// # Examples
    ///
    /// ```
    /// use data_structures::{DoubleEndedList, FixedSizeLinkedList};
    ///
    /// let mut dl = FixedSizeLinkedList::with_capacity(10);
    ///
    /// dl.push_front(2);
    /// assert_eq!(dl.front().unwrap(), &2);
    ///
    /// dl.push_front(1);
    /// assert_eq!(dl.front().unwrap(), &1);
    /// ```
    fn push_front(&mut self, item: T) -> bool;

    /// Removes the first element and returns it, or `None` if the list is
    /// empty.
    ///
    /// This operation should compute in *O*(1) time.
    ///
    /// # Examples
    ///
    /// ```
    /// use data_structures::{DoubleEndedList, FixedSizeLinkedList};
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
    fn pop_front(&mut self) -> Option<T>;

    /// Appends an element to the back of a list, return true if was successful
    ///
    /// This operation should compute in *O*(1) time.
    ///
    /// # Examples
    ///
    /// ```
    /// use data_structures::{DoubleEndedList, FixedSizeLinkedList};
    ///
    /// let mut d = FixedSizeLinkedList::with_capacity(10);
    /// d.push_back(1);
    /// d.push_back(3);
    /// assert_eq!(3, *d.back().unwrap());
    /// ```
    fn push_back(&mut self, item: T) -> bool;

    /// Appends an element to the back of a list. and if the list is full remove the first item and return it
    ///
    /// This operation should compute in *O*(1) time.
    ///
    /// # Examples
    ///
    /// ```
    /// use data_structures::{DoubleEndedList, FixedSizeLinkedList};
    ///
    /// let mut d = FixedSizeLinkedList::with_capacity(10);
    /// d.push_back(1);
    /// d.push_back(3);
    /// assert_eq!(3, *d.back().unwrap());
    /// ```
    fn push_back_rotate(&mut self, item: T) -> Option<T>;

    /// Removes the last element from a list and returns it, or `None` if
    /// it is empty.
    ///
    /// This operation should compute in *O*(1) time.
    ///
    /// # Examples
    ///
    /// ```
    /// use data_structures::{DoubleEndedList, FixedSizeLinkedList};
    ///
    /// let mut d = FixedSizeLinkedList::with_capacity(10);
    /// assert_eq!(d.pop_back(), None);
    /// d.push_back(1);
    /// d.push_back(3);
    /// assert_eq!(d.pop_back(), Some(3));
    /// ```
    fn pop_back(&mut self) -> Option<T>;

    fn iter(&self) -> Self::Iter<'_>;
}

#[cfg(test)]
pub(super) mod tests_utils {
    use crate::DoubleEndedList;
    use rand::distributions::{Distribution, Standard};
    use rand::{thread_rng, Rng, SeedableRng};
    use rand_chacha::ChaChaRng;
    use std::collections::LinkedList;
    use std::fmt::Debug;

    // All list operation
    #[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, PartialOrd, Ord)]
    pub(super) enum Operation {
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

    pub fn all<T: Copy + PartialEq + Debug>(capacity: usize, mut list: impl DoubleEndedList<T>)
    where
        Standard: Distribution<T>,
    {
        let mut rng = rand::thread_rng();

        let mut helper_list = LinkedList::<T>::new();

        assert_eq!(list.is_full(), false, "List should not be full after init");
        assert_eq!(list.is_empty(), true, "List should be empty after init");
        assert_eq!(list.len(), 0, "List is empty after init");
        assert_eq!(list.capacity(), capacity);

        for i in 0..capacity {
            let item = rng.gen();

            helper_list.push_back(item);
            assert_eq!(list.push_back(item), true, "should insert for iteration {}", i);

            assert_eq!(list.front(), helper_list.front(), "should be the same for iteration {}", i);
            assert_eq!(list.back(), helper_list.back(), "should be the same for iteration {}", i);
            assert_eq!(list.is_full(), helper_list.len() == capacity, "should be the same for iteration {}", i);
            assert_eq!(list.is_empty(), helper_list.is_empty(), "should be the same for iteration {}", i);
            assert_eq!(list.len(), helper_list.len(), "should be the same for iteration {}", i);
            assert_eq!(list.capacity(), capacity, "should be the same for iteration {}", i);
        }

        {
            let item = rng.gen();

            assert_eq!(list.push_back(item), false, "should not insert");

            assert_eq!(list.front(), helper_list.front());
            assert_eq!(list.back(), helper_list.back());
            assert_eq!(list.is_full(), true);
            assert_eq!(list.is_empty(), false);
            assert_eq!(list.len(), capacity);
            assert_eq!(list.capacity(), capacity);
        }

        {
            let item = rng.gen();

            assert_eq!(list.push_front(item), false, "should not insert");

            assert_eq!(list.front(), helper_list.front());
            assert_eq!(list.back(), helper_list.back());
            assert_eq!(list.is_full(), true);
            assert_eq!(list.is_empty(), false);
            assert_eq!(list.len(), capacity);
            assert_eq!(list.capacity(), capacity);
        }

        for _ in 0..capacity {
            let front = helper_list.pop_front();
            assert_eq!(list.pop_front(), front, "should pop front");

            assert_eq!(list.front(), helper_list.front());
            assert_eq!(list.back(), helper_list.back());
            assert_eq!(list.is_full(), helper_list.len() == capacity);
            assert_eq!(list.is_empty(), helper_list.is_empty());
            assert_eq!(list.len(), helper_list.len());
            assert_eq!(list.capacity(), capacity);
        }
        {
            assert_eq!(list.pop_front(), None, "should not pop");

            assert_eq!(list.front(), None);
            assert_eq!(list.back(), None);
            assert_eq!(list.is_full(), false);
            assert_eq!(list.is_empty(), true);
            assert_eq!(list.len(), 0);
            assert_eq!(list.capacity(), capacity);
        }

        {
            assert_eq!(list.pop_back(), None, "should not pop");

            assert_eq!(list.front(), None);
            assert_eq!(list.back(), None);
            assert_eq!(list.is_full(), false);
            assert_eq!(list.is_empty(), true);
            assert_eq!(list.len(), 0);
            assert_eq!(list.capacity(), capacity);
        }

        for _ in 0..capacity {
            let item = rng.gen();

            helper_list.push_front(item);
            assert_eq!(list.push_front(item), true, "should insert");

            assert_eq!(list.front(), helper_list.front());
            assert_eq!(list.back(), helper_list.back());
            assert_eq!(list.is_full(), helper_list.len() == capacity);
            assert_eq!(list.is_empty(), helper_list.is_empty());
            assert_eq!(list.len(), helper_list.len());
            assert_eq!(list.capacity(), capacity);
        }

        {
            let item = rng.gen();

            assert_eq!(list.push_back(item), false, "should not insert");

            assert_eq!(list.front(), helper_list.front());
            assert_eq!(list.back(), helper_list.back());
            assert_eq!(list.is_full(), true);
            assert_eq!(list.is_empty(), false);
            assert_eq!(list.len(), capacity);
            assert_eq!(list.capacity(), capacity);
        }

        {
            let item = rng.gen();

            assert_eq!(list.push_front(item), false, "should not insert");

            assert_eq!(list.front(), helper_list.front());
            assert_eq!(list.back(), helper_list.back());
            assert_eq!(list.is_full(), true);
            assert_eq!(list.is_empty(), false);
            assert_eq!(list.len(), capacity);
            assert_eq!(list.capacity(), capacity);
        }

        for _ in 0..capacity {
            let back = helper_list.pop_back();
            assert_eq!(list.pop_back(), back, "should pop back");

            assert_eq!(list.front(), helper_list.front());
            assert_eq!(list.back(), helper_list.back());
            assert_eq!(list.is_full(), helper_list.len() == capacity);
            assert_eq!(list.is_empty(), helper_list.is_empty());
            assert_eq!(list.len(), helper_list.len());
            assert_eq!(list.capacity(), capacity);
        }

        {
            assert_eq!(list.pop_front(), None, "should not pop");

            assert_eq!(list.front(), None);
            assert_eq!(list.back(), None);
            assert_eq!(list.is_full(), false);
            assert_eq!(list.is_empty(), true);
            assert_eq!(list.len(), 0);
            assert_eq!(list.capacity(), capacity);
        }
        {
            assert_eq!(list.pop_back(), None, "should not pop");

            assert_eq!(list.front(), None);
            assert_eq!(list.back(), None);
            assert_eq!(list.is_full(), false);
            assert_eq!(list.is_empty(), true);
            assert_eq!(list.len(), 0);
            assert_eq!(list.capacity(), capacity);
        }


        for i in 0..capacity {
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
            assert_eq!(list.is_full(), helper_list.len() == capacity);
            assert_eq!(list.is_empty(), helper_list.is_empty());
            assert_eq!(list.len(), helper_list.len());
            assert_eq!(list.capacity(), capacity);
        }

        for i in 0..capacity {
            if i % 2 == 0 {
                let back = helper_list.pop_back();
                assert_eq!(list.pop_back(), back, "should pop back");
            } else {
                let front = helper_list.pop_front();
                assert_eq!(list.pop_front(), front, "should pop front");
            }

            assert_eq!(list.front(), helper_list.front());
            assert_eq!(list.back(), helper_list.back());
            assert_eq!(list.is_full(), helper_list.len() == capacity);
            assert_eq!(list.is_empty(), helper_list.is_empty());
            assert_eq!(list.len(), helper_list.len());
            assert_eq!(list.capacity(), capacity);
        }


        for i in 0..capacity {
            let item = rng.gen();

            helper_list.push_back(item);
            assert_eq!(list.push_back_rotate(item), None, "should push back rotate when still left");

            assert_eq!(list.front(), helper_list.front());
            assert_eq!(list.back(), helper_list.back());
            assert_eq!(list.is_full(), helper_list.len() == capacity);
            assert_eq!(list.is_empty(), helper_list.is_empty());
            assert_eq!(list.len(), helper_list.len());
            assert_eq!(list.capacity(), capacity);
        }


        for _ in 0..(capacity * 5) {
            let item = rng.gen();


            // Manual rotate in the helper list
            let front = rotate_in_actual_linked_list(&mut helper_list, item, capacity);
            assert_eq!(list.push_back_rotate(item), front, "should push back and rotate");

            assert_eq!(list.front(), helper_list.front());
            assert_eq!(list.back(), helper_list.back());
            assert_eq!(list.is_full(), helper_list.len() == capacity);
            assert_eq!(list.is_empty(), helper_list.is_empty());
            assert_eq!(list.len(), helper_list.len());
            assert_eq!(list.capacity(), capacity);
        }
    }

    pub fn random<T: Copy + PartialEq + Debug, List: DoubleEndedList<T>, CustomValidationFn: Fn(&List)>(capacity: usize, mut list: List, custom_validation_after_each_op: CustomValidationFn)
    where
        Standard: Distribution<T>,
    {
        let shuffle_seed: u64 = thread_rng().gen::<u64>();
        println!("Seed used: {}", shuffle_seed);
        let mut rng = ChaChaRng::seed_from_u64(shuffle_seed);


        let mut helper_list = LinkedList::<T>::new();

        assert_eq!(list.is_full(), false, "List is not full after init");
        assert_eq!(list.is_empty(), true, "List is empty after init");
        assert_eq!(list.len(), 0, "List is empty after init");
        assert_eq!(list.capacity(), capacity);

        custom_validation_after_each_op(&list);

        for _ in 0..capacity * 10000 {
            let op = rng.sample::<Operation, _>(Standard);

            match op {
                Operation::PushFront => {
                    let item = rng.gen();

                    let pushed = list.push_front(item);
                    assert_eq!(pushed, helper_list.len() != capacity);
                    if pushed {
                        helper_list.push_front(item);
                    }
                }
                Operation::PushBack => {
                    let item = rng.gen();

                    let pushed = list.push_back(item);
                    assert_eq!(pushed, helper_list.len() != capacity);
                    if pushed {
                        helper_list.push_back(item);
                    }
                }
                Operation::PushRotate => {
                    let item = rng.gen();

                    let front = rotate_in_actual_linked_list(&mut helper_list, item, capacity);
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
                    for _ in 0..capacity {
                        if list.is_full() {
                            break;
                        }
                        let item = rng.gen();

                        helper_list.push_front(item);
                        assert_eq!(list.push_front(item), true);
                    }
                }
                Operation::PushBackUntilFull => {
                    for _ in 0..capacity {
                        if list.is_full() {
                            break;
                        }
                        let item = rng.gen();

                        helper_list.push_back(item);
                        assert_eq!(list.push_back(item), true);
                    }
                }
                Operation::PushRotateUntilFullCircle => {
                    for _ in 0..capacity {
                        let item = rng.gen();

                        let front = rotate_in_actual_linked_list(&mut helper_list, item, capacity);
                        assert_eq!(list.push_back_rotate(item), front);
                    }
                }
                Operation::PopFrontUntilEmpty => {
                    for _ in 0..capacity {
                        if list.is_empty() {
                            break;
                        }

                        let front = helper_list.pop_front();
                        assert_eq!(list.pop_front(), front);
                    }
                }
                Operation::PopBackUntilEmpty => {
                    for _ in 0..capacity {
                        if list.is_empty() {
                            break;
                        }

                        let front = helper_list.pop_back();
                        assert_eq!(list.pop_back(), front);
                    }
                }

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
            assert_eq!(list.is_full(), helper_list.len() == capacity);
            assert_eq!(list.is_empty(), helper_list.is_empty());
            assert_eq!(list.len(), helper_list.len());
            assert_eq!(list.capacity(), capacity);

            assert_eq!(list.iter().cloned().collect::<Vec<_>>(), helper_list.iter().cloned().collect::<Vec<T>>());

            custom_validation_after_each_op(&list);
        }
    }
}
