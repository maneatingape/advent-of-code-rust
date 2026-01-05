//! # Spinlock
//!
//! For part one, instead of inserting elements into a Vec, which would have a worst case
//! complexity of O(n²), we use a lightweight [B+tree] implementation with only one internal node.
//! For 2017 elements, a maximum leaf size of 128 seems to be a sweet spot.
//!
//! There are two insights that speed up part two.
//!
//! The first is that we don't need a buffer. We only need to preserve the last value inserted
//! whenever the index becomes zero. Once 50 million values have been inserted then this value
//! is the final result.
//!
//! The second trick is realizing that we can insert multiple values at a time before the index
//! wraps around. For example if the index is 1, the current value 10,000 and the step 300,
//! then we can insert 34 values at once. The [`div_ceil`] method is perfect for this computation.
//!
//! This reduces the number of loops needed to approximately √50000000 = 7071.
//!
//! [`div_ceil`]: usize::div_ceil
//! [B+tree]: https://en.wikipedia.org/wiki/B%2B_tree
use crate::{util::parse::*, year2017::day17::btree::Btree};

mod btree {
    pub(super) struct Btree<T: Copy> {
        max_leaf_len: usize,
        buffers: Vec<Vec<T>>,
        len: usize,
    }

    impl<T: Copy> Btree<T> {
        /// Create a new B+tree with a maximum leaf size
        pub(super) fn new(max_leaf_len: usize) -> Self {
            Self { max_leaf_len, buffers: vec![vec![]], len: 0 }
        }

        /// Return the number of items in the tree
        pub(super) fn len(&self) -> usize {
            self.len
        }

        /// Insert an `element` into the tree at the given `index`
        pub(super) fn insert(&mut self, index: usize, element: T) {
            // find the leaf that covers this index
            let mut j = 0;
            for (k, b) in self.buffers.iter_mut().enumerate() {
                if index <= j + b.len() {
                    // insert the element into the leaf and update the tree's length
                    b.insert(index - j, element);
                    self.len += 1;

                    // if the leaf has become too large, split it into two
                    if b.len() > self.max_leaf_len {
                        let new_b = b.split_off(b.len() / 2);
                        self.buffers.insert(k + 1, new_b);
                    }

                    break;
                }
                j += b.len();
            }
        }

        /// Get the element at the given `index` or `None` if the tree does not contain this index
        pub(super) fn get(&self, index: usize) -> Option<T> {
            let mut j = 0;
            for b in &self.buffers {
                if index < j + b.len() {
                    return Some(b[index - j]);
                }
                j += b.len();
            }
            None
        }
    }
}

pub fn parse(input: &str) -> usize {
    input.unsigned()
}

pub fn part1(input: &usize) -> u16 {
    let step = input + 1;
    let mut index = 0;
    let mut buffer = Btree::new(128);
    buffer.insert(0, 0);

    for n in 0..2017 {
        index = (index + step) % buffer.len();
        buffer.insert(index, n + 1);
    }

    buffer.get((index + 1) % buffer.len()).unwrap()
}

pub fn part2(input: &usize) -> usize {
    let step = input + 1;
    let mut n: usize = 1;
    let mut index = 0;
    let mut result = 0;

    while n <= 50_000_000 {
        if index == 0 {
            result = n;
        }

        let skip = (n - index).div_ceil(step);
        n += skip;
        index = (index + skip * step) % n;
    }

    result
}
