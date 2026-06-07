//! # Air Duct Spelunking
//!
//! This is a variant of the classic [Travelling Salesman Problem] and is similar to
//! [`Year 2015 Day 13`].
//!
//! We first simplify the problem by finding the distance between all locations using multiple
//! [BFS](https://en.wikipedia.org/wiki/Breadth-first_search)
//! searches starting from each location.
//!
//! Then we can use [Held-Karp's dynamic programming][Held-Karp] algorithm to determine the
//! shortest cycle. The problem asks us to start from node 0, which conveniently means that the
//! value g(127, k) is the shortest path to k, and adding the distance from k back to 0 for part 2
//! is also trivial. Thus, this day completes with only `7*6/2*128/2` or 1,344 comparisons, quite a
//! bit better than the 2,520 comparisons needed for an approach with 7!/2 permutations. A
//! slight complication is that set bit 0 maps to node 1.
//!
//! [`Year 2015 Day 13`]: crate::year2015::day13
//! [Travelling Salesman Problem]: https://en.wikipedia.org/wiki/Travelling_salesman_problem
//! [Held-Karp]: https://en.wikipedia.org/wiki/Held%E2%80%93Karp_algorithm
use crate::util::bitset::*;
use crate::util::grid::*;
use crate::util::parse::*;
use std::collections::VecDeque;

type Input = (u16, u16);

pub fn parse(input: &str) -> Input {
    let grid = Grid::parse(input);
    let found: Vec<_> =
        grid.bytes.iter().enumerate().filter(|(_, b)| b.is_ascii_digit()).map(|(i, _)| i).collect();

    let width = grid.width as isize;
    // There are 8 locations.
    let mut distance = [[0; 8]; 8];

    // BFS from each location. As minor optimizations we reuse `todo` and `seen`,
    // and short-circuit each BFS once it will not learn anything new.
    let mut todo = VecDeque::new();
    let mut seen = vec![0; grid.bytes.len()];

    for (rank, &start) in found.iter().skip(1).enumerate() {
        let from = grid.bytes[start].to_decimal() as usize;
        let mut need = found.len() - rank;

        todo.clear();
        todo.push_back((start, 0));
        seen[start] = start;

        while let Some((index, steps)) = todo.pop_front() {
            if grid.bytes[index].is_ascii_digit() {
                let to = grid.bytes[index].to_decimal() as usize;
                if distance[from][to] == 0 {
                    distance[from][to] = steps;
                    distance[to][from] = steps;
                    need -= 1;
                    // Short-circuit once we've found all needed pairs.
                    if need == 0 {
                        break;
                    }
                }
            }

            // All interesting points (digits and junctions) are at odd locations,
            // so we step by 2 spaces in each direction.
            for delta in [1, -1, width, -width] {
                let first = index.wrapping_add_signed(delta);
                if grid.bytes[first] != b'#' {
                    let second = index.wrapping_add_signed(2 * delta);
                    if seen[second] != start {
                        seen[second] = start;
                        todo.push_back((second, steps + 2));
                    }
                }
            }
        }
    }

    // Solve both parts simultaneously.
    // Initialize a table for each part: 2ⁿ⁻¹ sets with n-1 distances per set. Default each g({k},k)
    // singleton to distance[0][k+1] (since bit 0 maps to node 1), while the initial value of other
    // sets does not matter.
    let mut table = [[0_u16; 7]; 1 << 7];
    for k in 0..found.len() - 1 {
        table[1 << k][k] = distance[0][k + 1];
    }

    // Visit each non-empty set in order, with no work to do for singleton sets.
    for set in 3_usize..(1 << (found.len() - 1)) {
        if set & !(set - 1) == set {
            continue;
        }

        // For a given set, compute each g(set,k) for all k in the set.
        for k in set.biterator() {
            let subset = set ^ (1 << k);
            let mut shortest = u16::MAX;

            // For a given destination k, find which other bit m gives the best path from the
            // subset to m, and then m to k. All table[subset] references were filled in prior
            // iterations of the outer loop or the singleton base cases.
            for m in subset.biterator() {
                shortest = shortest.min(table[subset][m] + distance[m + 1][k + 1]);
            }
            table[set][k] = shortest;
        }
    }

    // With the sets now built, we have 7 candidates for each answer.
    let mut part_one = u16::MAX;
    let mut part_two = u16::MAX;
    for (k, &path_len) in
        table[(1 << (found.len() - 1)) - 1].iter().take(found.len() - 1).enumerate()
    {
        part_one = part_one.min(path_len);
        part_two = part_two.min(path_len + distance[k + 1][0]);
    }

    (part_one, part_two)
}

pub fn part1(input: &Input) -> u16 {
    input.0
}

pub fn part2(input: &Input) -> u16 {
    input.1
}
