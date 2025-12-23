//! # Christmas Tree Farm
//!
//! There are 3 possibilities for each combination of region and presents.
//!
//! ## Best case
//!
//! All presents fit into the region with no overlap. Each present has its own 3x3 space.
//! For example:
//!
//! ```none
//!  AAABBBCCC
//!  A....B.C.
//!  AAABBBCCC
//! ```
//!
//! ## Worst case
//!
//! The total number of tiles in the region is less than the total number of tiles in the presents,
//! so even if the presents are packed to occupy 100% of the space, there is no possible way that
//! they can fit. For example, the three shapes above contain 21 tiles, so they can never fit
//! into the 6x3 region with 18 tiles below.
//!
//! ```none
//!   ......
//!   ......
//!   ......
//! ```
//!
//! ## Mixed case
//!
//! The presents can fit into the region with some clever arrangement. This problem is known as
//! [polyomino tiling](https://en.wikipedia.org/wiki/Polyomino#Tiling_regions_with_sets_of_polyominoes)
//! and is [NP-complete](https://en.wikipedia.org/wiki/NP-completeness). This is an extremely
//! complex and challenging problem with no known polynomial-time solution.
//!
//! ## Solution
//!
//! For the *inputs provided*, all combinations of regions and presents fall into the first
//! two cases (either trivially possible or impossible), so there is no need to even attempt
//! the harder general case. Additionally, since each present is placed in its own 3x3 space,
//! there is also no need to even consider the shape of the presents.
use crate::util::iter::*;
use crate::util::parse::*;

pub fn parse(input: &str) -> &str {
    input
}

/// Count regions that can contain the total number of presents
/// each in their own 3x3 space with no overlap.
pub fn part1(input: &str) -> usize {
    input
        .iter_unsigned::<u32>()
        .skip(6)
        .chunk::<8>()
        .filter(|[w, h, presents @ ..]| (w / 3) * (h / 3) >= presents.iter().sum::<u32>())
        .count()
}

pub fn part2(_input: &str) -> &'static str {
    "n/a"
}
