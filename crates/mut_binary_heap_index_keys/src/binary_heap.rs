#![deny(unsafe_op_in_unsafe_fn)]
// #![stable(feature = "rust1", since = "1.0.0")]

use std::cmp::{min, Ordering};
use std::collections::HashMap;
use std::hash::{BuildHasherDefault, Hash};
use std::marker::PhantomData;
// use std::iter::FusedIterator;
// use std::vec::Drain;
use compare::Compare;
use core::fmt;
use core::mem::{swap, ManuallyDrop};
use core::ptr;
use std::ops::Deref;
use std::ops::DerefMut;
use std::vec;
use identity_hash::{IdentityHashable, IdentityHasher};
// use super::SpecExtend;

/// A priority queue implemented with a binary heap storing key-value pairs.
///
/// Unlike the implementation of [BinaryHeap](std::collections::BinaryHeap) in the
/// standard library, it is ok to modify values while in the heap. This is possible
/// through the [`BinaryHeapIndexKeys::get_mut()`] method. Updating a value through `RefCell`
/// or global state, etc however will still result in an invalid heap as the heap
/// won't get updated automatically.
///
/// # Examples
///
/// ```
/// use mut_binary_heap_index_keys::BinaryHeapIndexKeys;
///
/// // Type inference lets us omit an explicit type signature (which
/// // would be `BinaryHeapIndexKeys<i32, i32, MaxComparator>` in this example).
/// let mut heap: BinaryHeapIndexKeys<_, _> = BinaryHeapIndexKeys::new();
///
/// // We can use peek to look at the next item in the heap. In this case,
/// // there's no items in there yet so we get None.
/// assert_eq!(heap.peek(), None);
///
/// // Let's add some scores...
/// heap.push(1, 1);
/// heap.push(2, 5);
/// heap.push(3, 2);
///
/// // Now peek shows the most important item in the heap.
/// assert_eq!(heap.peek(), Some(&5));
///
/// // We can check the length of a heap.
/// assert_eq!(heap.len(), 3);
///
/// // We can iterate over the items in the heap, although they are returned in
/// // a random order.
/// for x in &heap {
///     println!("key {}, value {}", x.0, x.1);
/// }
///
/// // If we instead pop these scores, they should come back in order.
/// assert_eq!(heap.pop(), Some(5));
/// assert_eq!(heap.pop(), Some(2));
/// assert_eq!(heap.pop(), Some(1));
/// assert_eq!(heap.pop(), None);
///
/// // We can clear the heap of any remaining items.
/// heap.clear();
///
/// // The heap should now be empty.
/// assert!(heap.is_empty())
/// ```
///
/// A `BinaryHeap` with a known list of items can be initialized from an iterator
/// and a key selection function
///
/// ```
/// use mut_binary_heap_index_keys::BinaryHeapIndexKeys;
///
/// // This will create a max-heap.
/// let heap: BinaryHeapIndexKeys<_, _> = BinaryHeapIndexKeys::from([1, 5, 2].iter(), |v| *v.clone());
/// ```
///
/// ## Min-heap
///
/// `BinaryHeap` can also act as a min-heap without requiring [`Reverse`] or a custom [`Ord`]
/// implementation.
///
/// ```
/// use mut_binary_heap_index_keys::BinaryHeapIndexKeys;
///
/// let mut heap = BinaryHeapIndexKeys::new_min();
///
/// // There is no need to wrap values in `Reverse`
/// heap.push(1, 1);
/// heap.push(2, 5);
/// heap.push(3, 2);
///
/// // If we pop these scores now, they should come back in the reverse order.
/// assert_eq!(heap.pop(), Some(1));
/// assert_eq!(heap.pop(), Some(2));
/// assert_eq!(heap.pop(), Some(5));
/// assert_eq!(heap.pop(), None);
/// ```
///
/// # Time complexity
///
/// | method             | cost           |
/// |--------------------|----------------|
/// | [push]             | *O*(1)~        |
/// | [pop]              | *O*(log(*n*))  |
/// | [peek]/[peek\_mut] | *O*(1)         |
/// | [get]              | *O*(1)         |
/// | [get\_mut]         | *O*(log(*n*))  |
/// | [contains\_key]     | *O*(1)         |
///
/// The value for `push` is an expected cost; the method documentation gives a
/// more detailed analysis.
/// The cost for `get_mut` contains the cost of dropping the `RefMut` returned
/// by the function. Getting access is *O*(1).
///
/// [`Reverse`]: https://doc.rust-lang.org/stable/core/cmp/struct.Reverse.html
/// [`Ord`]: https://doc.rust-lang.org/stable/core/cmp/trait.Ord.html
/// [`Cell`]: https://doc.rust-lang.org/stable/core/cell/struct.Cell.html
/// [`RefCell`]: https://doc.rust-lang.org/stable/core/cell/struct.RefCell.html
/// [push]: BinaryHeapIndexKeys::push
/// [pop]: BinaryHeapIndexKeys::pop
/// [peek]: BinaryHeapIndexKeys::peek
/// [peek\_mut]: BinaryHeapIndexKeys::peek_mut
/// [get]: BinaryHeapIndexKeys::get
/// [get\_mut]: BinaryHeapIndexKeys::get_mut
/// [contains\_key]: BinaryHeapIndexKeys::contains_key
// #[stable(feature = "rust1", since = "1.0.0")]
pub struct BinaryHeapIndexKeys<K: IdentityHashable, T, C = MaxComparator> {
    data: Vec<(K, T)>,
    cmp: C,
    keys: HashMap<K, usize, BuildHasherDefault<IdentityHasher<K>>>,
    _not_sync: PhantomData<std::cell::Cell<()>>,
}


/// For `T` that implements `Ord`, you can use this struct to quickly
/// set up a max heap.
#[derive(Clone, Copy, Default, PartialEq, Eq, Debug)]
pub struct MaxComparator;

impl<T: Ord> Compare<T> for MaxComparator {
    fn compare(&self, a: &T, b: &T) -> Ordering {
        a.cmp(b)
    }
}

/// For `T` that implements `Ord`, you can use this struct to quickly
/// set up a min heap.
#[derive(Clone, Copy, Default, PartialEq, Eq, Debug)]
pub struct MinComparator;

impl<T: Ord> Compare<T> for MinComparator {
    fn compare(&self, a: &T, b: &T) -> Ordering {
        b.cmp(a)
    }
}

/// The comparator defined by closure
#[derive(Clone, Copy, Default, PartialEq, Eq, Debug)]
pub struct FnComparator<F>(pub F);

impl<T, F> Compare<T> for FnComparator<F>
where
    F: Fn(&T, &T) -> Ordering,
{
    fn compare(&self, a: &T, b: &T) -> Ordering {
        self.0(a, b)
    }
}

/// The comparator ordered by key
#[derive(Clone, Copy, Default, PartialEq, Eq, Debug)]
pub struct KeyComparator<F>(pub F);

impl<K: Ord, T, F> Compare<T> for KeyComparator<F>
where
    F: Fn(&T) -> K,
{
    fn compare(&self, a: &T, b: &T) -> Ordering {
        self.0(a).cmp(&self.0(b))
    }
}

/// Structure wrapping a mutable reference to the first item on a
/// `BinaryHeap`.
///
/// This `struct` is created by the [`peek_mut`] method on [`BinaryHeapIndexKeys`]. See
/// its documentation for more.
///
/// [`peek_mut`]: BinaryHeapIndexKeys::peek_mut
pub struct PeekMut<'a, K: Hash + Eq + IdentityHashable, T: 'a, C: 'a + Compare<T>> {
    heap: &'a mut BinaryHeapIndexKeys<K, T, C>,
    sift: bool,
}

impl<K: fmt::Debug + Hash + Eq + IdentityHashable, T: fmt::Debug, C: Compare<T>> fmt::Debug for PeekMut<'_, K, T, C> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("PeekMut").field(&self.heap.data[0]).finish()
    }
}

impl<K: Hash + Eq + IdentityHashable, T, C: Compare<T>> Drop for PeekMut<'_, K, T, C> {
    fn drop(&mut self) {
        if self.sift {
            // SAFETY: PeekMut is only instantiated for non-empty heaps.
            unsafe { self.heap.sift_down(0) };
        }
    }
}

impl<K: Hash + Eq + IdentityHashable, T, C: Compare<T>> Deref for PeekMut<'_, K, T, C> {
    type Target = T;
    fn deref(&self) -> &T {
        self.key_value().1
    }
}

impl<K: Hash + Eq + IdentityHashable, T, C: Compare<T>> DerefMut for PeekMut<'_, K, T, C> {
    fn deref_mut(&mut self) -> &mut T {
        self.key_value_mut().1
    }
}

