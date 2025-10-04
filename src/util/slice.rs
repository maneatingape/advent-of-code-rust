//! Extension methods for slices.
//!
//! # Methods
//!
//! [`permutations`]
//!
//! Generates all possible permutations of a mutable slice, passing them one at a time to a
//! callback function.
//! Uses [Heap's algorithm](https://en.wikipedia.org/wiki/Heap%27s_algorithm) for efficiency,
//! modifying the slice in place.
//!
//! [`fold_decimal`]
//!
//! Accumulates a slice of digits from 0 to 9 inclusive into a single integer.
//!
//! [`permutations`]: SliceOps::permutations
//! [`fold_decimal`]: SliceOps2::fold_decimal
use super::integer::*;

pub trait SliceOps<T> {
    fn permutations(self, callback: impl FnMut(&[T]));
}

impl<T> SliceOps<T> for &mut [T] {
    fn permutations(self, mut callback: impl FnMut(&[T])) {
        callback(self);

        let n = self.len();
        let mut c = vec![0; n];
        let mut i = 1;

        while i < n {
            if c[i] < i {
                let swap_index = if i.is_multiple_of(2) { 0 } else { c[i] };
                self.swap(swap_index, i);
                callback(self);
                c[i] += 1;
                i = 1;
            } else {
                c[i] = 0;
                i += 1;
            }
        }
    }
}

pub trait SliceOps2<T: Integer<T>> {
    /// Folds a slice of digits into an integer.
    fn fold_decimal(self) -> T;
}

impl<T: Integer<T>> SliceOps2<T> for &[T] {
    #[inline]
    fn fold_decimal(self) -> T {
        self.iter().fold(T::ZERO, |acc, &b| T::TEN * acc + b)
    }
}
