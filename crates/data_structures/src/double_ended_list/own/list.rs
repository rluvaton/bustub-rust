use std::fmt::{Debug, Formatter};
use crate::{DoubleEndedList, FixedSizeLinkedListIter};

pub struct FixedSizeLinkedList<T> {
    pub(super) capacity: usize,
    pub(super) length: usize,
    pub(super) front_index: usize,
    pub(super) back_index: usize,
    pub(super) data: Vec<Option<T>>,
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
}

impl<T> DoubleEndedList<T> for FixedSizeLinkedList<T> {
    type Iter<'a> = FixedSizeLinkedListIter<'a, T> where T: 'a;

    #[inline]
    #[must_use]
    fn capacity(&self) -> usize {
        self.capacity
    }

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
    fn is_empty(&self) -> bool {
        self.length == 0
    }

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
    fn is_full(&self) -> bool {
        self.length == self.capacity
    }

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
    fn len(&self) -> usize {
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
    fn start_over(&mut self) {
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
    fn clear(&mut self) where T: Clone {
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
    fn front(&self) -> Option<&T> {
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
    fn front_mut(&mut self) -> Option<&mut T> {
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
    fn back(&self) -> Option<&T> {
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
    fn back_mut(&mut self) -> Option<&mut T> {
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
    fn push_front(&mut self, item: T) -> bool {
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
    fn pop_front(&mut self) -> Option<T> {
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
    /// use data_structures::{DoubleEndedList, FixedSizeLinkedList};
    ///
    /// let mut d = FixedSizeLinkedList::with_capacity(10);
    /// d.push_back(1);
    /// d.push_back(3);
    /// assert_eq!(3, *d.back().unwrap());
    /// ```
    fn push_back(&mut self, item: T) -> bool {
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
    /// use data_structures::{DoubleEndedList, FixedSizeLinkedList};
    ///
    /// let mut d = FixedSizeLinkedList::with_capacity(10);
    /// d.push_back(1);
    /// d.push_back(3);
    /// assert_eq!(3, *d.back().unwrap());
    /// ```
    fn push_back_rotate(&mut self, item: T) -> Option<T> {
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
    /// use data_structures::{DoubleEndedList, FixedSizeLinkedList};
    ///
    /// let mut d = FixedSizeLinkedList::with_capacity(10);
    /// assert_eq!(d.pop_back(), None);
    /// d.push_back(1);
    /// d.push_back(3);
    /// assert_eq!(d.pop_back(), Some(3));
    /// ```
    fn pop_back(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            let prev_index = self.back_index;
            self.back_index = (self.back_index + self.capacity - 1) % self.capacity;

            self.length -= 1;
            self.data[prev_index].take()
        }
    }

    fn iter<'a>(&'a self) -> FixedSizeLinkedListIter<'a, T> {
        FixedSizeLinkedListIter::<'a, T>::new(&self)
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
        f.debug_list().entries(self.iter()).finish()
    }
}