impl<K: Hash + Eq + IdentityHashable, T, C: Compare<T>> PeekMut<'_, K, T, C> {
    /// returns the key of the first item on the heap.
    pub fn key(&self) -> &K {
        debug_assert!(!self.heap.is_empty());
        // SAFE: PeekMut is only instantiated for non-empty heaps
        let key_value = unsafe { self.heap.data.get_unchecked(0) };
        &key_value.0
    }

    /// returns the key-value pair that is the first item on the heap.
    pub fn key_value(&self) -> (&K, &T) {
        debug_assert!(!self.heap.is_empty());
        // SAFE: PeekMut is only instantiated for non-empty heaps
        let key_value = unsafe { self.heap.data.get_unchecked(0) };
        (&key_value.0, &key_value.1)
    }

    /// returns a mutable reference to the key-value pair that is the first item on the heap.
    pub fn key_value_mut(&mut self) -> (&mut K, &mut T) {
        debug_assert!(!self.heap.is_empty());
        self.sift = true;
        // SAFE: PeekMut is only instantiated for non-empty heaps
        let key_value = unsafe { self.heap.data.get_unchecked_mut(0) };
        (&mut key_value.0, &mut key_value.1)
    }

    /// Removes the peeked value from the heap and returns it.
    pub fn pop(mut self) -> T {
        let value = self.heap.pop().unwrap();
        self.sift = false;
        value
    }

    /// Removes the peeked value from the heap and returns it as a key-value pair.
    pub fn pop_with_key(mut self) -> (K, T) {
        let key_value = self.heap.pop_with_key().unwrap();
        self.sift = false;
        key_value
    }
}

/// Structure wrapping a mutable reference to any item on a `BinaryHeap`.
///
/// This `struct` is created by the [`get_mut`] method on [`BinaryHeapIndexKeys`]. See
/// its documentation for more.
///
/// [`get_mut`]: BinaryHeapIndexKeys::get_mut
pub struct RefMut<'a, K: 'a + Hash + Eq + IdentityHashable, T: 'a, C: 'a + Compare<T>> {
    heap: &'a mut BinaryHeapIndexKeys<K, T, C>,
    pos: usize,
    key: &'a K,
}

impl<K: fmt::Debug + Hash + Eq + IdentityHashable, T: fmt::Debug, C: Compare<T>> fmt::Debug for RefMut<'_, K, T, C> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("RefMut")
            .field(&self.key)
            .field(&self.heap.data.get(self.pos))
            .finish()
    }
}

impl<K: Hash + Eq + IdentityHashable, T, C: Compare<T>> Drop for RefMut<'_, K, T, C> {
    fn drop(&mut self) {
        self.heap.update(self.key);
    }
}

impl<K: Hash + Eq + IdentityHashable, T, C: Compare<T>> Deref for RefMut<'_, K, T, C> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.heap.data[self.pos].1
    }
}

impl<K: Hash + Eq + IdentityHashable, T, C: Compare<T>> DerefMut for RefMut<'_, K, T, C> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.heap.data[self.pos].1
    }
}

impl<K: Hash + Eq + IdentityHashable, T, C: Compare<T>> RefMut<'_, K, T, C> {
    /// returns the key of the heap item.
    pub fn key(&self) -> &K {
        self.key
    }

    /// returns a key-value pair for this heap item.
    pub fn key_value(&self) -> (&K, &T) {
        (self.key, self)
    }

    /// returns a mutable key-value pair for this heap item.
    /// modifying the key is not possible. Only the value is mutable.
    pub fn key_value_mut(&mut self) -> (&K, &mut T) {
        (self.key, self)
    }
}

impl<K: Clone + IdentityHashable, T: Clone, C: Clone> Clone for BinaryHeapIndexKeys<K, T, C> {
    fn clone(&self) -> Self {
        BinaryHeapIndexKeys {
            data: self.data.clone(),
            cmp: self.cmp.clone(),
            keys: self.keys.clone(),
            _not_sync: PhantomData::default(),
        }
    }

    fn clone_from(&mut self, source: &Self) {
        // TODO unit test
        self.data.clone_from(&source.data);
        self.keys.clone_from(&source.keys);
        self.cmp = source.cmp.clone();
    }
}

impl<K: Hash + Eq + IdentityHashable, T, C: Compare<T> + Default> Default for BinaryHeapIndexKeys<K, T, C> {
    /// Creates an empty `BinaryHeapIndexKeys<K, T>`.
    #[inline]
    fn default() -> BinaryHeapIndexKeys<K, T, C> {
        BinaryHeapIndexKeys::new()
    }
}

impl<K: fmt::Debug + IdentityHashable, T: fmt::Debug, C> fmt::Debug for BinaryHeapIndexKeys<K, T, C> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

impl<K: Hash + Eq + IdentityHashable, T, C: Compare<T> + Default> BinaryHeapIndexKeys<K, T, C> {
    /// Creates an empty `BinaryHeap`.
    ///
    /// This default version will create a max-heap.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use mut_binary_heap_index_keys::BinaryHeapIndexKeys;
    /// let mut heap: BinaryHeapIndexKeys<i32, i32> = BinaryHeapIndexKeys::new();
    /// heap.push(0, 3);
    /// heap.push(1, 1);
    /// heap.push(2, 5);
    /// assert_eq!(heap.pop(), Some(5));
    /// ```
    // #[stable(feature = "rust1", since = "1.0.0")]
    #[must_use]
    pub fn new() -> Self {
        unsafe { BinaryHeapIndexKeys::new_from_data_raw(Vec::new(), HashMap::with_hasher(
            BuildHasherDefault::default()
        ), C::default(), false) }
    }

    /// Creates an empty `BinaryHeap` with a specific capacity.
    /// This preallocates enough memory for `capacity` elements,
    /// so that the `BinaryHeap` does not have to be reallocated
    /// until it contains at least that many values.
    ///
    /// This default version will create a max-heap.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use mut_binary_heap_index_keys::BinaryHeapIndexKeys;
    /// let mut heap: BinaryHeapIndexKeys<i32, i32> = BinaryHeapIndexKeys::with_capacity(10);
    /// assert!(heap.capacity_min() >= 10);
    /// heap.push(0, 3);
    /// heap.push(1, 1);
    /// heap.push(2, 5);
    /// assert_eq!(heap.pop(), Some(5));
    /// ```
    // #[stable(feature = "rust1", since = "1.0.0")]
    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self {
        unsafe {
            BinaryHeapIndexKeys::new_from_data_raw(
                Vec::with_capacity(capacity),
                HashMap::with_capacity_and_hasher(capacity, BuildHasherDefault::default()),
                C::default(),
                false,
            )
        }
    }
}

impl<K: Hash + Eq + Clone + IdentityHashable, T, C: Compare<T> + Default> BinaryHeapIndexKeys<K, T, C> {
    pub fn from<I: IntoIterator<Item = T>, F: Fn(&T) -> K>(values: I, key_selector: F) -> Self {
        values
            .into_iter()
            .map(|value| (key_selector(&value), value))
            .collect()
    }
}

impl<K: IdentityHashable + Hash + Eq, T, C: Compare<T>> BinaryHeapIndexKeys<K, T, C> {
    /// Creates a new Binary Heap from a vec and hashmap.
    ///
    /// # Safety
    ///
    ///  caller is responsible for passing a valid combination of `data`,
    /// `keys` and `rebuild`.
    /// * `data`: there must not be any duplicate keys in data
    /// * `keys`: for each key in `data` there must be a entry in `keys` with the
    ///             index into `data`
    /// * `rebuild`: must be `true` if `data` is not in valid heap-order based on `cmp`
    #[must_use]
    #[doc(hidden)]
    pub unsafe fn new_from_data_raw(
        data: Vec<(K, T)>,
        keys: HashMap<K, usize, BuildHasherDefault<IdentityHasher<K>>>,
        cmp: C,
        rebuild: bool,
    ) -> Self {
        let mut heap = BinaryHeapIndexKeys {
            data,
            cmp,
            keys,
            _not_sync: PhantomData::default(),
        };
        debug_assert!(heap.data.len() == heap.keys.len());
        if rebuild && !heap.data.is_empty() {
            heap.rebuild();
        }
        heap
    }
}

impl<K: Hash + Eq + IdentityHashable, T: Ord> BinaryHeapIndexKeys<K, T, MinComparator> {
    /// Creates an empty `BinaryHeap`.
    ///
    /// The `_min()` version will create a min-heap.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use mut_binary_heap_index_keys::BinaryHeapIndexKeys;
    /// let mut heap = BinaryHeapIndexKeys::new_min();
    /// heap.push(0, 3);
    /// heap.push(1, 1);
    /// heap.push(2, 5);
    /// assert_eq!(heap.pop(), Some(1));
    /// ```
    #[must_use]
    pub fn new_min() -> Self {
        unsafe { BinaryHeapIndexKeys::new_from_data_raw(Vec::new(), HashMap::with_hasher(BuildHasherDefault::default()), MinComparator, false) }
    }

    /// Creates an empty `BinaryHeap` with a specific capacity.
    /// This preallocates enough memory for `capacity` elements,
    /// so that the `BinaryHeap` does not have to be reallocated
    /// until it contains at least that many values.
    ///
    /// The `_min()` version will create a min-heap.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use mut_binary_heap_index_keys::BinaryHeapIndexKeys;
    /// let mut heap = BinaryHeapIndexKeys::with_capacity_min(10);
    /// assert!(heap.capacity_min() >= 10);
    /// heap.push(0, 3);
    /// heap.push(1, 1);
    /// heap.push(2, 5);
    /// assert_eq!(heap.pop(), Some(1));
    /// ```
    #[must_use]
    pub fn with_capacity_min(capacity: usize) -> Self {
        unsafe {
            BinaryHeapIndexKeys::new_from_data_raw(
                Vec::with_capacity(capacity),
                HashMap::with_capacity_and_hasher(capacity, BuildHasherDefault::default()),
                MinComparator,
                false,
            )
        }
    }
}

