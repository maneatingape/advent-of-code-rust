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

/// For part 1, `SIZE` is half the grid width or height, in order to start in the center.
/// For part 2, `SIZE` is the number of 2x3 blocks in a row; a grid of 250x166 blocks can hold
/// a range of 500x498 cells in 2x3 blocks. `CENTER` only matters in part 2.
const SIZE: usize = 250;
const HALF_WIDTH: usize = SIZE / 2;
const THIRD_HEIGHT: usize = SIZE / 3;
const CENTER: usize = HALF_WIDTH + SIZE * THIRD_HEIGHT;

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

/// Use a compressed grid where each byte stores 6 cells (2x3 block) with 2 bits per cell.
pub fn part2(input: &Grid<u8>) -> usize {
    // Assume that the carrier will never go outside the range 0 to 500 in both x and y axes
    // starting at the center. As we store 6 nodes per u16, we compress the x axis by
    // 2 and the y axis by 3, for a grid of 250 by 166 blocks.  Offset so that we start on
    // heading 6 (moving up at sextant 4).
    //
    // The map of headings to u16 shift locations:
    //    [0][1]
    // [9] 0  1 [2]
    // [8] 2  3 [3]
    // [7] 4  5 [4]
    //    [6][5]
    let mut grid3 = vec![0_u16; 2 * HALF_WIDTH * 2 * THIRD_HEIGHT];

    // Precompute half of the 10 * 4096 possible state transitions for faster simulation.
    // Since a block is symmetric about 180-degree rotation, headings 5-9 utilize rotation
    // on the results of the cache for 0-4.
    let cache3: [[_; 4096]; 5] =
        from_fn(|heading| from_fn(|state| compute_block3(&mut grid3, heading, state)));

    // Copy the smaller initial input grid to the center of the larger grid, packing 6 nodes
    // into each byte. Ensure that center,center maps to sextant 4 (ie. lines center-2, center-1,
    // and center land in same u16), by putting the center at 250,251.
    let offset = SIZE - (input.width as usize / 2);

    for y in 0..input.height {
        for x in 0..input.width {
            if input[Point::new(x, y)] == b'#' {
                let (adjusted_x, adjusted_y) = (x as usize + offset, y as usize + offset + 1);
                let index = SIZE * (adjusted_y / 3) + (adjusted_x / 2);
                let offset = 4 * (adjusted_y % 3) + 2 * (adjusted_x % 2);
                // Mark node as infected.
                grid3[index] |= 2 << offset;
            }
        }
    }

    // Start in the center of the grid, in the bottom-left corner of a 2x3 cell, facing up.
    let mut index = CENTER;
    let mut heading = 6; // Bottom-left corner, facing up
    let mut infected = 0;
    let mut remaining = 10_000_000;

    // Memoized blocks can combine up to 12 steps. Handle the last few steps individually to
    // prevent overshooting the step target and overcounting the infected node transitions.
    // With 4.1 million cache lookups for 10 million repetitions, saving time inside this hot
    // loop is essential. By bit-packing 5 fields into a single `u32`, we limit the size of the
    // array to 20k entries * 4 bytes = 80kB making sure that it stays smaller than L1 cache.
    while remaining > 12 {
        let state = grid3[index] as usize;
        if heading < 5 {
            let packed = cache3[heading][state];
            grid3[index] = (packed % 4096) as u16; // bits 0-11
            index = index + (packed >> 23) as usize - SIZE; // bits 23-31
            heading = ((packed >> 12) % 16) as usize; // bits 12-15
            infected += ((packed >> 16) % 8) as usize; // bits 16-18
            remaining -= ((packed >> 19) % 16) as usize; // bits 19-22
        } else {
            let packed = cache3[heading - 5][rotate_block(state)];
            grid3[index] = rotate_block(packed as usize % 4096) as u16; // bits 0-11
            index = index + SIZE - (packed >> 23) as usize; // bits 23-31
            heading = (((packed >> 12) % 16) + 5) as usize % 10; // bits 12-15
            infected += ((packed >> 16) % 8) as usize; // bits 16-18
            remaining -= ((packed >> 19) % 16) as usize; // bits 19-22
        }
    }

    let (mut sextant, mut direction) = decode(heading);

    // Handle up to 12 remaining steps individually to prevent overcounting.
    for _ in 0..remaining {
        let [next_index, next_sextant, next_direction, next_infected] =
            step3(&mut grid3, index, sextant, direction);
        index = next_index;
        sextant = next_sextant;
        direction = next_direction;
        infected += next_infected;
    }

    infected
}

