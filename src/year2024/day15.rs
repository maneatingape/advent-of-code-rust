//! # Warehouse Woes
//!
//! Festive version of [Sokoban](https://en.wikipedia.org/wiki/Sokoban).
//!
//! Part one loops in a straight line looking for the next space `.` or wall `#`. No bounds checks
//! are needed as the maze is enclosed. If a space is found then all items are pushed one block
//! in that direction.
//!
//! Part two re-uses the part one logic for horizontal moves. Vertical moves use a
//! [breadth-first search](https://en.wikipedia.org/wiki/Breadth-first_search) to identify the
//! cascading boxes that need to be moved. Boxes are added strictly left to right to make checking
//! for previously added boxes easier. To prevent adding a box twice we check that the
//! item at `index - 2` is different. For example:
//!
//! ```none
//!     @          Indices:
//!     []         23
//!    [][]       4567
//!     []         89
//! ```
//!
//! When processing 6 we try to add 8, however 8 and 9 have already been added when processing 4
//! so we skip.
//!
//! If any next space is a wall then we cancel the entire move and return right away. Otherwise
//! all boxes are moved in the *reverse* order that they were found by the search.
use crate::util::grid::*;
use crate::util::point::*;
use std::mem::swap;

type Input<'a> = (Grid<u8>, &'a str);

pub fn parse(input: &str) -> Input<'_> {
    let (prefix, suffix) = input.split_once("\n\n").unwrap();
    let grid = Grid::parse(prefix);
    (grid, suffix)
}

pub fn part1(input: &Input<'_>) -> i32 {
    let (grid, moves) = input;

    // We don't need to move the robot symbol so mark as empty space once located.
    let mut grid = grid.clone();
    let mut position = grid.find(b'@').unwrap();
    grid[position] = b'.';

    // Treat moves as a single string ignoring any newline characters.
    for b in moves.bytes() {
        match b {
            b'<' => narrow(&mut grid, &mut position, LEFT),
            b'>' => narrow(&mut grid, &mut position, RIGHT),
            b'^' => narrow(&mut grid, &mut position, UP),
            b'v' => narrow(&mut grid, &mut position, DOWN),
            _ => (),
        }
    }

    gps(&grid, b'O')
}

pub fn part2(input: &Input<'_>) -> i32 {
    let (grid, moves) = input;

    let mut grid = stretch(grid);
    let mut position = grid.find(b'@').unwrap();
    grid[position] = b'.';

    // Reuse to minimize allocations.
    let mut todo = Vec::with_capacity(50);

    for b in moves.bytes() {
        match b {
            b'<' => narrow(&mut grid, &mut position, LEFT),
            b'>' => narrow(&mut grid, &mut position, RIGHT),
            b'^' => wide(&mut grid, &mut position, UP, &mut todo),
            b'v' => wide(&mut grid, &mut position, DOWN, &mut todo),
            _ => (),
        }
    }

    gps(&grid, b'[')
}

fn narrow(grid: &mut Grid<u8>, start: &mut Point, direction: Point) {
    let mut position = *start + direction;
    let mut size = 1;

    // Search for the next wall or open space.
    while grid[position] != b'.' && grid[position] != b'#' {
        position += direction;
        size += 1;
    }

    // Move items one space in direction.
    if grid[position] == b'.' {
        let mut previous = b'.';
        let mut position = *start + direction;

        for _ in 0..size {
            swap(&mut previous, &mut grid[position]);
            position += direction;
        }

        // Move robot
        *start += direction;
    }
}

fn wide(grid: &mut Grid<u8>, start: &mut Point, direction: Point, todo: &mut Vec<Point>) {
    // Short circuit if path in front of robot is empty.
    if grid[*start + direction] == b'.' {
        *start += direction;
        return;
    }

    // Clear any items from previous push.
    todo.clear();
    // Add dummy item to prevent index out of bounds when checking for previously added boxes.
    todo.push(ORIGIN);
    todo.push(*start);
    let mut index = 1;

    while index < todo.len() {
        let next = todo[index] + direction;
        index += 1;

        // Add boxes strictly left to right.
        let (first, second) = match grid[next] {
            b'[' => (next, next + RIGHT),
            b']' => (next + LEFT, next),
            b'#' => return, // Return early if there's a wall in the way.
            _ => continue,  // Open space doesn't add any more items to move.
        };

        // Check if this box has already been added by the previous box in this row.
        if first != todo[todo.len() - 2] {
            todo.push(first);
            todo.push(second);
        }
    }

    // Move boxes in reverse order, skipping the dummy item and robot.
    for &point in todo[2..].iter().rev() {
        grid[point + direction] = grid[point];
        grid[point] = b'.';
    }

    // Move robot
    *start += direction;
}

fn stretch(grid: &Grid<u8>) -> Grid<u8> {
    let mut next = Grid::new(grid.width * 2, grid.height, b'.');

    for y in 0..grid.height {
        for x in 0..grid.width {
            // Grid is already filled with '.', so only need to handle other kinds.
            let (left, right) = match grid[Point::new(x, y)] {
                b'#' => (b'#', b'#'),
                b'O' => (b'[', b']'),
                b'@' => (b'@', b'.'),
                _ => continue,
            };

            next[Point::new(2 * x, y)] = left;
            next[Point::new(2 * x + 1, y)] = right;
        }
    }

    next
}

fn gps(grid: &Grid<u8>, needle: u8) -> i32 {
    let mut result = 0;

    for y in 0..grid.height {
        for x in 0..grid.width {
            let point = Point::new(x, y);
            if grid[point] == needle {
                result += 100 * point.y + point.x;
            }
        }
    }

    result
}