impl<K: Hash + Eq + IdentityHashable, T, F> BinaryHeapIndexKeys<K, T, FnComparator<F>>
where
    F: Fn(&T, &T) -> Ordering,
{
    /// Creates an empty `BinaryHeap`.
    ///
    /// The `_by()` version will create a heap ordered by given closure.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use mut_binary_heap_index_keys::BinaryHeapIndexKeys;
    /// let mut heap = BinaryHeapIndexKeys::new_by(|a: &i32, b: &i32| b.cmp(a));
    /// heap.push(0, 3);
    /// heap.push(1, 1);
    /// heap.push(2, 5);
    /// assert_eq!(heap.pop(), Some(1));
    /// ```
    #[must_use]
    pub fn new_by(f: F) -> Self {
        unsafe { BinaryHeapIndexKeys::new_from_data_raw(Vec::new(), HashMap::with_hasher(BuildHasherDefault::default()), FnComparator(f), false) }
    }

    /// Creates an empty `BinaryHeap` with a specific capacity.
    /// This preallocates enough memory for `capacity` elements,
    /// so that the `BinaryHeap` does not have to be reallocated
    /// until it contains at least that many values.
    ///
    /// The `_by()` version will create a heap ordered by given closure.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use mut_binary_heap_index_keys::BinaryHeapIndexKeys;
    /// let mut heap = BinaryHeapIndexKeys::with_capacity_by(10, |a: &i32, b: &i32| b.cmp(a));
    /// assert!(heap.capacity_min() >= 10);
    /// heap.push(0, 3);
    /// heap.push(1, 1);
    /// heap.push(2, 5);
    /// assert_eq!(heap.pop(), Some(1));
    /// ```
    #[must_use]
    pub fn with_capacity_by(capacity: usize, f: F) -> Self {
        unsafe {
            BinaryHeapIndexKeys::new_from_data_raw(
                Vec::with_capacity(capacity),
                HashMap::with_capacity_and_hasher(capacity, BuildHasherDefault::default()),
                FnComparator(f),
                false,
            )
        }
    }
}

impl<K: Hash + Eq + IdentityHashable, T, F, C: Ord> BinaryHeapIndexKeys<K, T, KeyComparator<F>>
where
    F: Fn(&T) -> C,
{
    /// Creates an empty `BinaryHeap`.
    ///
    /// The `_by_sort_key()` version will create a heap ordered by
    /// key converted by given closure.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use mut_binary_heap_index_keys::BinaryHeapIndexKeys;
    /// let mut heap = BinaryHeapIndexKeys::new_by_sort_key(|a: &i32| a % 4);
    /// heap.push(0, 3);
    /// heap.push(1, 1);
    /// heap.push(2, 5);
    /// assert_eq!(heap.pop(), Some(3));
    /// ```
    #[must_use]
    pub fn new_by_sort_key(f: F) -> Self {
        unsafe {
            BinaryHeapIndexKeys::new_from_data_raw(Vec::new(), HashMap::with_hasher(BuildHasherDefault::default()), KeyComparator(f), false)
        }
    }

    /// Creates an empty `BinaryHeap` with a specific capacity.
    /// This preallocates enough memory for `capacity` elements,
    /// so that the `BinaryHeap` does not have to be reallocated
    /// until it contains at least that many values.
    ///
    /// The `_by_sort_key()` version will create a heap ordered by
    /// key coverted by given closure.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use mut_binary_heap_index_keys::BinaryHeapIndexKeys;
    /// let mut heap = BinaryHeapIndexKeys::with_capacity_by_sort_key(10, |a: &i32| a % 4);
    /// assert!(heap.capacity_min() >= 10);
    /// heap.push(0, 3);
    /// heap.push(1, 1);
    /// heap.push(2, 5);
    /// assert_eq!(heap.pop(), Some(3));
    /// ```
    #[must_use]
    pub fn with_capacity_by_sort_key(capacity: usize, f: F) -> Self {
        unsafe {
            BinaryHeapIndexKeys::new_from_data_raw(
                Vec::with_capacity(capacity),
                HashMap::with_capacity_and_hasher(capacity, BuildHasherDefault::default()),
                KeyComparator(f),
                false,
            )
        }
    }
}

impl<K: Hash + Eq + Clone + IdentityHashable, T, C: Compare<T>> BinaryHeapIndexKeys<K, T, C> {
    /**
     Pushes an item onto the binary heap.

     If the heap did not have this key present, [None] is returned.

     If the heap did have this key present, the value is updated, and the old
     value is returned. The key is not updated, though; this matters for
     types that can be `==` without being identical. For more information see
     the documentation of [HashMap::insert].

     # Examples

     Basic usage:

     ```
     use mut_binary_heap_index_keys::BinaryHeapIndexKeys;
     let mut heap: BinaryHeapIndexKeys<i32, i32> = BinaryHeapIndexKeys::new();
     heap.push(0, 3);
     heap.push(1, 5);
     heap.push(2, 1);

     assert_eq!(heap.len(), 3);
     assert_eq!(heap.peek(), Some(&5));
     ```

     # Time complexity

     The expected cost of `push`, averaged over every possible ordering of
     the elements being pushed, and over a sufficiently large number of
     pushes, is *O*(1). This is the most meaningful cost metric when pushing
     elements that are *not* already in any sorted pattern.

     The time complexity degrades if elements are pushed in predominantly
     ascending order. In the worst case, elements are pushed in ascending
     sorted order and the amortized cost per push is *O*(log(*n*)) against a heap
     containing *n* elements.

     The worst case cost of a *single* call to `push` is *O*(*n*). The worst case
     occurs when capacity is exhausted and needs a resize. The resize cost
     has been amortized in the previous figures.
    */
    pub fn push(&mut self, key: K, item: T) -> Option<T> {
        if let Some(pos) = self.keys.get(&key).copied() {
            let mut old = std::mem::replace(&mut self.data[pos], (key, item));
            // NOTE: the swap is required in order to keep the guarantee
            // that the key is not replaced by a second push.
            // I would prefer replacing the key, but that is not supported by
            // [HashMap]
            std::mem::swap(&mut old.0, &mut self.data[pos].0);
            self.update(&old.0);
            Some(old.1)
        } else {
            let old_len = self.len();
            self.data.push((key.clone(), item));
            self.keys.insert(key, old_len);
            // SAFETY: Since we pushed a new item it means that
            //  old_len = self.len() - 1 < self.len()
            unsafe { self.sift_up(0, old_len) };
            None
        }
    }
}

