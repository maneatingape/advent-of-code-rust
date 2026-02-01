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
//! [`half_permutations`]
//!
//! Like `permutations`, but skip any permutation which is lexically reversed from an earlier
//! callback.  Only half the permutations are visited in total.
//! Uses [Steinhaus-Johnson-Trotter's algorithm](https://en.wikipedia.org/wiki/Steinhaus%E2%80%93Johnson%E2%80%93Trotter_algorithm),
//! modifying the slice in place.
//!
//! [`permutations`]: SliceOps::permutations
//! [`half_permutations`]: SliceOps::half_permutations
pub trait SliceOps<T> {
    fn permutations(self, callback: impl FnMut(&[T]));
    fn half_permutations(self, callback: impl FnMut(&[T]));
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

    fn half_permutations(self, mut callback: impl FnMut(&[T])) {
        let n = self.len();
        // Compute n!/2, the number of iterations we need.
        let limit = (1..n + 1).product::<usize>() / 2;

        // Track how far each element has moved from its original position
        let mut pos = vec![0_usize; n];

        // Track which direction an element needs to move next (-1 or 1)
        let mut dir = vec![1_isize; n];

        for _ in 0..limit {
            callback(self);

            // Each iteration requires scanning up to n positions to locate a candidate to swap
            // with its neighbor, by moving left and right towards one another and updating pos and
            // dir along the way.
            let mut left = 0;
            let mut right = n - 1;

            loop {
                pos[right] = pos[right].wrapping_add_signed(dir[right]);
                if right + 1 == pos[right] {
                    dir[right] = -1;
                    if right > 1 {
                        right -= 1;
                    } else {
                        self.swap(left, left + 1);
                        break;
                    }
                } else if pos[right] == 0 {
                    dir[right] = 1;
                    left += 1;
                    if right > 1 {
                        right -= 1;
                    } else {
                        self.swap(left, left + 1);
                        break;
                    }
                } else {
                    let index = left + pos[right] - 1;
                    self.swap(index, index + 1);
                    break;
                }
            }
        }
    }
}
