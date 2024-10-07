#![allow(clippy::needless_doctest_main)]
//! This crate provides [`BinaryHeap`] that stores key-value pairs.
//! The main advantage of that is that unlike with an implementation like
//! [`std::collections::BinaryHeap`] checking if any given key exist is `O(1)` instead of `O(n)`.
//! Same for getting the value for a given key. This allows for cheap modification of
//! values within the binary heap. Updating a value is `O(log n)` iff you have direct access to the value.
//! For a binary heap that does not store key-value pairs update operations would be `O(n)` because
//! they first have to find the value to update. The disadvantage is the additional storage space
//! required to store a HashMap that provides indices into the heap for each key.
//!
//! # Quick start
//!
//! ## Max/Min Heap
//!
//! ### Max Heap
//!
//! ```rust
//! use mut_binary_heap_index_keys::*;
//!
//! // max heap
//! let mut h: BinaryHeapIndexKeys<i32, i32> = BinaryHeapIndexKeys::new();
//! // max heap with initial capacity
//! let mut h: BinaryHeapIndexKeys<i32, i32> = BinaryHeapIndexKeys::with_capacity(42);
//! // max heap from iterator and key selector
//! let mut h: BinaryHeapIndexKeys<i32, i32> = BinaryHeapIndexKeys::from((0..42), |v| *v);
//! assert_eq!(h.pop(), Some(41));
//! ```
//!
//! ### Min Heap
//!
//! ```rust
//! use mut_binary_heap_index_keys::*;
//!
//! // min heap
//! let mut h: BinaryHeapIndexKeys<i32, i32, MinComparator> = BinaryHeapIndexKeys::new();
//! // min heap with initial capacity
//! let mut h: BinaryHeapIndexKeys<i32, i32, MinComparator> = BinaryHeapIndexKeys::with_capacity(42);
//! // min heap from iterator
//! let mut h: BinaryHeapIndexKeys<i32, i32, MinComparator> = BinaryHeapIndexKeys::from((0..42), |v| *v);
//! assert_eq!(h.pop(), Some(0));
//! ```
//!
//! [`BinaryHeapIndexKeys::from_vec()`]: struct.BinaryHeap.html#method.from_vec
//!
//! ## Custom Heap
//!
//! For custom heap, [`BinaryHeapIndexKeys::new_by()`] and [`BinaryHeapIndexKeys::new_by_sort_key`]
//! works in a similar way to max/min heap. The only difference is that you add
//! a closure returning a [`std::cmp::Ordering`] or the sort key with an apropriate signature.
//!
//! ```rust
//! use mut_binary_heap_index_keys::BinaryHeapIndexKeys;
//!
//! let mut heap = BinaryHeapIndexKeys::new_by_sort_key(|a: &i32| a % 4);
//! heap.push(0, 3);
//! heap.push(1, 1);
//! heap.push(2, 5);
//! assert_eq!(heap.pop(), Some(3));
//! ```
//!
//! # Constructers
//!
//! ## Dedicated methods to create different kind of heaps
//!
//! * [`BinaryHeapIndexKeys::new()`] creates a max heap.
//! * [`BinaryHeapIndexKeys::new_min()`] creates a min heap.
//! * [`BinaryHeapIndexKeys::new_by()`] creates a heap sorted by the given closure.
//! * [`BinaryHeapIndexKeys::new_by_sort_key()`] creates a heap sorted by the key generated by the given closure.
//! * [`BinaryHeapIndexKeys::from()`] creates a max heap with the elements in the iterator and keys provided by the closure.
// TODO create BinaryHeapIndexKeys::from for min and custom heaps
//!
//! # Examples
//!
//! This is a larger example that implements [Dijkstra's algorithm][dijkstra]
//! to solve the [shortest path problem][sssp] on a [directed graph][dir_graph].
//! It shows how to use [`BinaryHeap`] with custom types.
//!
//! [dijkstra]: https://en.wikipedia.org/wiki/Dijkstra%27s_algorithm
//! [sssp]: https://en.wikipedia.org/wiki/Shortest_path_problem
//! [dir_graph]: https://en.wikipedia.org/wiki/Directed_graph
//!
//! ```rust
//! use mut_binary_heap_index_keys::BinaryHeapIndexKeys;
//! use std::cmp::Ordering;
//!
//! #[derive(Copy, Clone, Eq, PartialEq)]
//! struct Node {
//!     cost: usize,
//!     position: usize,
//! }
//!
//! // The priority queue depends on `Ord`.
//! // Explicitly implement the trait so the queue becomes a min-heap
//! // instead of a max-heap.
//! impl Ord for Node {
//!     fn cmp(&self, other: &Self) -> Ordering {
//!         // Notice that the we flip the ordering on costs.
//!         // In case of a tie we compare positions - this step is necessary
//!         // to make implementations of `PartialEq` and `Ord` consistent.
//!         other
//!             .cost
//!             .cmp(&self.cost)
//!             .then_with(|| self.position.cmp(&other.position))
//!     }
//! }
//!
//! // `PartialOrd` needs to be implemented as well.
//! impl PartialOrd for Node {
//!     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//!         Some(self.cmp(other))
//!     }
//! }
//!
//! // Each node is represented as a `usize`, for a shorter implementation.
//! struct Edge {
//!     node: usize,
//!     cost: usize,
//! }
//!
//! // Dijkstra's shortest path algorithm.
//!
//! // Start at `start` and use `dist` to track the current shortest distance
//! // to each node.
//! fn shortest_path(edges: &Vec<Vec<Edge>>, start: usize, goal: usize) -> Option<usize> {
//!     let mut heap: BinaryHeapIndexKeys<usize, Node> = BinaryHeapIndexKeys::new();
//!     heap.push(
//!         start,
//!         Node {
//!             cost: 0,
//!             position: start,
//!         },
//!     );
//!
//!     while let Some(Node { cost, position }) = heap.pop() {
//!         if position == goal {
//!             return Some(cost);
//!         }
//!
//!         for edge in &edges[position] {
//!             let next_cost = cost + edge.cost;
//!
//!             // if the edge points to a node that is already in the heap, check
//!             // if it's cost is greater than the cost via this edge.
//!             // Note that normally dijkstra would also have a closed list with all
//!             // nodes that we have already visited. That closed list is also used to
//!             // keep track of the path we have taken.
//!             // To simplify this example we ignore that and only calculate the cost
//!             // to the goal.
// FIXME why can't i use let Some(node) = heap.pop(). rust complains about the borrow persisting into the else branch
//!             if heap.contains_key(&edge.node) {
//!                 let mut node = heap.get_mut(&edge.node).unwrap();
//!                 assert_eq!(node.position, edge.node);
//!                 if next_cost < node.cost {
//!                     node.cost = next_cost;
//!                 }
//!                 // by dropping `node` the heap is autmatically updated.
//!             } else {
//!                 heap.push(
//!                     edge.node,
//!                     Node {
//!                         cost: next_cost,
//!                         position: edge.node,
//!                     },
//!                 );
//!             }
//!         }
//!     }
//!     // If the heap is empty, the goal wasn't found.
//!     None
//! }
//!
//! fn main() {
//!     // This is the directed graph we're going to use.
//!     // The node numbers correspond to the different states,
//!     // and the edge weights symbolize the cost of moving
//!     // from one node to another.
//!     // Note that the edges are one-way.
//!     //
//!     //                  7
//!     //          +-----------------+
//!     //          |                 |
//!     //          v   1        2    |  2
//!     //          0 -----> 1 -----> 3 ---> 4
//!     //          |        ^        ^      ^
//!     //          |        | 1      |      |
//!     //          |        |        | 3    | 1
//!     //          +------> 2 -------+      |
//!     //           10      |               |
//!     //                   +---------------+
//!     //
//!     // The graph is represented as an adjacency list where each index,
//!     // corresponding to a node value, has a list of outgoing edges.
//!     // Chosen for its efficiency.
//!     let graph = vec![
//!         // Node 0
//!         vec![Edge { node: 2, cost: 10 }, Edge { node: 1, cost: 1 }],
//!         // Node 1
//!         vec![Edge { node: 3, cost: 2 }],
//!         // Node 2
//!         vec![
//!             Edge { node: 1, cost: 1 },
//!             Edge { node: 3, cost: 3 },
//!             Edge { node: 4, cost: 1 },
//!         ],
//!         // Node 3
//!         vec![Edge { node: 0, cost: 7 }, Edge { node: 4, cost: 2 }],
//!         // Node 4
//!         vec![],
//!     ];
//!
//!     assert_eq!(shortest_path(&graph, 0, 1), Some(1));
//!     assert_eq!(shortest_path(&graph, 0, 3), Some(3));
//!     assert_eq!(shortest_path(&graph, 3, 0), Some(7));
//!     assert_eq!(shortest_path(&graph, 0, 4), Some(5));
//!     assert_eq!(shortest_path(&graph, 4, 0), None);
//! }
//! ```

