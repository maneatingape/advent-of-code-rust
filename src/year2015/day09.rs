//! # All in a Single Night
//!
//! This is a variant of the classic NP-hard
//! [Travelling Salesman Problem](https://en.wikipedia.org/wiki/Travelling_salesman_problem).
//!
//! There are 8 locations, so naively it would require checking 8! = 40,320 permutations. We can
//! reduce this to 7!/2 = 2,520 permutations by arbitrarily choosing one of the locations as the
//! start, and skipping lexically reversed permutations (since the path a->b->c has the same
//! length as c->b->a).
//!
//! We then compute the distance to complete the trip and return to the original location.
//! Since the problem does not ask us to end up in the same location we then "break" the cycle.
//! To compute the shortest journey we remove the longest single journey and to compute the
//! longest journey we remove the shortest single journey.
//!
//! For speed we first convert each location into an index, then store the distances between
//! every pair of locations in an array for fast lookup. Our utility [`half_permutations`] method uses
//! [Steinhaus-Johnson-Trotter's algorithm](https://en.wikipedia.org/wiki/Steinhaus%E2%80%93Johnson%E2%80%93Trotter_algorithm) for efficiency,
//! modifying the slice in place.
//!
//! [`half_permutations`]: crate::util::slice
use crate::util::hash::*;
use crate::util::iter::*;
use crate::util::parse::*;
use crate::util::slice::*;

type Result = (u32, u32);

pub fn parse(input: &str) -> Result {
    let tokens: Vec<_> = input.split_ascii_whitespace().chunk::<5>().collect();
    let mut indices = FastMap::new();

    for [start, _, end, ..] in &tokens {
        let size = indices.len();
        indices.entry(start).or_insert(size);

        let size = indices.len();
        indices.entry(end).or_insert(size);
    }

    let stride = indices.len();
    let mut distances = vec![0; stride * stride];

    for [start, _, end, _, distance] in &tokens {
        let start = indices[start];
        let end = indices[end];
        let distance = distance.unsigned();

        distances[stride * start + end] = distance;
        distances[stride * end + start] = distance;
    }

    let mut global_min = u32::MAX;
    let mut global_max = u32::MIN;
    let mut indices: Vec<_> = (1..stride).collect();

    indices.half_permutations(|slice| {
        let mut sum = 0;
        let mut local_min = u32::MAX;
        let mut local_max = u32::MIN;

        let mut trip = |from, to| {
            let distance = distances[stride * from + to];
            sum += distance;
            local_min = local_min.min(distance);
            local_max = local_max.max(distance);
        };

        // First trip.
        trip(0, slice[0]);
        // Last trip.
        trip(0, slice[slice.len() - 1]);
        // Intermediate trips.
        for i in 1..slice.len() {
            trip(slice[i], slice[i - 1]);
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
