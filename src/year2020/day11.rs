//! # Seating System
//!
//! Cellular automata are hard to speed up due to the need to check all neighbors each iteration.
//! For both parts we minimize expensive memory allocation by creating only two temporary buffers
//! then swapping between them each turn, a similar approach to double buffering.
//!
//! For part two we can further optimize by precalculating the locations of the nearest visible
//! seats only once then reusing that information for each step.
//!
//! The SIMD version speed things up by calculating 32 lanes at a time.
use crate::util::grid::*;
use crate::util::point::*;

const SEAT: u8 = b'L';

pub fn parse(input: &str) -> Grid<u8> {
    Grid::parse(input)
}

pub fn part1(input: &Grid<u8>) -> u32 {
    #[cfg(not(feature = "simd"))]
    let result = scalar::simulate(input, false, 4);

    #[cfg(feature = "simd")]
    let result = simd::simulate(input, false, 4);

    result
}

pub fn part2(input: &Grid<u8>) -> u32 {
    #[cfg(not(feature = "simd"))]
    let result = scalar::simulate(input, true, 5);

    #[cfg(feature = "simd")]
    let result = simd::simulate(input, true, 5);

    result
}

#[cfg(not(feature = "simd"))]
mod scalar {
    use super::*;

    struct Seat {
        point: Point,
        size: usize,
        neighbors: [Point; 8],
    }

    impl Seat {
        fn push(&mut self, index: Point) {
            self.neighbors[self.size] = index;
            self.size += 1;
        }
    }

    pub(super) fn simulate(input: &Grid<u8>, part_two: bool, limit: u8) -> u32 {
        let mut seats = Vec::new();

        for y in 0..input.height {
            for x in 0..input.width {
                let point = Point::new(x, y);
                if input[point] != SEAT {
                    continue;
                }

                let mut seat = Seat { point, size: 0, neighbors: [ORIGIN; 8] };

                for direction in DIAGONAL {
                    if part_two {
                        let mut next = point + direction;
                        while input.contains(next) {
                            if input[next] == SEAT {
                                seat.push(next);
                                break;
                            }
                            next += direction;
                        }
                    } else {
                        let next = point + direction;
                        if input.contains(next) && input[next] == SEAT {
                            seat.push(next);
                        }
                    }
                }

                seats.push(seat);
            }
        }

        let mut current = input.same_size_with(0);
        let mut next = input.same_size_with(0);

        loop {
            for seat in &seats {
                let total: u8 = seat.neighbors[0..seat.size].iter().map(|&i| current[i]).sum();

                next[seat.point] = if current[seat.point] == 0 {
                    u8::from(total == 0)
                } else {
                    u8::from(total < limit)
                };
            }

            (current, next) = (next, current);
            if current == next {
                return current.bytes.iter().map(|&n| n as u32).sum();
            }
        }
    }
}

#[cfg(feature = "simd")]
mod simd {
    use super::*;
    use std::simd::cmp::SimdPartialEq as _;
    use std::simd::cmp::SimdPartialOrd as _;
    use std::simd::*;

    const LANE_WIDTH: usize = 32;
    type Vector = Simd<u8, LANE_WIDTH>;

    pub(super) fn simulate(input: &Grid<u8>, part_two: bool, limit: u8) -> u32 {
        // Input grid is taller than it is wide. To make efficient use of the wide SIMD operations:
        // * Add an empty border to eliminate bounds checking.
        // * Transpose the input grid to make it wider than it is tall.
        // * Round width up to next multiple of LANE_WIDTH.
        let width = 2 + (input.height as usize).next_multiple_of(LANE_WIDTH) as i32;
        let height = 2 + input.width;
        let mut grid = Grid::new(width, height, 0);

        for y in 0..input.height {
            for x in 0..input.width {
                let from = Point::new(x, y);
                let to = Point::new(y + 1, x + 1);
                grid[to] = u8::from(input[from] == SEAT);
            }
        }

        // Build a list of seats that are non-adjacent but visible to each other.
        let mut visible = Vec::new();

        if part_two {
            for y in 0..height {
                for x in 0..width {
                    let from = Point::new(x, y);
                    if grid[from] == 0 {
                        continue;
                    }

                    for direction in DIAGONAL {
                        if grid[from + direction] == 1 {
                            continue;
                        }

                        let mut to = from + direction * 2;
                        while grid.contains(to) {
                            if grid[to] == 1 {
                                visible.push((from, to));
                                break;
                            }
                            to += direction;
                        }
                    }
                }
            }
        }

        // Common constants.
        let zero: Vector = Simd::splat(0);
        let one: Vector = Simd::splat(1);
        let limit: Vector = Simd::splat(limit);

        let mut current = grid.same_size_with(0);
        let mut next = grid.same_size_with(0);
        let mut extra = grid.same_size_with(0);

        loop {
            // Add any non-adjacent seats that are visible to the total.
            if part_two {
                extra.bytes.fill(0);
                for &(from, to) in &visible {
                    extra[to] += current[from];
                }
            }

            // Process grid column by column using wide SIMG vectors.
            for x in (1..width - 1).step_by(LANE_WIDTH) {
                let mut above = horizontal_neighbors(&current, x, 0);
                let mut row = horizontal_neighbors(&current, x, 1);

                for y in 1..height - 1 {
                    let index = (width * y + x) as usize;
                    let seats = Simd::from_slice(&grid.bytes[index..]);
                    let occupied = Simd::from_slice(&current.bytes[index..]);
                    let extra = Simd::from_slice(&extra.bytes[index..]);

                    let below = horizontal_neighbors(&current, x, y + 1);
                    let total = row + above + below + extra;
                    above = row;
                    row = below;

                    // Empty to occupied.
                    let first = total.simd_eq(zero).select(one, zero);
                    // Occupied to empty
                    let second = total.simd_le(limit).select(occupied, zero);
                    // Nobody sits on the floor.
                    let result = (first + second) & seats;

                    result.copy_to_slice(&mut next.bytes[index..]);
                }
            }

            (current, next) = (next, current);
            if current == next {
                return current.bytes.iter().map(|&b| b as u32).sum();
            }
        }
    }

    /// Create SIMD vector of the sum of left, right and center lanes.
    #[inline]
    fn horizontal_neighbors(grid: &Grid<u8>, x: i32, y: i32) -> Vector {
        let index = (grid.width * y + x) as usize;

        let center = Simd::from_slice(&grid.bytes[index..]);
        let left = center.shift_elements_left::<1>(grid.bytes[index + LANE_WIDTH]);
        let right = center.shift_elements_right::<1>(grid.bytes[index - 1]);

        center + left + right
    }
}