mod binary_heap;
pub use crate::binary_heap::*;

/// An intermediate trait for specialization of `Extend`.
// #[doc(hidden)]
// trait SpecExtend<I: IntoIterator> {
//     /// Extends `self` with the contents of the given iterator.
//     fn spec_extend(&mut self, iter: I);
// }

#[cfg(test)]
mod from_liballoc {
    // The following tests copyed from liballoc/tests/binary_heap.rs
    // I can't fully confirm what the original authors meant by liballoc.
    // However this is extremely similar to:
    // https://github.com/rust-lang/rust/blob/master/library/alloc/src/collections/binary_heap/tests.rs
    // TODO port tests that we are missing and mark commit hash for future reference

    use super::binary_heap::*;

    #[test]
    fn test_iterator() {
        let data = vec![5, 9, 3];
        let iterout = [9, 5, 3];
        let heap = BinaryHeapIndexKeys::<_, _>::from(data, |k| k.clone());
        let mut i = 0;
        for el in &heap {
            assert_eq!(*el.1, iterout[i]);
            i += 1;
        }
    }

    // #[test]
    // fn test_iterator_reverse() {
    //     let data = vec![5, 9, 3];
    //     let iterout = vec![3, 5, 9];
    //     let pq = BinaryHeapIndexKeys::<_, _>::from(data, |k| k.clone());

