//! # Camp Cleanup
//!
//! This puzzle asks to compute range intersections. To simplify the 2nd part, we can use a trick.
//! Rather than consider each possible case of intersection (overlapping start, overlapping end,
//! completely enclosed or completely enclosing) it's simpler to check if two ranges *don't* overlap
//! then invert.
//!
//! If `a` and `b` are the ordered start and end of the first range and `c` and `d` the ordered
//! start and end of the second range, then if:
//!
//! `a > d || c > b`
//!
//! the 2 ranges can't overlap. Using [DeMorgan's laws](https://en.wikipedia.org/wiki/De_Morgan%27s_laws)
//! this can be inverted to:
//!
//! `a <= d && c <= b`
//!
//! to check when two ranges do overlap.
use crate::util::iter::*;
use crate::util::parse::*;

type Pairs = [u32; 4];

/// Parse each line into 4 integers.
///
/// Notes:
/// * Extracting integers from redundant text is a very common theme in AoC that
///   the [`iter_unsigned`] method handles.
///
/// [`iter_unsigned`]: ParseOps::iter_unsigned
pub fn parse(input: &str) -> Vec<Pairs> {
    input.iter_unsigned().chunk::<4>().collect()
}

/// Count ranges completely enclosed by each other.
pub fn part1(input: &[Pairs]) -> usize {
    input.iter().filter(|[a, b, c, d]| (a >= c && b <= d) || (c >= a && d <= b)).count()
}

/// Count ranges with any intersection.
pub fn part2(input: &[Pairs]) -> usize {
    input.iter().filter(|[a, b, c, d]| a <= d && c <= b).count()
}
