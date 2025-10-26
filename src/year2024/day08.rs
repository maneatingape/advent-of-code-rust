//! # Resonant Collinearity
//!
//! Antenna frequencies are grouped together to reduce the O(n²) pairwise comparisons.
use crate::util::grid::*;
use crate::util::hash::*;
use crate::util::point::*;

type Input = (Grid<u8>, FastMap<u8, Vec<Point>>);

pub fn parse(input: &str) -> Input {
    let grid = Grid::parse(input);
    let mut antennas = FastMap::new();

    for y in 0..grid.height {
        for x in 0..grid.width {
            let point = Point::new(x, y);
            let frequency = grid[point];

            if frequency != b'.' {
                antennas.entry(frequency).or_insert_with(Vec::new).push(point);
            }
        }
    }

    (grid, antennas)
}

pub fn part1(input: &Input) -> u32 {
    let (grid, antennas) = input;
    let mut locations = grid.same_size_with(0);

    for frequency in antennas.values() {
        for &first in frequency {
            for &second in frequency {
                if first != second {
                    let distance = second - first;
                    let antinode = second + distance;

                    if grid.contains(antinode) {
                        locations[antinode] = 1;
                    }
                }
            }
        }
    }

    locations.bytes.iter().sum()
}

pub fn part2(input: &Input) -> u32 {
    let (grid, antennas) = input;
    let mut locations = grid.same_size_with(0);

    for frequency in antennas.values() {
        for &first in frequency {
            for &second in frequency {
                if first != second {
                    let distance = second - first;
                    let mut antinode = second;

                    while grid.contains(antinode) {
                        locations[antinode] = 1;
                        antinode += distance;
                    }
                }
            }
        }
    }

    locations.bytes.iter().sum()
}