    //     let v: Vec<_> = pq.iter().rev().cloned().collect();
    //     assert_eq!(v, iterout);
    // }

    // #[test]
    // fn test_move_iter() {
    //     let data = vec![5, 9, 3];
    //     let iterout = vec![9, 5, 3];
    //     let pq = BinaryHeapIndexKeys::<_, _>::from(data, |k| k.clone());

    //     let v: Vec<_> = pq.into_iter().collect();
    //     assert_eq!(v, iterout);
    // }

    #[test]
    fn test_move_iter_size_hint() {
        let data = vec![5, 9];
        let pq = BinaryHeapIndexKeys::<_, _>::from(data, |k| k.clone());

        let mut it = pq.into_iter();

        assert_eq!(it.size_hint(), (2, Some(2)));
        assert_eq!(it.next(), Some((9, 9)));

        assert_eq!(it.size_hint(), (1, Some(1)));
        assert_eq!(it.next(), Some((5, 5)));

        assert_eq!(it.size_hint(), (0, Some(0)));
        assert_eq!(it.next(), None);
    }

    // #[test]
    // fn test_move_iter_reverse() {
    //     let data = vec![5, 9, 3];
    //     let iterout = vec![3, 5, 9];
    //     let pq = BinaryHeapIndexKeys::<_, _>::from(data, |k| k.clone());

    //     let v: Vec<_> = pq.into_iter().rev().collect();
    //     assert_eq!(v, iterout);
    // }