/// Computes the number of steps taken, infected nodes and next location for 2 x 3 blocks of nodes.
#[inline]
fn compute_block3(grid3: &mut [u16], heading: usize, state: usize) -> u32 {
    let mut index = CENTER;
    let mut infected = 0;
    let mut steps = 0;

    // Temporarily use the grid. This allows the index to move without exceeding bounds.
    grid3[CENTER] = state as u16;

    // Convert heading into placement and direction
    let (mut placement, mut direction) = decode(heading);

    // Count steps and infected nodes until we leave this cell.
    while index == CENTER {
        let [next_index, next_placement, next_direction, next_infected] =
            step3(grid3, index, placement, direction);
        index = next_index;
        placement = next_placement;
        direction = next_direction;
        infected += next_infected;
        steps += 1;
    }

    // Reset the grid to zero and figure out the next index. We offset index by SIZE to keep the
    // value positive for easier bit manipulation.
    let next_state = take(&mut grid3[CENTER]);
    let next_index = index + SIZE - CENTER;

    let heading = match (placement, direction) {
        (0, 2) => 0,
        (1, 2) => 1,
        (1, 3) => 2,
        (3, 3) => 3,
        (5, 3) => 4,
        (5, 0) => 5,
        (4, 0) => 6,
        (4, 1) => 7,
        (2, 1) => 8,
        _ => 9,
    };

    // Pack six fields into a single `u32`, maximizing cache locality by minimizing space.
    next_state as u32  // 0-11
        | (heading << 12) // 12-15 (value 0-9)
        | (infected << 16) as u32 // 16-18 (value 0-4)
        | (steps << 19) // 19-22 (value 0-12)
        | (next_index << 23) as u32 // 23-31 (value SIZE-1,SIZE+1,SIZE-250,SIZE+250)
}

/// Process a single step in any arbitrary location on the grid.
#[inline]
fn step3(grid3: &mut [u16], index: usize, placement: usize, direction: usize) -> [usize; 4] {
    // 6 nodes are packed into a single byte with headings arranged as:
    // [ 0 1 ]
    // [ 2 3 ]
    // [ 4 5 ]
    let shift = 2 * placement;
    let node = (grid3[index] >> shift) % 4;

    // Nodes cycle between 4 possible values:
    // 0 = Clean, 1 = Weakened, 2 = Infected, 3 = Flagged
    let next_node = (node + 1) % 4;
    // Direction changes based on the *previous* value of the node. In clockwise order:
    // 0 = Up, 1 = Right, 2 = Down, 3 = Left
    let next_direction = (direction + node as usize + 3) % 4;

    // Update the 2 bits representing the current node.
    let mask = !(0b11 << shift);
    grid3[index] = (grid3[index] & mask) | (next_node << shift);

    // Calculate x and y coordinates as if a single node was stored in each cell.
    // This is used in the next step in order to calculate if the index has changed.
    let (x, y) = (2 * (index % SIZE) + placement % 2, 3 * (index / SIZE) + placement / 2);
    let (x, y) = match next_direction {
        0 => (x, y - 1),
        1 => (x + 1, y),
        2 => (x, y + 1),
        _ => (x - 1, y),
    };

    // Convert the x and y coordinates back into the compressed values for 2 x 3 nodes in each cell.
    let next_index = SIZE * (y / 3) + (x / 2);
    let next_placement = 2 * (y % 3) + (x % 2);
    let infected = usize::from(next_node == 2);

    [next_index, next_placement, next_direction, infected]
}

/// Map a heading (0-9) into its sextant and direction.
fn decode(heading: usize) -> (usize, usize) {
    match heading {
        0 => (0, 2),
        1 => (1, 2),
        2 => (1, 3),
        3 => (3, 3),
        4 => (5, 3),
        5 => (5, 0),
        6 => (4, 0),
        7 => (4, 1),
        8 => (2, 1),
        _ => (0, 1),
    }
}

/// Rotate a 2x3 block by 180 degrees. The two bits for sextant 012345 become 543210.
#[inline]
fn rotate_block(value: usize) -> usize {
    // See https://graphics.stanford.edu/~seander/bithacks.html#ReverseByteWith64Bits
    // for inspiration. Expand 12 bits into three 16-bit lanes.
    let reps = value * 0x4_0004_0004;
    // Mask out each pair that occurs in right 12-bit lane.
    let bits = reps & 0x30c_0c30_30c0;
    // Multiply to merge those lanes into a common 12-bit middle; carry is not an issue.
    let prod = bits.wrapping_mul(0x10_0100_1001);
    // Grab the desired result.
    (prod >> 36) & 0xfff
}
