//! # Grove Positioning System
//!
//! We store the numbers in an array of `vec`s. The initial size of each vector is 20
//! so that numbers are spread as evenly as possible.
//!
//! Using multiple leaf `vec`s greatly reduces the time to insert, remove and find
//! numbers, compared to storing all numbers in a single flat `vec`. Some further optimizations:
//! * The first and second level indices of a number change only when it moves, so these can be
//!   stored in a lookup array for fast access.
//! * The size of each first level `vec` is the sum of the second level `vec`s contained
//!   inside. This is stored in the `skip` array to prevent recomputing on each move.
//!
//! This implementation is both faster and simpler than the previous version (preserved in the
//! commit history) that used an [order statistic tree](https://en.wikipedia.org/wiki/Order_statistic_tree),
//! although perhaps adding [balancing rotations](https://en.wikipedia.org/wiki/Tree_rotation)
//! to the tree would make it faster.
//!
//! Leaf `vec`s are padded to a size modulo 64 to speed up searching for numbers. A SIMD variant
//! can search for 64 numbers simultaneously.
use crate::util::parse::*;
use std::array::from_fn;
use std::iter::repeat_n;

struct PaddedVec {
    size: usize,
    vec: Vec<u16>,
}

pub fn parse(input: &str) -> Vec<i64> {
    input.iter_signed().collect()
}

pub fn part1(input: &[i64]) -> i64 {
    decrypt(input, 1, 1)
}

pub fn part2(input: &[i64]) -> i64 {
    decrypt(input, 811589153, 10)
}

fn decrypt(input: &[i64], key: i64, rounds: usize) -> i64 {
    // Important nuance, size is one less because we don't consider the moving number.
    let size = input.len() - 1;
    // Another nuance, input contain duplicate numbers, so use index to refer to each number uniquely.
    let indices: Vec<_> = (0..input.len() as u16).collect();
    // Pre-process the numbers, converting any negative indices to positive indices that will wrap.
    // For example, -1 becomes 4998.
    let numbers: Vec<_> =
        input.iter().map(|&n| (n * key).rem_euclid(size as i64) as usize).collect();
    // Store location of each number within `mixed` for faster lookup.
    let mut lookup = Vec::with_capacity(input.len());
    // Size of each block of 16 elements for faster lookup.
    let mut skip = [0; 16];
    // Break 5000 numbers into roughly equals chunks.
    let mut mixed: [_; 256] = from_fn(|_| PaddedVec { size: 0, vec: Vec::with_capacity(128) });

    for (second, slice) in indices.chunks(input.len().div_ceil(256)).enumerate() {
        let size = slice.len();

        mixed[second].size = size;
        mixed[second].vec.resize(size.next_multiple_of(64), 0);
        mixed[second].vec[..size].copy_from_slice(slice);

        lookup.extend(repeat_n(second, size));
        skip[second / 16] += size;
    }

    for _ in 0..rounds {
        'mix: for index in 0..input.len() {
            // Quickly find the leaf vector storing the number.
            let number = numbers[index];
            let second = lookup[index];
            let first = second / 16;

            // Third level changes as other numbers are added and removed,
            // so needs to be checked each time.
            let third = position(&mixed[second], index as u16);

            // Find the offset of the number by adding the size of all previous `vec`s.
            let position = third
                + skip[..first].iter().sum::<usize>()
                + mixed[16 * first..second].iter().map(|v| v.size).sum::<usize>();
            // Update our position, wrapping around if necessary.
            let mut next = (position + number) % size;

            // Remove number from current leaf vector, also updating the first level size.
            mixed[second].size -= 1;
            mixed[second].vec.remove(third);
            mixed[second].vec.push(0);
            skip[first] -= 1;

            // Find our new destination, by checking `vec`s in order until the total elements
            // are greater than our new index.
            for (first, outer) in mixed.chunks_exact_mut(16).enumerate() {
                if next > skip[first] {
                    next -= skip[first];
                } else {
                    for (second, inner) in outer.iter_mut().enumerate() {
                        if next > inner.size {
                            next -= inner.size;
                        } else {
                            // Insert number into its new home.
                            inner.size += 1;
                            inner.vec.insert(next, index as u16);
                            inner.vec.resize(inner.size.next_multiple_of(64), 0);
                            // Update location.
                            skip[first] += 1;
                            lookup[index] = 16 * first + second;
                            continue 'mix;
                        }
                    }
                }
            }
        }
    }

    let indices: Vec<_> =
        mixed.into_iter().flat_map(|pv| pv.vec.into_iter().take(pv.size)).collect();
    let zeroth = indices.iter().position(|&i| input[i as usize] == 0).unwrap();

    [1000, 2000, 3000]
        .iter()
        .map(|offset| (zeroth + offset) % indices.len())
        .map(|index| input[indices[index] as usize] * key)
        .sum()
}

/// The compiler optimizes the position search when the size of the chunk is known.
#[cfg(not(feature = "simd"))]
#[inline]
fn position(haystack: &PaddedVec, needle: u16) -> usize {
    for (base, slice) in haystack.vec.chunks_exact(64).enumerate() {
        if let Some(offset) = slice.iter().position(|&i| i == needle) {
            return 64 * base + offset;
        }
    }

    unreachable!()
}

/// Search 64 lanes simultaneously.
#[cfg(feature = "simd")]
#[inline]
fn position(haystack: &PaddedVec, needle: u16) -> usize {
    use std::simd::cmp::SimdPartialEq as _;
    use std::simd::*;

    for (base, slice) in haystack.vec.chunks_exact(64).enumerate() {
        if let Some(offset) =
            Simd::<u16, 64>::from_slice(slice).simd_eq(Simd::splat(needle)).first_set()
        {
            return 64 * base + offset;
        }
    }

    unreachable!()
}