    // #[test]
    // fn test_into_iter_sorted_collect() {
    //     let heap = BinaryHeapIndexKeys::from(vec![2, 4, 6, 2, 1, 8, 10, 3, 5, 7, 0, 9, 1]);
    //     let it = heap.into_iter_sorted();
    //     let sorted = it.collect::<Vec<_>>();
    //     assert_eq!(sorted, vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 2, 1, 1, 0]);
    // }

    #[test]
    fn test_peek_and_pop() {
        let data = vec![2, 4, 6, 2, 1, 8, 10, 3, 5, 7, 0, 9, 1];
        let mut sorted = data.clone();
        sorted.sort();
        let data = data.into_iter().enumerate().map(|(i, v)| (i, v));
        let mut heap: BinaryHeapIndexKeys<_, _> = data.collect();
        while !heap.is_empty() {
            assert_eq!(heap.peek().unwrap(), sorted.last().unwrap());
            assert_eq!(heap.pop().unwrap(), sorted.pop().unwrap());
        }
    }

    #[test]
    fn test_peek_mut() {
        let data = [2, 4, 6, 2, 1, 8, 10, 3, 5, 7, 0, 9, 1]
            .into_iter()
            .enumerate()
            .map(|(i, v)| (i, v));
        let mut heap: BinaryHeapIndexKeys<_, _> = data.collect();
        assert_eq!(heap.peek(), Some(&10));
        {
            let mut top = heap.peek_mut().unwrap();
            *top -= 2;
        }
        assert_eq!(heap.peek(), Some(&9));
    }

    #[test]
    fn test_peek_mut_pop() {
        let data = [2, 4, 6, 2, 1, 8, 10, 3, 5, 7, 0, 9, 1]
            .into_iter()
            .enumerate()
            .map(|(i, v)| (i, v));
        let mut heap: BinaryHeapIndexKeys<_, _> = data.collect();
        assert_eq!(heap.peek(), Some(&10));
        {
            let mut top = heap.peek_mut().unwrap();
            *top -= 2;
            assert_eq!(PeekMut::pop(top), 8);
        }
        assert_eq!(heap.peek(), Some(&9));
    }

    #[test]
    fn test_push() {
        let mut heap = BinaryHeapIndexKeys::<_, _>::from(vec![2, 4, 9], |k| k.clone());
        assert_eq!(heap.len(), 3);
        assert!(*heap.peek().unwrap() == 9);
        heap.push(11, 11);
        assert_eq!(heap.len(), 4);
        assert!(*heap.peek().unwrap() == 11);
        heap.push(5, 5);
        assert_eq!(heap.len(), 5);
        assert!(*heap.peek().unwrap() == 11);
        heap.push(27, 27);
        assert_eq!(heap.len(), 6);
        assert!(*heap.peek().unwrap() == 27);
        heap.push(3, 3);
        assert_eq!(heap.len(), 7);
        assert!(*heap.peek().unwrap() == 27);
        heap.push(103, 103);
        assert_eq!(heap.len(), 8);
        assert!(*heap.peek().unwrap() == 103);
    }

    #[test]
    fn test_push_unique() {
        let data: Vec<Box<i32>> = [2, 4, 9].iter().map(|v| Box::new(*v)).collect();
        let mut heap = BinaryHeapIndexKeys::<i32, Box<i32>>::from(data, |k| **k);
        assert_eq!(heap.len(), 3);
        assert!(**heap.peek().unwrap() == 9);
        heap.push(11, Box::new(11));
        assert_eq!(heap.len(), 4);
        assert!(**heap.peek().unwrap() == 11);
        heap.push(5, Box::new(5));
        assert_eq!(heap.len(), 5);
        assert!(**heap.peek().unwrap() == 11);
        heap.push(27, Box::new(27));
        assert_eq!(heap.len(), 6);
        assert!(**heap.peek().unwrap() == 27);
        heap.push(3, Box::new(3));
        assert_eq!(heap.len(), 7);
        assert!(**heap.peek().unwrap() == 27);
        heap.push(103, Box::new(103));
        assert_eq!(heap.len(), 8);
        assert!(**heap.peek().unwrap() == 103);
    }