impl<K: Hash + Eq + IdentityHashable, T, C: Compare<T>> BinaryHeapIndexKeys<K, T, C> {
    /// Returns a mutable reference to the first item in the binary heap, or
    /// `None` if it is empty.
    ///
    /// Note: If the `PeekMut` value is leaked, the heap may be in an
    /// inconsistent state.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use mut_binary_heap_index_keys::BinaryHeapIndexKeys;
    /// let mut heap: BinaryHeapIndexKeys<i32, i32> = BinaryHeapIndexKeys::new();
    /// assert!(heap.peek_mut().is_none());
    ///
    /// heap.push(0, 1);
    /// heap.push(1, 5);
    /// heap.push(2, 2);
    /// {
    ///     let mut val = heap.peek_mut().unwrap();
    ///     assert_eq!(*val, 5);
    ///     *val = 0;
    /// }
    /// assert_eq!(heap.peek(), Some(&2));
    /// ```
    ///
    /// # Time complexity
    ///
    /// If the item is modified then the worst case time complexity is *O*(log(*n*)),
    /// otherwise it's *O*(1).
    // #[stable(feature = "binary_heap_peek_mut", since = "1.12.0")]
    pub fn peek_mut(&mut self) -> Option<PeekMut<'_, K, T, C>> {
        if self.is_empty() {
            None
        } else {
            Some(PeekMut {
                heap: self,
                sift: false,
            })
        }
    }

    /// Removes the greatest item from the binary heap and returns it, or `None` if it
    /// is empty.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use mut_binary_heap_index_keys::BinaryHeapIndexKeys;
    /// let mut heap = BinaryHeapIndexKeys::<_, _>::from([1, 3], |v| v.clone());
    ///
    /// assert_eq!(heap.pop(), Some(3));
    /// assert_eq!(heap.pop(), Some(1));
    /// assert_eq!(heap.pop(), None);
    /// ```
    ///
    /// # Time complexity
    ///
    /// The worst case cost of `pop` on a heap containing *n* elements is *O*(log(*n*)).
    pub fn pop(&mut self) -> Option<T> {
        self.pop_with_key().map(|kv| kv.1)
    }

    /// Removes the greatest item from the binary heap and returns it as a key-value pair,
    /// or `None` if it is empty.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use mut_binary_heap_index_keys::BinaryHeapIndexKeys;
    /// let mut heap = BinaryHeapIndexKeys::<_,_>::from(vec![1, 3], |v| v.clone());
    ///
    /// assert_eq!(heap.pop_with_key(), Some((3, 3)));
    /// assert_eq!(heap.pop_with_key(), Some((1, 1)));
    /// assert_eq!(heap.pop_with_key(), None);
    /// ```
    ///
    /// # Time complexity
    ///
    /// The worst case cost of `pop` on a heap containing *n* elements is *O*(log(*n*)).
    pub fn pop_with_key(&mut self) -> Option<(K, T)> {
        let item = self.data.pop().map(|mut item| {
            // NOTE: we can't just use self.is_empty here, because that will
            //  trigger a debug_assert that keys and data are equal lenght.
            if !self.data.is_empty() {
                swap(&mut item, &mut self.data[0]);
                // SAFETY: !self.is_empty() means that self.len() > 0
                unsafe { self.sift_down_to_bottom(0) };
            }
            item
        });
        item.as_ref().and_then(|kv| self.keys.remove(&kv.0));
        item
    }

    /// Returns `true` if the heap contains a value for the given key.
    ///
    /// # Examples
    /// ```
    /// use mut_binary_heap_index_keys::BinaryHeapIndexKeys;
    /// let mut heap = BinaryHeapIndexKeys::<_,_>::from([1, 3], |v| v.clone());
    ///
    /// assert!(heap.contains_key(&1));
    /// assert!(heap.contains_key(&3));
    /// assert!(!heap.contains_key(&2));
    /// ```
    ///
    /// # Time complexity
    ///
    /// This method runs in *O*(1) time.
    ///
    pub fn contains_key(&self, key: &K) -> bool {
        self.keys.contains_key(key)
    }

    /// Returns a reference to the value for a given key or [None] if the key does not exist.
    ///
    /// # Examples
    /// ```
    /// use mut_binary_heap_index_keys::BinaryHeapIndexKeys;
    /// let mut heap = BinaryHeapIndexKeys::<_,_>::from(vec![1, 3], |v| v.clone());
    ///
    /// assert_eq!(heap.get(&1), Some(&1));
    /// assert_eq!(heap.get(&2), None);
    /// ```
    ///
    /// # Time complecity
    ///
    /// This method runs in *O*(1) time.
    pub fn get(&self, key: &K) -> Option<&T> {
        self.keys.get(key).map(|index| &self.data[*index].1)
    }

    /// Returns a mutable reference to the value for a given key or
    /// [None] if the key does not exist.
    ///
    /// The heap is updated when [RefMut] is dropped.
    /// # Examples
    /// ```
    /// use mut_binary_heap_index_keys::BinaryHeapIndexKeys;
    /// let mut heap = BinaryHeapIndexKeys::<i32, i32>::from(vec![1, 3], |v| v.clone());
    ///
    /// {
    ///     let mut v = heap.get_mut(&1).unwrap();
    ///     assert_eq!(*v, 1);
    ///     *v = 5;
    ///     // Drop updates the heap
    /// }
    /// assert_eq!(heap.peek_with_key(), Some((&1, &5)));
    /// assert_eq!(heap.get(&2), None);
    /// ```
    ///
    /// # Time complecity
    ///
    pub fn get_mut<'a>(&'a mut self, key: &'a K) -> Option<RefMut<'a, K, T, C>> {
        self.keys.get(key).copied().map(|pos| RefMut {
            heap: self,
            pos,
            key,
        })
    }

    /// Removes a key from the heap, returning the `(key, value)` if the key
    /// was previously in the heap.
    ///
    /// The key may be any borrowed form of the map's key type, but
    /// [Hash] and [Eq] on the borrowed form *must* match those for
    /// the key type.
    ///
    /// # Example
    /// ```
    /// use mut_binary_heap_index_keys::BinaryHeapIndexKeys;
    ///
    /// let mut heap: BinaryHeapIndexKeys<_, _> = BinaryHeapIndexKeys::new();
    /// heap.push(0, 5);
    /// heap.push(1, 3);
    /// heap.push(2, 6);
    ///
    /// assert_eq!(heap.remove(&0), Some((0, 5)));
    /// assert_eq!(heap.remove(&3), None);
    /// assert_eq!(heap.len(), 2);
    /// assert_eq!(heap.pop(), Some(6));
    /// assert_eq!(heap.pop(), Some(3));
    ///
    /// ```
    pub fn remove(&mut self, key: &K) -> Option<(K, T)> {
        if let Some(pos) = self.keys.get(key).copied() {
            let item = self.data.pop().map(|mut item| {
                if !self.data.is_empty() && pos < self.data.len() {
                    swap(&mut item, &mut self.data[pos]);
                    // SAFETY: !self.is_empty && pos < self.data.len()
                    unsafe { self.sift_down_to_bottom(pos) };
                }
                item
            });
            item.as_ref().and_then(|kv| self.keys.remove(&kv.0));
            item
        } else {
            None
        }
    }

    /// Updates the binary heap after the value behind this key was modified.
    ///
    /// This is called by [push] if the key already existed and also by [RefMut].
    ///
    /// This function will panic if the key is not part of the binary heap.
    /// A none panicing alternative is to check with [BinaryHeapIndexKeys::contains_key]
    /// or using [BinaryHeapIndexKeys::get_mut] instead.
    ///
    /// # Time complexity
    ///
    /// This function runs in *O*(*log* n) time.
    #[doc(hidden)]
    pub fn update(&mut self, key: &K) {
        let pos = self.keys[key];
        let pos_after_sift_up = unsafe { self.sift_up(0, pos) };
        if pos_after_sift_up != pos {
            return;
        }
        unsafe {
            self.sift_down(pos);
        }
    }

    /// Consumes the `BinaryHeap` and returns a vector in sorted
    /// (ascending) order.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use mut_binary_heap_index_keys::BinaryHeapIndexKeys;
    ///
    /// let mut heap = BinaryHeapIndexKeys::<_, _>::from([1, 2, 4, 5, 7], |v| v.clone());
    /// heap.push(0, 6);
    /// heap.push(1, 3);
    ///
    /// // let vec = heap.into_sorted_vec();
    /// // assert_eq!(vec, [1, 2, 3, 4, 5, 6, 7]);
    /// ```
    // #[must_use = "`self` will be dropped if the result is not used"]
    // // #[stable(feature = "binary_heap_extras_15", since = "1.5.0")]
    // pub fn into_sorted_vec(mut self) -> Vec<T> {
    //     let mut end = self.len();
    //     while end > 1 {
    //         end -= 1;
    //         // SAFETY: `end` goes from `self.len() - 1` to 1 (both included),
    //         //  so it's always a valid index to access.
    //         //  It is safe to access index 0 (i.e. `ptr`), because
    //         //  1 <= end < self.len(), which means self.len() >= 2.
    //         unsafe {
    //             let ptr = self.data.as_mut_ptr();
    //             ptr::swap(ptr, ptr.add(end));
    //         }
    //         // SAFETY: `end` goes from `self.len() - 1` to 1 (both included) so:
    //         //  0 < 1 <= end <= self.len() - 1 < self.len()
    //         //  Which means 0 < end and end < self.len().
    //         unsafe { self.sift_down_range(0, end) };
    //     }
    //     self.into_vec()
    // }

    // The implementations of sift_up and sift_down use unsafe blocks in
    // order to move an element out of the vector (leaving behind a
    // hole), shift along the others and move the removed element back into the
    // vector at the final location of the hole.
    // The `Hole` type is used to represent this, and make sure
    // the hole is filled back at the end of its scope, even on panic.
    // Using a hole reduces the constant factor compared to using swaps,
    // which involves twice as many moves.

    /// Reserves capacity for at least `additional` more elements to be inserted in the
    /// `BinaryHeap`. The collection may reserve more space to avoid frequent reallocations.
    ///
    /// # Panics
    ///
    /// Panics if the new capacity overflows `usize`.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use mut_binary_heap_index_keys::BinaryHeapIndexKeys;
    /// let mut heap: BinaryHeapIndexKeys<_, _> = BinaryHeapIndexKeys::new();
    /// heap.reserve(100);
    /// assert!(heap.capacity_min() >= 100);
    /// heap.push(0, 4);
    /// ```
    // #[stable(feature = "rust1", since = "1.0.0")]
    pub fn reserve(&mut self, additional: usize) {
        self.data.reserve(additional);
        self.keys.reserve(additional);
    }

    /// Discards as much additional capacity as possible.
    /// The implementation of [Vec] and [HashMap] the exact value of the
    /// new capacity.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use mut_binary_heap_index_keys::BinaryHeapIndexKeys;
    /// let mut heap: BinaryHeapIndexKeys<i32, i32> = BinaryHeapIndexKeys::with_capacity(100);
    ///
    /// assert!(heap.capacity_min() >= 100);
    /// heap.shrink_to_fit();
    /// assert!(heap.capacity_min() >= 0);
    /// ```
    // #[stable(feature = "rust1", since = "1.0.0")]
    pub fn shrink_to_fit(&mut self) {
        self.data.shrink_to_fit();
        self.keys.shrink_to_fit();
    }

    /// Discards capacity with a lower bound.
    /// The implementation of [Vec] and [HashMap] the exact value of the
    /// new capacity.
    ///
    /// The capacity will remain at least as large as both the length
    /// and the supplied value.
    ///
    /// If the current capacity is less than the lower limit, this is a no-op.
    ///
    /// # Examples
    ///
    /// ```
    /// use mut_binary_heap_index_keys::BinaryHeapIndexKeys;
    /// let mut heap: BinaryHeapIndexKeys<i32, i32> = BinaryHeapIndexKeys::with_capacity(100);
    ///
    /// assert!(heap.capacity_min() >= 100);
    /// heap.shrink_to(10);
    /// assert!(heap.capacity_min() >= 10);
    /// ```
    #[inline]
    pub fn shrink_to(&mut self, min_capacity: usize) {
        self.data.shrink_to(min_capacity);
        self.keys.shrink_to(min_capacity);
    }

    /// # Safety
    ///
    /// The caller must guarantee that `pos < self.data.len()`.
    unsafe fn sift_up(&mut self, start: usize, pos: usize) -> usize {
        // Take out the value at `pos` and create a hole.
        // SAFETY: The caller guarantees that pos < self.data.len()
        let mut hole = unsafe { Hole::new(&mut self.data, &mut self.keys, pos) };

        while hole.pos() > start {
            let parent = (hole.pos() - 1) / 2;

            // SAFETY: hole.pos() > start >= 0, which means hole.pos() > 0
            //  and so hole.pos() - 1 can't underflow.
            //  This guarantees that parent < hole.pos() so
            //  it's a valid index and also != hole.pos().
            if self
                .cmp
                .compares_le(hole.element(), unsafe { hole.get(parent) })
            {
                break;
            }

            // SAFETY: Same as above
            unsafe { hole.move_to(parent) };
        }

        hole.pos()
    }

    /// Take an element at `pos` and move it down the heap,
    /// while its children are larger.
    ///
    /// # Safety
    ///
    /// The caller must guarantee that `pos < end <= self.data.len()`.
    unsafe fn sift_down_range(&mut self, pos: usize, end: usize) {
        // SAFETY: The caller guarantees that pos < end <= self.data.len().
        let mut hole = unsafe { Hole::new(&mut self.data, &mut self.keys, pos) };
        let mut child = 2 * hole.pos() + 1;

        // Loop invariant: child == 2 * hole.pos() + 1.
        while child <= end.saturating_sub(2) {
            // compare with the greater of the two children
            // SAFETY: child < end - 1 < self.data.len() and
            //  child + 1 < end <= self.data.len(), so they're valid indexes.
            //  child == 2 * hole.pos() + 1 != hole.pos() and
            //  child + 1 == 2 * hole.pos() + 2 != hole.pos().
            // FIXME: 2 * hole.pos() + 1 or 2 * hole.pos() + 2 could overflow
            //  if T is a ZST
            child += unsafe { self.cmp.compares_le(hole.get(child), hole.get(child + 1)) } as usize;

            // if we are already in order, stop.
            // SAFETY: child is now either the old child or the old child+1
            //  We already proven that both are < self.data.len() and != hole.pos()
            if self
                .cmp
                .compares_ge(hole.element(), unsafe { hole.get(child) })
            {
                return;
            }

            // SAFETY: same as above.
            unsafe { hole.move_to(child) };
            child = 2 * hole.pos() + 1;
        }

        // SAFETY: && short circuit, which means that in the
        //  second condition it's already true that child == end - 1 < self.data.len().
        if child == end - 1
            && self
                .cmp
                .compares_lt(hole.element(), unsafe { hole.get(child) })
        {
            // SAFETY: child is already proven to be a valid index and
            //  child == 2 * hole.pos() + 1 != hole.pos().
            unsafe { hole.move_to(child) };
        }
    }

    /// # Safety
    ///
    /// The caller must guarantee that `pos < self.data.len()`.
    unsafe fn sift_down(&mut self, pos: usize) {
        let len = self.data.len();
        // SAFETY: pos < len is guaranteed by the caller and
        //  obviously len = self.data.len() <= self.len().
        unsafe { self.sift_down_range(pos, len) };
    }

    /// Take an element at `pos` and move it all the way down the heap,
    /// then sift it up to its position.
    ///
    /// Note: This is faster when the element is known to be large / should
    /// be closer to the bottom.
    ///
    /// # Safety
    ///
    /// The caller must guarantee that `pos < self.data.len()`.
    unsafe fn sift_down_to_bottom(&mut self, mut pos: usize) {
        let end = self.data.len();
        let start = pos;

        // SAFETY: The caller guarantees that pos < self.data.len().
        let mut hole = unsafe { Hole::new(&mut self.data, &mut self.keys, pos) };
        let mut child = 2 * hole.pos() + 1;

        // Loop invariant: child == 2 * hole.pos() + 1.
        while child <= end.saturating_sub(2) {
            // SAFETY: child < end - 1 < self.data.len() and
            //  child + 1 < end <= self.data.len(), so they're valid indexes.
            //  child == 2 * hole.pos() + 1 != hole.pos() and
            //  child + 1 == 2 * hole.pos() + 2 != hole.pos().
            // FIXME: 2 * hole.pos() + 1 or 2 * hole.pos() + 2 could overflow
            //  if T is a ZST
            child += unsafe { self.cmp.compares_le(hole.get(child), hole.get(child + 1)) } as usize;

            // SAFETY: Same as above
            unsafe { hole.move_to(child) };
            child = 2 * hole.pos() + 1;
        }

        if child == end - 1 {
            // SAFETY: child == end - 1 < self.data.len(), so it's a valid index
            //  and child == 2 * hole.pos() + 1 != hole.pos().
            unsafe { hole.move_to(child) };
        }
        pos = hole.pos();
        drop(hole);

        // SAFETY: pos is the position in the hole and was already proven
        //  to be a valid index.
        unsafe { self.sift_up(start, pos) };
    }

    /// Rebuild assuming data[0..start] is still a proper heap.
    #[allow(dead_code)] // TODO this is unused because append is currently not implemented
    fn rebuild_tail(&mut self, start: usize) {
        if start == self.len() {
            return;
        }

        let tail_len = self.len() - start;

        #[inline(always)]
        fn log2_fast(x: usize) -> usize {
            (usize::BITS - x.leading_zeros() - 1) as usize
        }

        // `rebuild` takes O(self.data.len()) operations
        // and about 2 * self.data.len() comparisons in the worst case
        // while repeating `sift_up` takes O(tail_len * log(start)) operations
        // and about 1 * tail_len * log_2(start) comparisons in the worst case,
        // assuming start >= tail_len. For larger heaps, the crossover point
        // no longer follows this reasoning and was determined empirically.
        let better_to_rebuild = if start < tail_len {
            true
        } else if self.data.len() <= 2048 {
            2 * self.data.len() < tail_len * log2_fast(start)
        } else {
            2 * self.data.len() < tail_len * 11
        };

        if better_to_rebuild {
            self.rebuild();
        } else {
            for i in start..self.data.len() {
                // SAFETY: The index `i` is always less than self.data.len().
                unsafe { self.sift_up(0, i) };
            }
        }
    }

    /// rebuild the entire heap.
    ///
    /// In some cases it might be faster to rebuild
    /// the entire heap instead of just updating the specific elements that have
    /// been modified.
    fn rebuild(&mut self) {
        let mut n = self.len() / 2;
        while n > 0 {
            n -= 1;
            // SAFETY: n starts from self.data.len() / 2 and goes down to 0.
            //  The only case when !(n < self.data.len()) is if
            //  self.data.len() == 0, but it's ruled out by the loop condition.
            unsafe { self.sift_down(n) };
        }
    }

    //    /// Moves all the elements of `other` into `self`, leaving `other` empty.
    //    ///
    //    /// # Examples
    //    ///
    //    /// Basic usage:
    //    ///
    //    /// ```
    //    /// use mut_binary_heap_index_keys::BinaryHeapIndexKeys;
    //    ///
    //    /// let mut a = BinaryHeapIndexKeys::from([-10, 1, 2, 3, 3]);
    //    /// let mut b = BinaryHeapIndexKeys::from([-20, 5, 43]);
    //    ///
    //    /// a.append(&mut b);
    //    ///
    //    /// assert_eq!(a.into_sorted_vec(), [-20, -10, 1, 2, 3, 3, 5, 43]);
    //    /// assert!(b.is_empty());
    //    /// ```
    //    ///
    // pub fn append(&mut self, other: &mut Self) {
    //     if self.len() < other.len() {
    //         swap(self, other);
    //     }
    //
    //     let start = self.data.len();
    //
    //     // TODO append needs to also copy keys. How do we handle duplicate keys?
    //     self.data.append(&mut other.data);
    //
    //     self.rebuild_tail(start);
    // }
}

