use super::FixedSizeLinkedListWithoutOptionIter;
use crate::DoubleEndedList;
use std::fmt::{Debug, Formatter};
use std::mem;

pub struct FixedSizeLinkedListWithoutOption<T: Copy> {
    pub(super) length: usize,
    pub(super) front_index: usize,
    pub(super) back_index: usize,
    pub(super) data: Vec<T>,
}

impl<T: Copy> FixedSizeLinkedListWithoutOption<T> {
    pub fn with_capacity_and_value(capacity: usize, value: T) -> Self {
        Self {
            length: 0,
            front_index: 0,
            back_index: capacity - 1,
            data: vec![value; capacity],
        }
    }
}

impl<T: Default + Copy> FixedSizeLinkedListWithoutOption<T> {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            length: 0,
            front_index: 0,
            back_index: capacity - 1,
            data: vec![T::default(); capacity],
        }
    }
}

impl<T: Copy> DoubleEndedList<T> for FixedSizeLinkedListWithoutOption<T> {
    type Iter<'a> = FixedSizeLinkedListWithoutOptionIter<'a, T> where T: 'a;

    #[inline]
    #[must_use]
    fn capacity(&self) -> usize {
        self.data.capacity()
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
        self.length == self.data.capacity()
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
    #[inline]
    fn start_over(&mut self) {
        self.front_index = 0;
        self.back_index = self.data.capacity() - 1;
        self.length = 0;
    }

    /// Removes all elements from the `FixedSizeLinkedList`.
    ///
    /// This operation should compute in *O*(*n*) time.
    ///
    #[inline]
    fn clear(&mut self) where T: Clone {
        self.front_index = 0;
        self.back_index = self.data.capacity() - 1;
        self.length = 0;
    }

    /// Provides a reference to the front element, or `None` if the list is
    /// empty.
    ///
    /// This operation should compute in *O*(1) time.
    ///
    #[inline]
    #[must_use]
    fn front(&self) -> Option<&T> {
        if self.is_empty() {
            None
        } else {
            Some(&self.data[self.front_index])
        }
    }

    /// Provides a mutable reference to the front element, or `None` if the list
    /// is empty.
    ///
    /// This operation should compute in *O*(1) time.
    ///
    #[inline]
    #[must_use]
    fn front_mut(&mut self) -> Option<&mut T> {
        if self.is_empty() {
            None
        } else {
            Some(&mut self.data[self.front_index])
        }
    }

    /// Provides a reference to the back element, or `None` if the list is
    /// empty.
    ///
    /// This operation should compute in *O*(1) time.
    #[inline]
    #[must_use]
    fn back(&self) -> Option<&T> {
        if self.is_empty() {
            None
        } else {
            Some(&self.data[self.back_index])
        }
    }

    /// Provides a mutable reference to the back element, or `None` if the list
    /// is empty.
    ///
    /// This operation should compute in *O*(1) time.
    #[inline]
    fn back_mut(&mut self) -> Option<&mut T> {
        if self.is_empty() {
            None
        } else {
            Some(&mut self.data[self.back_index])
        }
    }

    /// Adds an element first in the list. return true if was successful
    ///
    /// This operation should compute in *O*(1) time.
    fn push_front(&mut self, item: T) -> bool {
        if self.is_full() {
            false
        } else {
            // Going 1 index back and rotate if reached 0
            let prev_index = (self.front_index + self.data.capacity() - 1) % self.data.capacity();

            self.data[prev_index] = item;

            self.front_index = prev_index;
            self.length += 1;

            true
        }
    }

    /// Removes the first element and returns it, or `None` if the list is
    /// empty.
    ///
    /// This operation should compute in *O*(1) time.
    fn pop_front(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            let next_index = self.front_index;
            self.front_index = (self.front_index + 1) % self.data.capacity();

            self.length -= 1;

            Some(self.data[next_index])
        }
    }

    /// Appends an element to the back of a list, return true if was successful
    ///
    /// This operation should compute in *O*(1) time.
    fn push_back(&mut self, item: T) -> bool {
        if self.is_full() {
            false
        } else {
            let next_index = (self.back_index + 1) % self.data.capacity();

            self.data[next_index] = item;

            self.back_index = next_index;
            self.length += 1;

            true
        }
    }

    /// Appends an element to the back of a list. and if the list is full remove the first item and return it
    ///
    /// This operation should compute in *O*(1) time.
    fn push_back_rotate(&mut self, item: T) -> Option<T> {
        if self.is_full() {
            let front = mem::replace(&mut self.data[self.front_index], item);

            self.back_index = self.front_index;
            self.front_index = (self.front_index + 1) % self.data.capacity();

            Some(front)
        } else {
            // No need for rotate, same as push_back
            let next_index = (self.back_index + 1) % self.data.capacity();

            self.data[next_index] = item;

            self.back_index = next_index;
            self.length += 1;


            None
        }
    }

    /// Removes the last element from a list and returns it, or `None` if
    /// it is empty.
    ///
    /// This operation should compute in *O*(1) time.
    fn pop_back(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            let prev_index = self.back_index;
            self.back_index = (self.back_index + self.data.capacity() - 1) % self.data.capacity();

            self.length -= 1;
            Some(self.data[prev_index])
        }
    }

    fn iter<'a>(&'a self) -> FixedSizeLinkedListWithoutOptionIter<'a, T> {
        FixedSizeLinkedListWithoutOptionIter::<'a, T>::new(&self)
    }
}


impl<T: Copy> Clone for FixedSizeLinkedListWithoutOption<T> {
    fn clone(&self) -> Self {
        Self {
            length: self.length,
            front_index: self.front_index,
            back_index: self.back_index,
            data: self.data.clone(),
        }
    }
}

// Implementing send if the item is implementing send
unsafe impl<T: Send + Copy> Send for FixedSizeLinkedListWithoutOption<T> {
}

impl<T: Debug + Copy> Debug for FixedSizeLinkedListWithoutOption<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}