    // fn check_to_vec(mut data: Vec<i32>) {
    //     let heap = BinaryHeapIndexKeys::from(data.clone());
    //     let mut v = heap.clone().into_vec();
    //     v.sort();
    //     data.sort();

    //     assert_eq!(v, data);
    //     assert_eq!(heap.into_sorted_vec(), data);
    // }

    #[test]
    fn test_empty_pop() {
        let mut heap = BinaryHeapIndexKeys::<i32, i32>::new();
        assert!(heap.pop().is_none());
    }

    #[test]
    fn test_empty_peek() {
        let empty = BinaryHeapIndexKeys::<i32, i32>::new();
        assert!(empty.peek().is_none());
    }

    #[test]
    fn test_empty_peek_mut() {
        let mut empty = BinaryHeapIndexKeys::<i32, i32>::new();
        assert!(empty.peek_mut().is_none());
    }

    // #[test]
    // fn test_from_iter() {
    //     let xs = vec![9, 8, 7, 6, 5, 4, 3, 2, 1];

    //     let mut q: BinaryHeapIndexKeys<_> = xs.iter().rev().cloned().collect();

    //     for &x in &xs {
    //         assert_eq!(q.pop().unwrap(), x);
    //     }
    // }

    // #[test]
    // fn test_drain() {
    //     let mut q: BinaryHeapIndexKeys<_> = [9, 8, 7, 6, 5, 4, 3, 2, 1].iter().cloned().collect();

    //     assert_eq!(q.drain().take(5).count(), 5);

    //     assert!(q.is_empty());
    // }

    // #[test]
    // fn test_extend_ref() {
    //     let mut a = BinaryHeapIndexKeys::new();
    //     a.push(1);
    //     a.push(2);

    //     a.extend(&[3, 4, 5]);

    //     assert_eq!(a.len(), 5);
    //     assert_eq!(a.into_sorted_vec(), [1, 2, 3, 4, 5]);

    //     let mut a = BinaryHeapIndexKeys::new();
    //     a.push(1);
    //     a.push(2);
    //     let mut b = BinaryHeapIndexKeys::new();
    //     b.push(3);
    //     b.push(4);
    //     b.push(5);

    //     a.extend(&b);

    //     assert_eq!(a.len(), 5);
    //     assert_eq!(a.into_sorted_vec(), [1, 2, 3, 4, 5]);
    // }

    // #[test]
    // fn test_append() {
    //     let mut a = BinaryHeapIndexKeys::from(vec![-10, 1, 2, 3, 3]);
    //     let mut b = BinaryHeapIndexKeys::from(vec![-20, 5, 43]);

    //     a.append(&mut b);

    //     assert_eq!(a.into_sorted_vec(), [-20, -10, 1, 2, 3, 3, 5, 43]);
    //     assert!(b.is_empty());
    // }

    // #[test]
    // fn test_append_to_empty() {
    //     let mut a = BinaryHeapIndexKeys::new();
    //     let mut b = BinaryHeapIndexKeys::from(vec![-20, 5, 43]);

    //     a.append(&mut b);

    //     assert_eq!(a.into_sorted_vec(), [-20, 5, 43]);
    //     assert!(b.is_empty());
    // }

    // #[test]
    // fn test_extend_specialization() {
    //     let mut a = BinaryHeapIndexKeys::from(vec![-10, 1, 2, 3, 3]);
    //     let b = BinaryHeapIndexKeys::from(vec![-20, 5, 43]);

    //     a.extend(b);

    //     assert_eq!(a.into_sorted_vec(), [-20, -10, 1, 2, 3, 3, 5, 43]);
    // }

