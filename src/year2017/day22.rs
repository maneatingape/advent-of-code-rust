//! # Sporifica Virus
//!
//! Part two is made faster by a factor of two by packing 4 nodes into each byte using
//! 2 bits per node. Then multiple steps are memoized for each of the 256 possible states,
//! for each of the 4 positions and each of the 4 directions, for a total of 4,096 combinations.
//! This allows us to skip forward up to 8 steps at a time. For example:
//!
//! ```none
//!    . = Clean   # = Infected   F = Flagged   W = Weakened
//!
//!    State    Direction    Steps    Infected
//!    [W] #    Down         0        0
//!     F  W
//!
//!     #  #    Down         1        1
//!    [F] W
//!
//!    [#] #    Up           2        1
//!     .  W
//!
//!     F [#]   Right        3        1
//!     .  W
//!
//!     F  F    Down         4        1
//!     . [W]
//!
//!     F  F    Down         5        2
//!     .  #
//!       [ ]
//! ```
//!
//! Starting in the top-left corner facing down, after 5 steps the virus carrier leaves the 2x2
//! block having infected 2 nodes. This is memoized as:
//!
//! ```none
//!     [0][2][01111001] => (2, 5, 10001111)
//! ```
use crate::util::grid::*;
use crate::util::point::*;
use std::array::from_fn;
use std::mem::take;

const SIZE: usize = 250;
const HALF: usize = SIZE / 2;
const CENTER: usize = SIZE * HALF + HALF;

pub fn parse(input: &str) -> Grid<u8> {
    Grid::parse(input)
}

/// Direct implementation on a fixed-size grid.
pub fn part1(input: &Grid<u8>) -> u32 {
    let size = SIZE as i32;
    let center = Point::new(size, size);
    let offset = center - Point::new(input.width / 2, input.height / 2);

    // Assume the virus carrier will never leave a 500 x 500 grid, starting at the center.
    let mut grid = Grid::new(2 * size, 2 * size, false);
    let mut position = center;
    let mut direction = UP;
    let mut infected = 0;

    // Copy the smaller initial input grid to the center of the larger grid.
    for y in 0..input.height {
        for x in 0..input.width {
            let point = Point::new(x, y);
            grid[point + offset] = input[point] == b'#';
        }
    }

    // The grid toggles between clean and infected.
    for _ in 0..10_000 {
        direction = if grid[position] {
            direction.clockwise()
        } else {
            infected += 1;
            direction.counter_clockwise()
        };
        grid[position] = !grid[position];
        position += direction;
    }

    infected
}

/// Use a compressed grid where each byte stores 4 cells (2x2 block) with 2 bits per cell.
pub fn part2(input: &Grid<u8>) -> usize {
    // Assume that the carrier will never go outside the range 0 to 500 in both x and y axes
    // starting at the center. As we store 4 nodes per byte, we compress the x and y axes by two.
    let mut grid = vec![0; SIZE * SIZE];

    // Precompute all 4 * 4 * 256 possible state transitions for faster simulation.
    let cache: [[[_; 256]; 4]; 4] = from_fn(|quadrant| {
        from_fn(|direction| from_fn(|state| compute_block(&mut grid, quadrant, direction, state)))
    });

    // Copy the smaller initial input grid to the center of the larger grid,
    // packing 4 nodes into each byte.
    let offset = SIZE - (input.width / 2) as usize;

    for x in 0..input.width {
        for y in 0..input.height {
            if input[Point::new(x, y)] == b'#' {
                let (adjusted_x, adjusted_y) = (x as usize + offset, y as usize + offset);
                let index = SIZE * (adjusted_y / 2) + (adjusted_x / 2);
                let offset = 4 * (adjusted_y % 2) + 2 * (adjusted_x % 2);
                // Mark node as infected.
                grid[index] |= 2 << offset;
            }
        }
    }

    // Start in the center of the grid, in the top-left corner of a 2x2 cell, facing up.
    let mut index = CENTER;
    let mut quadrant = 0; // Top-left corner
    let mut direction = 0; // Facing up
    let mut infected = 0;
    let mut remaining = 10_000_000;

    // Memoized blocks can combine up to 8 steps. Handle the last few steps individually to
    // prevent overshooting the step target and overcounting the infected node transitions.
    while remaining > 8 {
        let state = grid[index] as usize;
        let packed = cache[quadrant][direction][state];

        // With 10 million repetitions, saving time inside this hot loop is essential.
        // By bit-packing 6 fields into a single `u32`, we limit the size of the array to 16kB
        // making sure that it fits into L1 cache.
        grid[index] = packed as u8; // bits 0-7
        index = index + (packed >> 20) as usize - SIZE; // bits 20 to 31
        quadrant = ((packed >> 8) % 4) as usize; // bits 8-9
        direction = ((packed >> 10) % 4) as usize; // bits 10-11
        infected += ((packed >> 12) % 16) as usize; // bits 12-15
        remaining -= ((packed >> 16) % 16) as usize; // bits 16-19
    }

    // Handle up to 8 remaining steps individually to prevent overcounting.
    for _ in 0..remaining {
        let [next_index, next_quadrant, next_direction, next_infected] =
            step(&mut grid, index, quadrant, direction);
        index = next_index;
        quadrant = next_quadrant;
        direction = next_direction;
        infected += next_infected;
    }

    infected
}