impl<K: IdentityHashable, T, C> BinaryHeapIndexKeys<K, T, C> {
    /// Returns an iterator visiting all key-value pairs in the underlying vector, in
    /// arbitrary order.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use mut_binary_heap_index_keys::BinaryHeapIndexKeys;
    /// let heap = BinaryHeapIndexKeys::<_,_>::from([1, 2, 3, 4], |v| v.clone());
    ///
    /// // Print (1, 1), (2, 2), (3, 3), (4, 4) in arbitrary order
    /// for x in heap.iter() {
    ///     println!("key {}, value {}", x.0, x.1);
    /// }
    /// ```
    pub fn iter(&self) -> Iter<'_, K, T> {
        Iter {
            iter: self.data.iter(),
        }
    }

    /// Returns an iterator visiting all values in the underlying vector, in
    /// arbitrary order.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use mut_binary_heap_index_keys::BinaryHeapIndexKeys;
    /// let heap = BinaryHeapIndexKeys::<_,_>::from([1, 2, 3, 4], |v| v.clone());
    ///
    /// // Print 1, 2, 3, 4 in arbitrary order
    /// for x in heap.iter_values() {
    ///     println!("{}", x);
    /// }
    /// ```
    pub fn iter_values(&self) -> IterValues<'_, K, T> {
        IterValues {
            iter: self.data.iter(),
        }
    }

    /// Returns an iterator visiting all keys in the underlying vector, in
    /// arbitrary order.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use mut_binary_heap_index_keys::BinaryHeapIndexKeys;
    /// let heap = BinaryHeapIndexKeys::<_,_>::from([1, 2, 3, 4], |v| v.clone());
    ///
    /// // Print 1, 2, 3, 4 in arbitrary order
    /// for x in heap.iter_keys() {
    ///     println!("{}", x);
    /// }
    /// ```
    pub fn iter_keys(&self) -> IterKeys<'_, K, T> {
        IterKeys {
            iter: self.data.iter(),
        }
    }

    /// Creates a consuming iterator, that is, one that moves each value out of
    /// the heap in arbitrary order. The heap cannot be used after calling this.
    ///
    /// See also [BinaryHeapIndexKeys::into_iter()], [BinaryHeapIndexKeys::into_keys()]
    pub fn into_values(self) -> IntoValues<K, T> {
        IntoValues {
            iter: self.data.into_iter(),
        }
    }

    /// Creates a consuming iterator, that is, one that moves each key out of
    /// the heap in arbitrary order. The heap cannot be used after calling this.
    ///
    /// See also [BinaryHeapIndexKeys::into_iter()], [BinaryHeapIndexKeys::into_values()]
    pub fn into_keys(self) -> IntoKeys<K, T> {
        IntoKeys {
            iter: self.data.into_iter(),
        }
    }

    /// Returns an iterator which retrieves elements in heap order.
    /// This method consumes the original heap.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use mut_binary_heap_index_keys::BinaryHeapIndexKeys;
    /// let heap = BinaryHeapIndexKeys::<_,_>::from([1, 2, 3, 4, 5], |v| v.clone());
    ///
    /// assert_eq!(heap.into_iter_sorted().take(2).collect::<Vec<_>>(), [5, 4]);
    /// ```
    // #[unstable(feature = "binary_heap_into_iter_sorted", issue = "59278")]
    pub fn into_iter_sorted(self) -> IntoIterSorted<K, T, C> {
        IntoIterSorted { inner: self }
    }

    /// Returns the greatest item in the binary heap, or `None` if it is empty.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use mut_binary_heap_index_keys::BinaryHeapIndexKeys;
    /// let mut heap: BinaryHeapIndexKeys<_, _> = BinaryHeapIndexKeys::new();
    /// assert_eq!(heap.peek(), None);
    ///
    /// heap.push(1, 1);
    /// heap.push(2, 5);
    /// heap.push(3, 2);
    /// assert_eq!(heap.peek(), Some(&5));
    ///
    /// ```
    ///
    /// # Time complexity
    ///
    /// Cost is *O*(1) in the worst case.
    #[must_use]
    // #[stable(feature = "rust1", since = "1.0.0")]
    pub fn peek(&self) -> Option<&T> {
        self.peek_with_key().map(|kv| kv.1)
    }

    /// Returns the greatest item in the binary heap as a key-value pair,
    /// or `None` if it is empty.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use mut_binary_heap_index_keys::BinaryHeapIndexKeys;
    /// let mut heap: BinaryHeapIndexKeys<_, _> = BinaryHeapIndexKeys::new();
    /// assert_eq!(heap.peek(), None);
    ///
    /// heap.push(1, 1);
    /// heap.push(2, 5);
    /// heap.push(3, 2);
    /// assert!(heap.peek_mut().is_some());
    /// assert_eq!(heap.peek_mut().unwrap().key_value(), (&2, &5));
    ///
    /// ```
    ///
    /// # Time complexity
    ///
    /// Cost is *O*(1) in the worst case.
    #[must_use]
    pub fn peek_with_key(&self) -> Option<(&K, &T)> {
        let kv = self.data.get(0);
        kv.map(|kv| (&kv.0, &kv.1))
    }

    /// Returns the number of elements the binary heap can hold without reallocating.
    /// Returns a touple with the capacity of the internal vector and hashmap.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use mut_binary_heap_index_keys::BinaryHeapIndexKeys;
    /// let mut heap: BinaryHeapIndexKeys<_, _> = BinaryHeapIndexKeys::with_capacity(100);
    /// assert!(heap.capacity().0 >= 100);
    /// assert!(heap.capacity().1 >= 100);
    /// heap.push(0, 4);
    /// ```
    #[must_use]
    pub fn capacity(&self) -> (usize, usize) {
        (self.data.capacity(), self.keys.capacity())
    }

    /// Returns the minimum number of elements the binary heap can hold without reallocating.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use mut_binary_heap_index_keys::BinaryHeapIndexKeys;
    /// let mut heap: BinaryHeapIndexKeys<_, _> = BinaryHeapIndexKeys::with_capacity(100);
    /// assert!(heap.capacity_min() >= 100);
    /// heap.push(0, 4);
    /// ```
    #[must_use]
    pub fn capacity_min(&self) -> usize {
        min(self.data.capacity(), self.keys.capacity())
    }

    /// Consumes the `BinaryHeap` and returns the underlying vector
    /// in arbitrary order.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use mut_binary_heap_index_keys::BinaryHeapIndexKeys;
    /// let heap = BinaryHeapIndexKeys::<_,_>::from([1, 2, 3, 4, 5, 6, 7], |v| v.clone());
    /// // let vec = heap.into_vec();
    ///
    /// // Will print in some order
    /// // for x in vec {
    /// //    println!("{}", x);
    /// // }
    /// ```
    // TODO into_vec impl and type def
    // #[must_use = "`self` will be dropped if the result is not used"]
    // // #[stable(feature = "binary_heap_extras_15", since = "1.5.0")]
    // pub fn into_vec(self) -> Vec<T> {
    //     self.into()
    // }

    /// Returns the length of the binary heap.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use mut_binary_heap_index_keys::BinaryHeapIndexKeys;
    /// let heap = BinaryHeapIndexKeys::<_,_>::from([1, 3].iter(), |v| *v.clone());
    ///
    /// assert_eq!(heap.len(), 2);
    /// ```
    #[must_use]
    // #[stable(feature = "rust1", since = "1.0.0")]
    pub fn len(&self) -> usize {
        debug_assert!(self.data.len() == self.keys.len());
        self.data.len()
    }

    /// Checks if the binary heap is empty.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use mut_binary_heap_index_keys::BinaryHeapIndexKeys;
    /// let mut heap: BinaryHeapIndexKeys<_, _> = BinaryHeapIndexKeys::new();
    ///
    /// assert!(heap.is_empty());
    ///
    /// heap.push(0, 3);
    /// heap.push(1, 5);
    /// heap.push(2, 1);
    ///
    /// assert!(!heap.is_empty());
    /// ```
    #[must_use]
    // #[stable(feature = "rust1", since = "1.0.0")]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Clears the binary heap, returning an iterator over the removed elements
    /// in arbitrary order. If the iterator is dropped before being fully
    /// consumed, it drops the remaining elements in arbitrary order.
    ///
    /// The returned iterator keeps a mutable borrow on the heap to optimize
    /// its implementation.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use mut_binary_heap_index_keys::BinaryHeapIndexKeys;
    /// let mut heap = BinaryHeapIndexKeys::<_,_>::from([1, 3].iter().clone(), |v| *v.clone());
    ///
    /// assert!(!heap.is_empty());
    ///
    /// for x in heap.drain() {
    ///     println!("key {}, value {}", x.0, x.1);
    /// }
    ///
    /// assert!(heap.is_empty());
    /// ```
    #[inline]
    pub fn drain(&mut self) -> Drain<'_, (K, T)> {
        self.keys.clear();
        Drain {
            iter: self.data.drain(..),
        }
    }

    /// Drops all items from the binary heap.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use mut_binary_heap_index_keys::BinaryHeapIndexKeys;
    /// let mut heap = BinaryHeapIndexKeys::<_,_>::from([1, 3].iter(), |v| *v.clone());
    ///
    /// assert!(!heap.is_empty());
    ///
    /// heap.clear();
    ///
    /// assert!(heap.is_empty());
    /// ```
    pub fn clear(&mut self) {
        self.drain();
    }
}


