//! # All in a Single Night
//!
//! This is a variant of the classic NP-hard
//! [Travelling Salesman Problem](https://en.wikipedia.org/wiki/Travelling_salesman_problem).
//!
//! There are 8 locations, so naively it would require checking 8! = 40,320 permutations. We can
//! reduce this to 7! = 5,040 permutations by arbitrarily choosing one of the locations as the
//! start.
//!
//! We then compute the distance to complete the trip and return to the original location.
//! Since the problem does not ask us to end up in the same location we then "break" the cycle.
//! To compute the shortest journey we remove the longest single journey and to compute the
//! longest journey we remove the shortest single journey.
//!
//! For speed we first convert each location into an index, then store the distances between
//! every pair of locations in an array for fast lookup. Our utility [`permutations`] method uses
//! [Heap's algorithm](https://en.wikipedia.org/wiki/Heap%27s_algorithm) for efficiency,
//! modifying the slice in place.
//!
//! [`permutations`]: crate::util::slice
use crate::util::hash::*;
use crate::util::iter::*;
use crate::util::parse::*;
use crate::util::slice::*;

type Result = (u32, u32);

pub fn parse(input: &str) -> Result {
    let tokens: Vec<_> = input.split_ascii_whitespace().chunk::<5>().collect();

    let mut indices = FastMapBuilder::empty();
    for [start, _, end, ..] in tokens.iter() {
        if !indices.contains_key(start) {
            indices.insert(start, indices.len());
        }
        if !indices.contains_key(end) {
            indices.insert(end, indices.len());
        }
    }

    let stride = indices.len();
    let mut distances = vec![0_u32; stride * stride];
    for [start, _, end, _, distance] in tokens.iter() {
        let start = indices[start];
        let end = indices[end];
        let distance = from(distance);
        distances[stride * start + end] = distance;
        distances[stride * end + start] = distance;
    }

    let mut global_min = u32::MAX;
    let mut global_max = u32::MIN;
    let mut middle: Vec<_> = (1..stride).collect();

    middle.permutations(|slice| {
        let first = distances[slice[0]];
        let last = distances[slice[stride - 2]];
        let mut sum = first + last;
        let mut local_min = first.min(last);
        let mut local_max = first.max(last);

        for w in slice.windows(2) {
            let trip = distances[stride * w[0] + w[1]];
            sum += trip;
            local_min = local_min.min(trip);
            local_max = local_max.max(trip);
        }

        global_min = global_min.min(sum - local_max);
        global_max = global_max.max(sum - local_min);
    });

    (global_min, global_max)
}

pub fn part1(input: &Result) -> u32 {
    input.0
}

pub fn part2(input: &Result) -> u32 {
    input.1
}