    // #[test]
    // fn test_placement() {
    //     let mut a = BinaryHeapIndexKeys::new();
    //     &mut a <- 2;
    //     &mut a <- 4;
    //     &mut a <- 3;
    //     assert_eq!(a.peek(), Some(&4));
    //     assert_eq!(a.len(), 3);
    //     &mut a <- 1;
    //     assert_eq!(a.into_sorted_vec(), vec![1, 2, 3, 4]);
    // }

    // #[test]
    // fn test_placement_panic() {
    //     let mut heap = BinaryHeapIndexKeys::from(vec![1, 2, 3]);
    //     fn mkpanic() -> usize {
    //         panic!()
    //     }
    //     let _ = panic::catch_unwind(panic::AssertUnwindSafe(|| {
    //         &mut heap <- mkpanic();
    //     }));
    //     assert_eq!(heap.len(), 3);
    // }

    #[allow(dead_code)]
    fn assert_covariance() {
        fn drain<'new>(d: Drain<'static, &'static str>) -> Drain<'new, &'new str> {
            d
        }
    }

    // old binaryheap failed this test
    //
    // Integrity means that all elements are present after a comparison panics,
    // even if the order might not be correct.
    //
    // Destructors must be called exactly once per element.
    // FIXME: re-enable emscripten once it can unwind again
    #[test]
    #[cfg(not(target_os = "emscripten"))]
    fn panic_safe() {
        use std::cmp;
        use std::panic::{self, AssertUnwindSafe};
        use std::sync::atomic::{AtomicUsize, Ordering};

        use rand::{seq::SliceRandom, thread_rng};

        static DROP_COUNTER: AtomicUsize = AtomicUsize::new(0);

        #[derive(Eq, PartialEq, PartialOrd, Clone, Debug)]
        struct PanicOrd<T>(T, bool);

        impl<T> Drop for PanicOrd<T> {
            fn drop(&mut self) {
                // update global drop count
                DROP_COUNTER.fetch_add(1, Ordering::SeqCst);
            }
        }

        impl<T: Ord> Ord for PanicOrd<T> {
            fn cmp(&self, other: &Self) -> cmp::Ordering {
                if self.1 || other.1 {
                    panic!("Panicking comparison");
                }
                self.0.cmp(&other.0)
            }
        }
        let mut rng = thread_rng();
        const DATASZ: usize = 32;
        // Miri is too slow
        let ntest = if cfg!(miri) { 1 } else { 10 };

        // don't use 0 in the data -- we want to catch the zeroed-out case.
        let data = (1..=DATASZ).collect::<Vec<_>>();

        // since it's a fuzzy test, run several tries.
        for _ in 0..ntest {
            for i in 1..=DATASZ {
                DROP_COUNTER.store(0, Ordering::SeqCst);

                let mut panic_ords: Vec<_> = data
                    .iter()
                    .filter(|&&x| x != i)
                    .map(|&x| PanicOrd(x, false))
                    .collect();
                let panic_item = PanicOrd(i, true);

                // heapify the sane items
                panic_ords.shuffle(&mut rng);
                let mut heap = BinaryHeapIndexKeys::<_, _>::from(panic_ords, |p| p.0);
                let inner_data: Vec<PanicOrd<usize>>;

                {
                    // push the panicking item to the heap and catch the panic
                    let thread_result = {
                        let mut heap_ref = AssertUnwindSafe(&mut heap);
                        panic::catch_unwind(move || {
                            heap_ref.push(panic_item.0, panic_item);
                        })
                    };
                    assert!(thread_result.is_err());

                    // Assert no elements were dropped
                    let drops = DROP_COUNTER.load(Ordering::SeqCst);
                    assert!(drops == 0, "Must not drop items. drops={}", drops);
                    inner_data = heap.clone().into_values().collect();
                    drop(heap);
                }
                let drops = DROP_COUNTER.load(Ordering::SeqCst);
                assert_eq!(drops, DATASZ);

                let mut data_sorted = inner_data.into_iter().map(|p| p.0).collect::<Vec<_>>();
                data_sorted.sort();
                assert_eq!(data_sorted, data);
            }
        }
    }
}