/// Computes the number of steps taken, infected nodes and next location for 2 x 2 blocks of nodes.
fn compute_block(grid: &mut [u8], mut quadrant: usize, mut direction: usize, state: usize) -> u32 {
    let mut index = CENTER;
    let mut infected = 0;
    let mut steps = 0;

    // Temporarily use the grid. This allows the index to move without exceeding bounds.
    grid[CENTER] = state as u8;

    // Count steps and infected nodes until we leave this cell.
    while index == CENTER {
        let [next_index, next_quadrant, next_direction, next_infected] =
            step(grid, index, quadrant, direction);
        index = next_index;
        quadrant = next_quadrant;
        direction = next_direction;
        infected += next_infected;
        steps += 1;
    }

    // Reset the grid to zero and figure out the next index. We offset index by SIZE to keep the
    // value positive for easier bit manipulation.
    let next_state = take(&mut grid[CENTER]);
    let next_index = index + SIZE - CENTER;

    // Pack six fields into a single `u32`, maximizing cache locality by minimizing space.
    next_state as u32
        | (quadrant << 8) as u32
        | (direction << 10) as u32
        | (infected << 12) as u32
        | (steps << 16)
        | (next_index << 20) as u32
}

// Process a single step in any arbitrary location on the grid.
fn step(grid: &mut [u8], index: usize, quadrant: usize, direction: usize) -> [usize; 4] {
    // 4 nodes are packed into a single byte with quadrants arranged as:
    // [ 0 1 ]
    // [ 2 3 ]
    let shift = 2 * quadrant;
    let node = (grid[index] >> shift) % 4;

    // Nodes cycle between 4 possible values:
    // 0 = Clean, 1 = Weakened, 2 = Infected, 3 = Flagged
    let next_node = (node + 1) % 4;
    // Direction changes based on the *previous* value of the node. In clockwise order:
    // 0 = Up, 1 = Right, 2 = Down, 3 = Left
    let next_direction = (direction + node as usize + 3) % 4;

    // Update the 2 bits representing the current node.
    let mask = !(0b11 << shift);
    grid[index] = (grid[index] & mask) | (next_node << shift);

    // Calculate x and y coordinates as if a single node was stored in each cell.
    // This is used in the next step in order to calculate if the index has changed.
    let (x, y) = (2 * (index % SIZE) + quadrant % 2, 2 * (index / SIZE) + quadrant / 2);
    let (x, y) = match next_direction {
        0 => (x, y - 1),
        1 => (x + 1, y),
        2 => (x, y + 1),
        3 => (x - 1, y),
        _ => unreachable!(),
    };

    // Convert the x and y coordinates back into the compressed values for 2 x 2 nodes in each cell.
    let next_index = SIZE * (y / 2) + (x / 2);
    let next_quadrant = 2 * (y % 2) + (x % 2);
    let infected = usize::from(next_node == 2);

    [next_index, next_quadrant, next_direction, infected]
}
