//! # Seating System
//!
//! Cellular automata are hard to speed up due to the need to check all neighbors each iteration.
//! For both parts we minimize expensive memory allocation by creating only two temporary buffers
//! then swapping between them each turn, a similar approach to double buffering.
//!
//! For part two we can further optimize by precalculating the locations of the nearest visible
//! seats only once then reusing that information for each step.
use crate::util::grid::*;
use crate::util::point::*;
use std::mem::swap;

const FLOOR: u8 = b'.';
const DIRECTIONS: [Point; 8] = [
    Point { x: -1, y: -1 },
    Point { x: 0, y: -1 },
    Point { x: 1, y: -1 },
    Point { x: -1, y: 0 },
    Point { x: 1, y: 0 },
    Point { x: -1, y: 1 },
    Point { x: 0, y: 1 },
    Point { x: 1, y: 1 },
];

struct Seat {
    index: u16,
    size: u8,
    neighbors: [u16; 8],
}

impl Seat {
    #[inline]
    fn push(&mut self, index: u16) {
        self.neighbors[self.size as usize] = index;
        self.size += 1;
    }
}

pub fn parse(input: &str) -> Grid<u8> {
    Grid::parse(input)
}

pub fn part1(input: &Grid<u8>) -> u32 {
    simulate(input, true, 4)
}

pub fn part2(input: &Grid<u8>) -> u32 {
    simulate(input, false, 5)
}

pub fn simulate(input: &Grid<u8>, part_one: bool, limit: u8) -> u32 {
    let width = input.width;
    let height = input.height;
    let mut seats = Vec::new();

    for y in 0..height {
        for x in 0..width {
            let point = Point { x, y };
            if input[point] == FLOOR {
                continue;
            }

            let mut seat = Seat { index: (width * y + x) as u16, size: 0, neighbors: [0; 8] };

            for direction in DIRECTIONS {
                if part_one {
                    let next = point + direction;
                    if input.contains(next) && input[next] != FLOOR {
                        seat.push((width * next.y + next.x) as u16);
                    }
                } else {
                    let mut next = point + direction;
                    while input.contains(next) {
                        if input[next] != FLOOR {
                            seat.push((width * next.y + next.x) as u16);
                            break;
                        }
                        next += direction;
                    }
                }
            }

            seats.push(seat);
        }
    }

    let mut current = vec![0; (width * height) as usize];
    let mut next = vec![0; (width * height) as usize];
    let mut change = true;

    while change {
        change = false;

        for seat in &seats {
            let index = seat.index as usize;
            let mut total = 0;

            for i in 0..seat.size {
                total += current[seat.neighbors[i as usize] as usize];
            }

            if current[index] == 0 && total == 0 {
                next[index] = 1;
                change |= true;
            } else if current[index] == 1 && total >= limit {
                next[index] = 0;
                change |= true;
            } else {
                next[index] = current[index];
            }
        }

        swap(&mut current, &mut next);
    }

    current.iter().map(|&n| n as u32).sum()
}
