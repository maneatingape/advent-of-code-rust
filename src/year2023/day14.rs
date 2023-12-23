//! # Parabolic Reflector Dish
//!
//! To solve part two we look for a cycle where the dish returns to a previously seen state.
//! By storing each dish and a index in a `HashMap` we can calculate the offset and length of the
//! cycle then use that to find to state at the billionth step.
use crate::util::grid::*;
use crate::util::hash::*;
use crate::util::point::*;

pub struct Input {
    width: i32,
    height: i32,
    rounded: Vec<i16>,
    fixed_north: Vec<i16>,
    fixed_west: Vec<i16>,
    fixed_south: Vec<i16>,
    fixed_east: Vec<i16>,
    roll_north: Vec<i16>,
    roll_west: Vec<i16>,
    roll_south: Vec<i16>,
    roll_east: Vec<i16>,
}

pub fn parse(input: &str) -> Input {
    let inner = Grid::parse(input);
    let mut grid = Grid {
        width: inner.width + 2,
        height: inner.height + 2,
        bytes: vec![b'#'; ((inner.width + 2) * (inner.height + 2)) as usize],
    };

    // Copy
    for y in 0..inner.width {
        for x in 0..inner.width {
            let src = Point::new(x, y);
            let dst = Point::new(x + 1, y + 1);
            grid[dst] = inner[src];
        }
    }

    let copy = || Grid { width: grid.width, height: grid.height, bytes: vec![0; grid.bytes.len()] };

    let mut rounded = Vec::new();
    let mut north = copy();
    let mut west = copy();
    let mut south = copy();
    let mut east = copy();
    let mut roll_north = Vec::new();
    let mut roll_west = Vec::new();
    let mut roll_south = Vec::new();
    let mut roll_east = Vec::new();

    // Rounded
    for y in 0..grid.height {
        for x in 0..grid.width {
            let point = Point::new(x, y);
            if grid[point] == b'O' {
                rounded.push((grid.width * point.y + point.x) as i16);
            }
        }
    }

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

    let mut result = 0;
    let rounded = &mut input.rounded.clone();
    let state = tilt(rounded, fixed_north, roll_north, *width as i16);

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

    // Find cycle
    let (start, end) = loop {
        tilt(rounded, &input.fixed_north, &input.roll_north, *width as i16);
        tilt(rounded, &input.fixed_west, &input.roll_west, 1);
        tilt(rounded, &input.fixed_south, &input.roll_south, -(*width) as i16);
        let state = tilt(rounded, &input.fixed_east, &input.roll_east, -1);

        if let Some(previous) = seen.insert(state, seen.len()) {
            break (previous, seen.len());
        }
    };

    let offset = 1_000_000_000 - 1 - start;
    let cycle_width = end - start;
    let remainder = offset % cycle_width;
    let target = start + remainder;

    let (state, _) = seen.iter().find(|(_, &i)| i == target).unwrap();
    let mut result = 0;

    for (&a, &b) in input.roll_east.iter().zip(state.iter()) {
        let n = (a - b) as i32;
        let y = (a as i32) / width;
        result += n * (height - 1 - y);
    }

    result
}

fn tilt(rounded: &mut [i16], fixed: &[i16], roll: &[i16], direction: i16) -> Vec<i16> {
    let mut state = roll.to_vec();

    for rock in rounded {
        let index = fixed[*rock as usize] as usize;
        state[index] += direction;
        *rock = state[index];
    }

    state
}
