//! # Parabolic Reflector Dish
//!
//! To solve part two we look for a cycle where the dish returns to a previously seen state.
//! By storing each dish and a index in a `HashMap` we can calculate the offset and length of the
//! cycle then use that to find to state at the billionth step.
//!
//! Calculating the state needs to be done sequentially so we use some tricks to make it as fast as
//! possible.
//!
//! First the location of each each ball is stored in a `vec`. My input had ~2,000 balls compared to
//! 10,000 grid squares total, so this approach reduces the amount of data to scan by 5x. The 2D
//! coordinates are converted so a 1D number, for example the index of a ball on the second row
//! second column would be 1 * 100 + 1 = 101.
//!
//! Next for each possible tilt orientation (north, south, east and west) an approach similar to a
//! prefix sum is used. Each edge or fixed rock is assigned an index. We expand the grid by 2 in
//! each direction (one for each edge) to handles the edges. For example, using west (left):
//!
//! ```none
//!     ..#.#..
//! ```
//!
//! is represented in `fixed_west` as (noticing the extra 0 for the left edge)
//!
//! ```none
//!     0 0 0 1 1 2 2 2
//! ```
//!
//! The the number of balls the come to rest against each fixed point is counted, for example:
//!
//! ```none
//!     OO#.#OO
//! ```
//!
//! is stored in `roll_west` similar to:
//!
//! ```none
//!    2 0 2
//! ```
//!
//! This approach has two huge advantages:
//!
//! First, the number of balls resting against each fixed point completely represents the state of the
//! grid in a very compact format. For example my input has ~1600 fixed points. Using 2 bytes per
//! point needs 3.2K total to represent the grid, compared to 100 * 100 = 10K for the simple approach.
//! 3x less data is 3x faster to hash when storing states in a `HashMap` looking for duplicates.
//!
//! Second, calculating the new position of a ball is very fast. For each ball:
//!
//! * Use `fixed_*` to lookup the index in the corresponding `roll_*` vec.
//! * This stores the current index of the last ball resting against that fixed point.
//! * Increment this value by ±1 for horizontal movement or ±width for vertical movement
//!   and then update the new location of this ball.
//!
//! For example, tilting a single row west, processing each ball from left to right where each line
//! represent the new state would look like:
//!
//! ```none
//!    grid              rounded         fixed_west                       roll_west
//!    .O#..O.OO.#..O    [1 5 7 8 13]    [0 0 1 1 1 1 1 1 1 1 2 2 2 2]    [-1 2 10]
//!    O.#..O.OO.#..O    [0 5 7 8 13]    [0 0 1 1 1 1 1 1 1 1 2 2 2 2]    [0 2 10]
//!    O.#O...OO.#..O    [0 3 7 8 13]    [0 0 1 1 1 1 1 1 1 1 2 2 2 2]    [0 3 10]
//!    O.#OO...O.#..O    [0 3 4 8 13]    [0 0 1 1 1 1 1 1 1 1 2 2 2 2]    [0 4 10]
//!    O.#OOO....#..O    [0 3 4 5 13]    [0 0 1 1 1 1 1 1 1 1 2 2 2 2]    [0 5 10]
//!    O.#OOO....#O..    [0 3 4 5 11]    [0 0 1 1 1 1 1 1 1 1 2 2 2 2]    [0 5 11]
//! ```
use crate::util::grid::*;
use crate::util::hash::*;
use crate::util::point::*;

pub struct Input {
    width: i32,
    height: i32,
    // Index of each ball.
    rounded: Vec<i16>,
    // Index into corresponding `roll_` vec for each possible grid location.
    fixed_north: Vec<i16>,
    fixed_west: Vec<i16>,
    fixed_south: Vec<i16>,
    fixed_east: Vec<i16>,
    // The current index of the ball resting against each fixed point.
    roll_north: Vec<i16>,
    roll_west: Vec<i16>,
    roll_south: Vec<i16>,
    roll_east: Vec<i16>,
}