/// Hole represents a hole in a slice i.e., an index without valid value
/// (because it was moved from or duplicated).
/// In drop, `Hole` will restore the slice by filling the hole
/// position with the value that was originally removed.
struct Hole<'a, K: Hash + Eq + IdentityHashable, T: 'a> {
    data: &'a mut [(K, T)],
    keys: &'a mut HashMap<K, usize, BuildHasherDefault<IdentityHasher<K>>>,
    elt: ManuallyDrop<(K, T)>,
    pos: usize,
}

impl<'a, K: Hash + Eq + IdentityHashable, T> Hole<'a, K, T> {
    /// Create a new `Hole` at index `pos`.
    ///
    /// Unsafe because pos must be within the data slice.
    #[inline]
    unsafe fn new(data: &'a mut [(K, T)], keys: &'a mut HashMap<K, usize, BuildHasherDefault<IdentityHasher<K>>>, pos: usize) -> Self {
        debug_assert!(pos < data.len());
        // SAFE: pos should be inside the slice
        let elt = unsafe { ptr::read(data.get_unchecked(pos)) };
        debug_assert!(keys.contains_key(&elt.0));
        Hole {
            data,
            keys,
            elt: ManuallyDrop::new(elt),
            pos,
        }
    }

    /// Returns the position of the hole.
    #[inline]
    fn pos(&self) -> usize {
        self.pos
    }

    /// Returns a reference to the element removed.
    #[inline]
    fn element(&self) -> &T {
        &self.elt.1
    }

    /// Returns a reference to the element at `index`.
    ///
    /// # Safety
    ///
    /// Index must be within the data slice and not equal to pos.
    #[inline]
    unsafe fn get(&self, index: usize) -> &T {
        debug_assert!(index != self.pos);
        debug_assert!(index < self.data.len());
        let key_value = unsafe { self.data.get_unchecked(index) };
        &key_value.1
    }

