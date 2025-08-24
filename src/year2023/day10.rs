//! # Pipe Maze
//!
//! This solution uses the [Shoelace formula](https://en.wikipedia.org/wiki/Shoelace_formula)
//! and [Pick's theorem](https://en.wikipedia.org/wiki/Pick%27s_theorem).
//!
//! Starting at `S` we trace out the path followed by the pipes. Each corner piece
//! (`7`, `F`, `J`, `L` and finally `S`) is considered a vertex and added to the running total
//! for the area using the Shoelace formula. Additionally we keep track of the perimeter length.
//!
//! As the path is a loop the answer for part one is half the perimeter length.
//!
//! The answer for part two is the number of interior points. Rearranging Pick's theorem:
//!
//! `A = i + b / 2 - 1 => i = A - b / 2 + 1`
use crate::util::grid::*;
use crate::util::point::*;

type Input = (i32, i32);

pub fn parse(input: &str) -> Input {
    let grid = Grid::parse(input);
    let determinant = |a: Point, b: Point| a.x * b.y - a.y * b.x;

    // Find the starting position and direction.
    let mut corner = grid.find(b'S').unwrap();
    let mut direction = if matches!(grid[corner + UP], b'|' | b'7' | b'F') { UP } else { DOWN };
    let mut position = corner + direction;
    // Incrementally add up both perimeter and area.
    let mut steps = 1;
    let mut area = 0;

    loop {
        // Follow straight paths.
        while grid[position] == b'-' || grid[position] == b'|' {
            position += direction;
            steps += 1;
        }

        // Change direction at corner pieces.
        direction = match grid[position] {
            b'7' => {
                if direction == UP {
                    LEFT
                } else {
                    DOWN
                }
            }
            b'F' => {
                if direction == UP {
                    RIGHT
                } else {
                    DOWN
                }
            }
            b'J' => {
                if direction == DOWN {
                    LEFT
                } else {
                    UP
                }
            }
            b'L' => {
                if direction == DOWN {
                    RIGHT
                } else {
                    UP
                }
            }
            _ => {
                // We've looped all the way back to the start.
                area += determinant(corner, position);
                break;
            }
        };

        area += determinant(corner, position);
        corner = position;
        position += direction;
        steps += 1;
    }

    let part_one = steps / 2;
    let part_two = area.abs() / 2 - steps / 2 + 1;
    (part_one, part_two)
}

pub fn part1(input: &Input) -> i32 {
    input.0
}

pub fn part2(input: &Input) -> i32 {
    input.1
}