pub fn parse(input: &str) -> Input {
    // Expand the grid by 2 in each direction to handle edges the same way as fixed points.
    let inner = Grid::parse(input);
    let mut grid = Grid::new(inner.width + 2, inner.height + 2, b'#');

    // Copy inner grid.
    for y in 0..inner.width {
        for x in 0..inner.width {
            let src = Point::new(x, y);
            let dst = Point::new(x + 1, y + 1);
            grid[dst] = inner[src];
        }
    }

    let mut rounded = Vec::new();
    let mut north = grid.same_size_with(0);
    let mut west = grid.same_size_with(0);
    let mut south = grid.same_size_with(0);
    let mut east = grid.same_size_with(0);
    let mut roll_north = Vec::new();
    let mut roll_west = Vec::new();
    let mut roll_south = Vec::new();
    let mut roll_east = Vec::new();

    // Starting index of each rounded ball.
    for y in 0..grid.height {
        for x in 0..grid.width {
            let point = Point::new(x, y);
            if grid[point] == b'O' {
                rounded.push((grid.width * point.y + point.x) as i16);
            }
        }
    }

    // For each direction, store the next index that a ball will roll to in that direction.

    // North
    for x in 0..grid.width {
        for y in 0..grid.height {
            let point = Point::new(x, y);
            if grid[point] == b'#' {
                roll_north.push((grid.width * point.y + point.x) as i16);
            }
            north[point] = (roll_north.len() - 1) as i16;
        }
    }

    // West
    for y in 0..grid.height {
        for x in 0..grid.width {
            let point = Point::new(x, y);
            if grid[point] == b'#' {
                roll_west.push((grid.width * point.y + point.x) as i16);
            }
            west[point] = (roll_west.len() - 1) as i16;
        }
    }

    // South
    for x in 0..grid.width {
        for y in (0..grid.height).rev() {
            let point = Point::new(x, y);
            if grid[point] == b'#' {
                roll_south.push((grid.width * point.y + point.x) as i16);
            }
            south[point] = (roll_south.len() - 1) as i16;
        }
    }

    // East
    for y in 0..grid.height {
        for x in (0..grid.width).rev() {
            let point = Point::new(x, y);
            if grid[point] == b'#' {
                roll_east.push((grid.width * point.y + point.x) as i16);
            }
            east[point] = (roll_east.len() - 1) as i16;
        }
    }

    Input {
        width: grid.width,
        height: grid.height,
        rounded,
        fixed_north: north.bytes,
        fixed_west: west.bytes,
        fixed_south: south.bytes,
        fixed_east: east.bytes,
        roll_north,
        roll_west,
        roll_south,
        roll_east,
    }
}

pub fn part1(input: &Input) -> i32 {
    let Input { width, height, fixed_north, roll_north, .. } = input;

    // Tilt north only once.
    let mut result = 0;
    let rounded = &mut input.rounded.clone();
    let state = tilt(rounded, fixed_north, roll_north, *width as i16);

    // Find vertical distance of each ball from the bottom, remembering that the grid is 2 bigger.
    for (&a, &b) in input.roll_north.iter().zip(state.iter()) {
        for index in (a..b).step_by(input.width as usize) {
            let y = (index as i32) / width;
            result += height - 2 - y;
        }
    }

    result
}

pub fn part2(input: &Input) -> i32 {
    let Input { width, height, .. } = input;

    let rounded = &mut input.rounded.clone();
    let mut seen = FastMap::with_capacity(100);

    // Simulate tilting until a cycle is found.
    let (start, end) = loop {
        tilt(rounded, &input.fixed_north, &input.roll_north, *width as i16);
        tilt(rounded, &input.fixed_west, &input.roll_west, 1);
        tilt(rounded, &input.fixed_south, &input.roll_south, -(*width) as i16);
        let state = tilt(rounded, &input.fixed_east, &input.roll_east, -1);

        if let Some(previous) = seen.insert(state, seen.len()) {
            break (previous, seen.len());
        }
    };

    // Find the index of the state after 1 billion repetitions.
    let offset = 1_000_000_000 - 1 - start;
    let cycle_width = end - start;
    let remainder = offset % cycle_width;
    let target = start + remainder;

    let (state, _) = seen.iter().find(|&(_, &i)| i == target).unwrap();
    let mut result = 0;

    for (&a, &b) in input.roll_east.iter().zip(state.iter()) {
        // Number of balls resting against the fixed point.
        let n = (a - b) as i32;
        // Distance from bottom.
        let y = (a as i32) / width;
        // Total load.
        result += n * (height - 1 - y);
    }

    result
}

/// Very fast calculation of new state after tilting in the specified direction.
fn tilt(rounded: &mut [i16], fixed: &[i16], roll: &[i16], direction: i16) -> Vec<i16> {
    let mut state = roll.to_vec();

    for rock in rounded {
        let index = fixed[*rock as usize] as usize;
        state[index] += direction;
        *rock = state[index];
    }

    state
}