    /// Move hole to new location
    ///
    /// # Safety
    ///
    /// target_position must be within the data slice and not equal to pos.
    #[inline]
    unsafe fn move_to(&mut self, target_position: usize) {
        debug_assert!(target_position != self.pos);
        debug_assert!(target_position < self.data.len());
        unsafe {
            let ptr = self.data.as_mut_ptr();
            let target_ptr: *const _ = ptr.add(target_position);

            // update target index in key map
            let target_element: &(K, T) = &*target_ptr;
            *self.keys.get_mut(&target_element.0).expect(
                "Hole can only exist for key values pairs, that are already part of the heap.",
            ) = self.pos;

            // move target into hole
            let hole_ptr = ptr.add(self.pos);
            ptr::copy_nonoverlapping(target_ptr, hole_ptr, 1);
        }
        // update hole position
        self.pos = target_position;
    }
}

impl<K: Hash + Eq + IdentityHashable, T> Drop for Hole<'_, K, T> {
    #[inline]
    fn drop(&mut self) {
        // fill the hole again
        unsafe {
            let pos = self.pos;
            ptr::copy_nonoverlapping(&*self.elt, self.data.get_unchecked_mut(pos), 1);
        }
        let key = &self.elt.0;
        *self.keys.get_mut(key).expect(
            "Hole can only exist for key values pairs, that are already part of the heap.",
        ) = self.pos;
    }
}

#[must_use = "iterators are lazy and do nothing unless consumed"]
// #[unstable(feature = "binary_heap_into_iter_sorted", issue = "59278")]
#[derive(Clone, Debug)]
pub struct IntoIterSorted<K: IdentityHashable, T, C> {
    inner: BinaryHeapIndexKeys<K, T, C>,
}

// #[unstable(feature = "binary_heap_into_iter_sorted", issue = "59278")]
impl<K: Hash + Eq + IdentityHashable, T, C: Compare<T>> Iterator for IntoIterSorted<K, T, C> {
    type Item = T; // TODO IntoIterSorted should return (K, T) and we need a variant for only keys or values

    #[inline]
    fn next(&mut self) -> Option<T> {
        self.inner.pop()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let exact = self.inner.len();
        (exact, Some(exact))
    }
}

/// A draining iterator over the elements of a `BinaryHeap`.
///
/// This `struct` is created by [`BinaryHeapIndexKeys::drain()`]. See its
/// documentation for more.
#[derive(Debug)]
pub struct Drain<'a, T: 'a> {
    iter: vec::Drain<'a, T>,
}

impl<T> Iterator for Drain<'_, T> {
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<T> {
        self.iter.next()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<T> DoubleEndedIterator for Drain<'_, T> {
    #[inline]
    fn next_back(&mut self) -> Option<T> {
        self.iter.next_back()
    }
}

// #[stable(feature = "drain", since = "1.6.0")]
// impl<'a, T: 'a> ExactSizeIterator for Drain<'a, T> {
//     fn is_empty(&self) -> bool {
//         self.iter.is_empty()
//     }
// }

// #[stable(feature = "fused", since = "1.26.0")]
// impl<'a, T: 'a> FusedIterator for Drain<'a, T> {}

// TODO From implementations
// // #[stable(feature = "binary_heap_extras_15", since = "1.5.0")]
// impl<K, T: Ord> From<Vec<T>> for BinaryHeapIndexKeys<K, T> {
//     /// Converts a `Vec<T>` into a `BinaryHeapIndexKeys<K, T>`.
//     ///
//     /// This conversion happens in-place, and has *O*(*n*) time complexity.
//     fn from(vec: Vec<T>) -> Self {
//         BinaryHeapIndexKeys::from_vec(vec)
//     }
// }

// impl<K, T: Ord, const N: usize> From<[T; N]> for BinaryHeapIndexKeys<K, T> {
//     /// ```
//     /// use mut_binary_heap_index_keys::BinaryHeapIndexKeys;
//     ///
//     /// let mut h1 = BinaryHeapIndexKeys::from([1, 4, 2, 3]);
//     /// let mut h2: BinaryHeapIndexKeys<_> = [1, 4, 2, 3].into();
//     /// while let Some((a, b)) = h1.pop().zip(h2.pop()) {
//     ///     assert_eq!(a, b);
//     /// }
//     /// ```
//     fn from(arr: [T; N]) -> Self {
//         Self::from_iter(arr)
//     }
// }

// impl<K, T, C> From<BinaryHeapIndexKeys<K, T, C>> for Vec<T> {
//     /// Converts a `BinaryHeapIndexKeys<K, T>` into a `Vec<T>`.
//     ///
//     /// This conversion requires no data movement or allocation, and has
//     /// constant time complexity.
//     fn from(heap: BinaryHeapIndexKeys<K, T, C>) -> Vec<T> {
//         heap.data
//     }
// }

// #[stable(feature = "rust1", since = "1.0.0")]
// impl<K: Hash + Eq + Clone, T: Ord> FromIterator<(K, T)> for BinaryHeapIndexKeys<K, T> {
//     fn from_iter<I: IntoIterator<Item = (K, T)>>(iter: I) -> Self {
//         let iter = iter.into_iter();
//         let size_hint = iter.size_hint().0;

//         let mut heap = BinaryHeapIndexKeys::with_capacity(size_hint);

//         for (key, value) in iter {
//             heap.data.push((key.clone(), value));
//             heap.keys.insert(key, heap.data.len() - 1);
//         }
//         heap.rebuild();
//         heap
//     }
// }

impl<K: Hash + Eq + Clone + IdentityHashable, T, C: Compare<T> + Default> FromIterator<(K, T)>
    for BinaryHeapIndexKeys<K, T, C>
{
    fn from_iter<I: IntoIterator<Item = (K, T)>>(iter: I) -> Self {
        let iter = iter.into_iter();
        let size_hint = iter.size_hint().0;

        let mut heap = BinaryHeapIndexKeys::with_capacity(size_hint);

        for (key, value) in iter {
            heap.data.push((key.clone(), value));
            let existing = heap.keys.insert(key, heap.data.len() - 1);

            #[cfg(debug_assertions)]
            if let Some(existing_key) = existing {
                debug_assert!(
                    false,
                    "Tried to insert the same key multiple times: {}",
                    existing_key
                );
            }
        }

        heap.rebuild();
        heap
    }
}

impl<K: IdentityHashable, T, C> IntoIterator for BinaryHeapIndexKeys<K, T, C> {
    type Item = (K, T);
    type IntoIter = IntoIter<K, T>;

    /// Creates a consuming iterator, that is, one that moves each key-value pair
    /// out of the binary heap in arbitrary order. The binary heap cannot be used
    /// after calling this.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use mut_binary_heap_index_keys::BinaryHeapIndexKeys;
    /// let heap = BinaryHeapIndexKeys::<_,_>::from([1, 2, 3, 4].iter(), |v| *v.clone());
    ///
    /// // Print 1, 2, 3, 4 in arbitrary order
    /// for x in heap.into_iter() {
    ///     // x has type (i32, i32), not (&i32, &i32)
    ///     println!("key {}, value {}", x.0, x.1);
    /// }
    /// ```
    fn into_iter(self) -> IntoIter<K, T> {
        IntoIter {
            iter: self.data.into_iter(),
        }
    }
}

// TODO implement Debug for Iterator types
// TODO implement FusedIterator for Iterator types

/// An owning iterator over the elements of a `BinaryHeap`.
///
/// This `struct` is created by [`BinaryHeapIndexKeys::into_iter()`]
/// (provided by the [`IntoIterator`] trait). See its documentation for more.
///
/// [`IntoIterator`]: https://doc.rust-lang.org/stable/core/iter/trait.IntoIterator.html
// #[stable(feature = "rust1", since = "1.0.0")]
#[derive(Clone)]
pub struct IntoIter<K, T> {
    iter: vec::IntoIter<(K, T)>,
}

impl<K, T> Iterator for IntoIter<K, T> {
    type Item = (K, T);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

#[derive(Clone)]
pub struct IntoValues<K, V> {
    iter: vec::IntoIter<(K, V)>,
}

impl<K, V> Iterator for IntoValues<K, V> {
    type Item = V;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|kv| kv.1)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

#[derive(Clone)]
pub struct IntoKeys<K, V> {
    iter: vec::IntoIter<(K, V)>,
}

impl<K, V> Iterator for IntoKeys<K, V> {
    type Item = K;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|kv| kv.0)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

#[derive(Clone)]
pub struct Iter<'a, K, T> {
    iter: std::slice::Iter<'a, (K, T)>,
}

impl<'a, K, T> Iterator for Iter<'a, K, T> {
    type Item = (&'a K, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|kv| (&kv.0, &kv.1))
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }

    #[inline]
    fn last(self) -> Option<Self::Item>
    where
        Self: Sized,
    {
        self.iter.last().map(|kv| (&kv.0, &kv.1))
    }
}

impl<'a, K, T> DoubleEndedIterator for Iter<'a, K, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back().map(|kv| (&kv.0, &kv.1))
    }
}

#[derive(Clone)]
pub struct IterValues<'a, K, T> {
    iter: std::slice::Iter<'a, (K, T)>,
}

