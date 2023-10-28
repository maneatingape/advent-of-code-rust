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
//! every pair of locations in an vec for fast lookup. Our utility [`permutations`] method uses
//! [Heap's algorithm](https://en.wikipedia.org/wiki/Heap%27s_algorithm) for efficiency,
//! modifying the slice in place.
//!
//! There are 8 locations, however since we always start at `0` this requires checking only
//! 7! = 5,040 permutations. We find the answer to both part one and two simultaneously.
//!
//! [`permutations`]: crate::util::slice
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

    let stride = found.len();
    let mut distance = vec![0; stride * stride];

    // BFS from each location. As a minor optimization we reuse `todo` and `visited`.
    let mut todo = VecDeque::new();
    let mut visited = vec![0; grid.bytes.len()];
    let orthogonal = [1, -1, grid.width, -grid.width].map(|i| i as usize);

    for start in found {
        let from = grid.bytes[start].to_decimal() as usize;

        todo.push_back((start, 0));
        visited[start] = start;

        while let Some((index, steps)) = todo.pop_front() {
            if grid.bytes[index].is_ascii_digit() {
                let to = grid.bytes[index].to_decimal() as usize;
                distance[stride * from + to] = steps;
            }

            for offset in orthogonal {
                let next_index = index.wrapping_add(offset);

                if grid.bytes[next_index] != b'#' && visited[next_index] != start {
                    visited[next_index] = start;
                    todo.push_back((next_index, steps + 1));
                }
            }
        }
    }

    // Solve both parts simultaneously.
    let mut part_one = u32::MAX;
    let mut part_two = u32::MAX;
    let mut indices: Vec<_> = (1..stride).collect();

    indices.permutations(|slice| {
        let link = |from, to| distance[stride * from + to];

        let first = link(0, slice[0]);
        let middle = slice.windows(2).map(|w| link(w[0], w[1])).sum::<u32>();
        let last = link(slice[slice.len() - 1], 0);

        part_one = part_one.min(first + middle);
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
