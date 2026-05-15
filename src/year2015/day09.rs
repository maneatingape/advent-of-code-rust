//! # All in a Single Night
//!
//! This is a variant of the classic NP-hard [Travelling Salesman Problem].
//!
//! There are 8 locations, so naively it would require checking 8! = 40,320 permutations. We can
//! reduce this to 7!/2 = 2,520 permutations by arbitrarily choosing one of the locations as the
//! start, and skipping lexically reversed permutations (since the path a->b->c has the same
//! length as c->b->a). Computing the shortest and longest path is then done by completing the
//! cycle for each permutation, then discarding the longest or shortest edge seen along the way.
//! Skipping lexically reversed permutations is possible with
//! [Steinhaus-Johnson-Trotter][Steinhaus-Johnson-Trotter's algorithm].
//!
//! However, since the graph is complete (every node has a distance to every other node), this
//! particular problem can be solved even faster, by avoiding the overhead of permutations and
//! instead using [Held-Karp's dynamic programming][Held-Karp] solution. This algorithm is
//! O(n²*2ⁿ); for our puzzle with 8 nodes, this gives `8*7/2*256/2` or 3,584 comparisons needed.
//! On the surface, this is more comparisons than the 2,520 comparisons of the O(n!) permutation
//! solution, but set manipulation is less expensive than computation of permutations, so it
//! is an overall win.
//!
//! The core behavior of Held-Karp involves computing the function g(set, k) for all possible
//! sets of cities, where the function represents the best (shortest or longest) distance seen so
//! far for a given set of cities and ending on the city k which is a member of the set. Unlike the
//! permutations approach which visits every path in the graph, Held-Karp discards information
//! along the way to compute only the best cycle in a graph anchored to a given start point. It is
//! easy to demonstrate a graph where the best path is not part of the best cycle (discarding the
//! longest edge from the shortest cycle might leave you with a path longer than the graph's true
//! shortest path, if that other path had a different start anchor). But this is easy to work
//! around, by adding a ninth "location" with distance 0 to every other location, and using that
//! location as the start and end for every cycle, at which point the best cycle of nine includes
//! the best path of all eight locations.
//!
//! The algorithm is recursive: with a base case of g({k},k) being zero (the best distance
//! to a singleton set from our ninth point is 0), all other g(set,k) can be computed by
//! iterating over each member of set excluding k, and choosing the best variant g(set∖k,m)+d(k,m)
//! possible from a smaller set ending in m. Iterating over bitmasks from 0 to 255 ensures
//! that all earlier subsets are available when computing for a larger set.
//!
//! For speed we first convert each location into an index, then store the distances between
//! every pair of locations in an array for fast lookup. Storing sets plus the last city visited
//! requires 8 bits for the set and 3 bits for the city, for a total table size of 2¹¹ distances.
//!
//! [Travelling Salesman Problem]: https://en.wikipedia.org/wiki/Travelling_salesman_problem
//! [`half_permutations`]: crate::util::slice
//! [Steinhaus-Johnson-Trotter]: https://en.wikipedia.org/wiki/Steinhaus-Johnson-Trotter_algorithm
//! [Held-Karp]: https://en.wikipedia.org/wiki/Held%E2%80%93Karp_algorithm
use crate::util::bitset::*;
use crate::util::hash::*;
use crate::util::iter::*;
use crate::util::parse::*;

type Result = (u16, u16);

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
    let mut distances = vec![0_u16; stride * stride];

    for [start, _, end, _, distance] in &tokens {
        let start = indices[start];
        let end = indices[end];
        let distance = distance.unsigned();

        distances[stride * start + end] = distance;
        distances[stride * end + start] = distance;
    }

    // Initialize a table for each part: 2ⁿ sets with n distances per set. Default 0 matches
    // g({k},k) of zero for all singleton sets. Initial value of other sets does not matter.
    let mut table_one = vec![0_u16; stride * (1 << stride)];
    let mut table_two = vec![0_u16; stride * (1 << stride)];

    // Visit each non-empty set in order, with no work to do for singleton sets.
    for set in 3_usize..(1 << stride) {
        if set & !(set - 1) == set {
            continue;
        }

        // For a given set, compute each g(set,k) for all k in the set.
        for k in set.biterator() {
            let subset = set ^ (1 << k);
            let mut shortest = u16::MAX;
            let mut longest = 0;

            // For a given destination k, find which other bit m gives the best path from the
            // subset to m, and then m to k. All table[subset] references were filled in prior
            // iterations of the outer loop or the singleton base cases.
            for m in subset.biterator() {
                shortest = shortest.min(table_one[subset * stride + m] + distances[m * stride + k]);
                longest = longest.max(table_two[subset * stride + m] + distances[m * stride + k]);
            }
            table_one[set * stride + k] = shortest;
            table_two[set * stride + k] = longest;
        }
    }

    // With the sets now built, we have stride candidates for each answer.
    let last_row = table_one.len() - stride;
    (*table_one[last_row..].iter().min().unwrap(), *table_two[last_row..].iter().max().unwrap())
}

pub fn part1(input: &Result) -> u16 {
    input.0
}

pub fn part2(input: &Result) -> u16 {
    input.1
}