impl<'a, K, T> Iterator for IterValues<'a, K, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|kv| &kv.1)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }

    #[inline]
    fn last(self) -> Option<Self::Item>
    where
        Self: Sized,
    {
        self.iter.last().map(|kv| (&kv.1))
    }
}

#[derive(Clone)]
pub struct IterKeys<'a, K, T> {
    iter: std::slice::Iter<'a, (K, T)>,
}

impl<'a, K, T> Iterator for IterKeys<'a, K, T> {
    type Item = &'a K;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|kv| &kv.0)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }

    #[inline]
    fn last(self) -> Option<Self::Item>
    where
        Self: Sized,
    {
        self.iter.last().map(|kv| (&kv.0))
    }
}

impl<'a, K: IdentityHashable, T, C> IntoIterator for &'a BinaryHeapIndexKeys<K, T, C> {
    type Item = (&'a K, &'a T);
    type IntoIter = Iter<'a, K, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

/// An Iterator that yields mutable references to the values in the heap.
/// The heap will be rebuild after the iterator is droped.
// NOTE: this can not implement Clone or we invalidate the mutability guarantee.
pub struct MutIter<'a, K: Hash + Eq + IdentityHashable, T, C: Compare<T>> {
    heap: *mut BinaryHeapIndexKeys<K, T, C>,
    iter: std::slice::Iter<'a, (K, T)>,
}

impl<'a, K: Hash + Eq + IdentityHashable, T, C: Compare<T>> IntoIterator for &'a mut BinaryHeapIndexKeys<K, T, C> {
    type Item = (&'a K, &'a mut T);
    type IntoIter = MutIter<'a, K, T, C>;

    fn into_iter(self) -> Self::IntoIter {
        MutIter {
            heap: self,
            iter: self.data.iter(),
        }
    }
}

impl<'a, K: Hash + Eq + IdentityHashable, T, C: Compare<T>> Iterator for MutIter<'a, K, T, C> {
    type Item = (&'a K, &'a mut T);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(kv) = self.iter.next() {
            let key = &kv.0;
            let ptr: *const T = &kv.1 as *const T;
            let mut_ptr: *mut T = ptr as *mut T;
            // SAFTEY: We have mut access to the heap, because we are in a
            //  MutIter which can only be constructed with a mut ref to the
            //  heap.
            //
            //  We only give out a mut ref once per element in the heap, so this
            //  reference has not been shared so it's unique.
            #[allow(invalid_reference_casting)]
            let value = unsafe { &mut *mut_ptr };
            Some((key, value))
        } else {
            None
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<'a, K: Hash + Eq + IdentityHashable, T, C: Compare<T>> Drop for MutIter<'a, K, T, C> {
    fn drop(&mut self) {
        // SAFETY: MutIter was constructed from a valid mut reference
        let heap = unsafe { &mut *self.heap };
        heap.rebuild();
    }
}

// #[stable(feature = "rust1", since = "1.0.0")]
// TODO heap extension helpers
// impl<K, T, C: Compare<T>> Extend<T> for BinaryHeapIndexKeys<K, T, C> {
//     #[inline]
//     fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
//         // <Self as SpecExtend<I>>::spec_extend(self, iter);
//         self.extend_desugared(iter);
//     }
// }

// impl<K, T, I: IntoIterator<Item = T>> SpecExtend<I> for BinaryHeapIndexKeys<K, T> {
//     default fn spec_extend(&mut self, iter: I) {
//         self.extend_desugared(iter.into_iter());
//     }
// }

// impl<K, T> SpecExtend<BinaryHeapIndexKeys<K, T>> for BinaryHeapIndexKeys<K, T> {
//     fn spec_extend(&mut self, ref mut other: BinaryHeapIndexKeys<K, T>) {
//         self.append(other);
//     }
// }

// impl<K, T, C: Compare<T>> BinaryHeapIndexKeys<K, T, C> {
//     fn extend_desugared<I: IntoIterator<Item = T>>(&mut self, iter: I) {
//         let iterator = iter.into_iter();
//         let (lower, _) = iterator.size_hint();

//         self.reserve(lower);

//         iterator.for_each(move |elem| self.push(elem));
//     }
// }

// // #[stable(feature = "extend_ref", since = "1.2.0")]
// impl<'a, K, T: 'a + Copy, C: Compare<T>> Extend<&'a T> for BinaryHeapIndexKeys<K, T, C> {
//     fn extend<I: IntoIterator<Item = &'a T>>(&mut self, iter: I) {
//         self.extend(iter.into_iter().cloned());
//     }
// }

// #[unstable(feature = "collection_placement",
//            reason = "placement protocol is subject to change",
//            issue = "30172")]
// pub struct BinaryHeapPlace<'a, T: 'a>
// where T: Clone {
//     heap: *mut BinaryHeapIndexKeys<K, T>,
//     place: vec::PlaceBack<'a, T>,
// }

// #[unstable(feature = "collection_placement",
//            reason = "placement protocol is subject to change",
//            issue = "30172")]
// impl<'a, T: Clone + Ord + fmt::Debug> fmt::Debug for BinaryHeapPlace<'a, T> {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         f.debug_tuple("BinaryHeapPlace")
//          .field(&self.place)
//          .finish()
//     }
// }

// #[unstable(feature = "collection_placement",
//            reason = "placement protocol is subject to change",
//            issue = "30172")]
// impl<'a, T: 'a> Placer<T> for &'a mut BinaryHeapIndexKeys<K, T>
// where T: Clone + Ord {
//     type Place = BinaryHeapPlace<'a, T>;

//     fn make_place(self) -> Self::Place {
//         let ptr = self as *mut BinaryHeapIndexKeys<K, T>;
//         let place = Placer::make_place(self.data.place_back());
//         BinaryHeapPlace {
//             heap: ptr,
//             place,
//         }
//     }
// }

// #[unstable(feature = "collection_placement",
//            reason = "placement protocol is subject to change",
//            issue = "30172")]
// unsafe impl<'a, T> Place<T> for BinaryHeapPlace<'a, T>
// where T: Clone + Ord {
//     fn pointer(&mut self) -> *mut T {
//         self.place.pointer()
//     }
// }

// #[unstable(feature = "collection_placement",
//            reason = "placement protocol is subject to change",
//            issue = "30172")]
// impl<'a, T> InPlace<T> for BinaryHeapPlace<'a, T>
// where T: Clone + Ord {
//     type Owner = &'a T;

//     unsafe fn finalize(self) -> &'a T {
//         self.place.finalize();

//         let heap: &mut BinaryHeapIndexKeys<K, T> = &mut *self.heap;
//         let len = heap.len();
//         let i = heap.sift_up(0, len - 1);
//         heap.data.get_unchecked(i)
//     }
// }

#[cfg(test)]
mod test {
    use crate::BinaryHeapIndexKeys;
    use std::collections::HashMap;
    use std::hash::Hash;
    use identity_hash::IdentityHashable;

    fn is_normal<T: Send + Unpin>() {}

    #[test]
    fn check_is_send_unpin() {
        is_normal::<BinaryHeapIndexKeys<i64, i64>>();
        assert!(true);
    }

    fn assert_key_map_valid<K: Hash + Eq + Clone + IdentityHashable, T, C>(bh: &BinaryHeapIndexKeys<K, T, C>) {
        let mut expected_keys = HashMap::new();
        for (i, kv) in bh.data.iter().enumerate() {
            expected_keys.insert(kv.0.clone(), i);
        }

        for key_index in &expected_keys {
            let key = &key_index.0;
            let index = *key_index.1;
            assert!(bh.keys.contains_key(&key));
            assert_eq!(bh.keys[key], index);
        }
        assert_eq!(bh.keys.len(), expected_keys.len());
    }

    #[test]
    fn valid_key_map() {
        // TODO why do I need to specify the type here? The compiler should be able to infer this
        let mut heap: BinaryHeapIndexKeys<_, _> = BinaryHeapIndexKeys::new();

        assert_key_map_valid(&heap);

        heap.push(0, 0);

        assert_key_map_valid(&heap);

        heap.push(1, 10);
        heap.push(2, 15);
        heap.push(3, 5);
        heap.push(4, 8);

        assert_key_map_valid(&heap);

        assert_eq!(heap.pop_with_key(), Some((2, 15)));

        assert_key_map_valid(&heap);

        assert_eq!(heap.pop_with_key(), Some((1, 10)));
        assert_eq!(heap.pop_with_key(), Some((4, 8)));

        heap.push(5, 2);

        assert_key_map_valid(&heap);

        assert_eq!(heap.pop_with_key(), Some((3, 5)));
        assert_eq!(heap.pop_with_key(), Some((5, 2)));
        assert_eq!(heap.pop_with_key(), Some((0, 0)));

        assert_key_map_valid(&heap);

        assert_eq!(heap.pop_with_key(), None);

        assert_key_map_valid(&heap);
    }

    #[test]
    fn valid_key_map_after_clear() {
        // TODO why do I need to specify the type here? The compiler should be able to infer this
        let mut heap: BinaryHeapIndexKeys<_, _> = BinaryHeapIndexKeys::new();

        assert_key_map_valid(&heap);

        heap.push(0, 0);

        assert_key_map_valid(&heap);

        heap.push(1, 10);
        heap.push(2, 15);
        heap.push(3, 5);
        heap.push(4, 8);

        assert_key_map_valid(&heap);

        heap.clear();

        assert_key_map_valid(&heap);
        assert_eq!(heap.len(), 0);
    }
}
