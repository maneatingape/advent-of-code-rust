//! # Air Duct Spelunking
//!
//! This is a variant of the classic
//! [Travelling Salesman Problem](https://en.wikipedia.org/wiki/Travelling_salesman_problem) and
//! is similar to [`Year 2015 Day 13`].
//!
//! We first simplify the problem by finding the distance between all locations using multiple
//! [BFS](https://en.wikipedia.org/wiki/Breadth-first_search)
//! searches starting from each location.
//!
//! For speed we convert each location into an index, then store the distances between
//! every pair of locations in a vec for fast lookup. Our utility [`half_permutations`] method uses
//! [Steinhaus-Johnson-Trotter's algorithm](https://en.wikipedia.org/wiki/Steinhaus%E2%80%93Johnson%E2%80%93Trotter_algorithm) for efficiency,
//! modifying the slice in place.
//!
//! There are 8 locations, however since we always start at `0` this requires checking only
//! 7!/2 = 2,520 permutations. We find the answer to both part one and two simultaneously.
//!
//! [`half_permutations`]: crate::util::slice
//! [`Year 2015 Day 13`]: crate::year2015::day13
use crate::util::grid::*;
use crate::util::parse::*;
use crate::util::slice::*;
use std::collections::VecDeque;

type Input = (u32, u32);

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
                    // Short-circuit once we found all needed pairs
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
    let mut part_one = u32::MAX;
    let mut part_two = u32::MAX;
    let mut indices: Vec<_> = (1..found.len()).collect();

    indices.half_permutations(|slice| {
        let first = distance[0][slice[0]];
        let middle = slice.windows(2).map(|w| distance[w[0]][w[1]]).sum::<u32>();
        let last = distance[slice[slice.len() - 1]][0];

        part_one = part_one.min(first + middle).min(middle + last);
        part_two = part_two.min(first + middle + last);
    });

    (part_one, part_two)
}

pub fn part1(input: &Input) -> u32 {
    input.0
}

pub fn part2(input: &Input) -> u32 {
    input.1
}
