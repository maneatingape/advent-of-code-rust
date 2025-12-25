//! # Monkey Map
//!
//! Parses any arbitrary cube map calculating the transitions between faces dynamically
//! using 3D vectors.
//!
//! We build the transitions with a BFS over the connected cube map. The first face we find
//! is labelled A in the diagram below. For each face we define 3 vectors:
//!
//! * `i` Horizontal from left to right in the plane of the face.
//! * `j` Vertical from top to bottom in the plane of the face.
//! * `k` Perpendicular to the face pointing into the body of the cube.
//!
//! ```none
//!              k (0, 0, 1)
//!             ^
//!            /
//!           /
//!          -------------+
//!         /            /|
//!        /    B       / |
//!       /            /  |
//!      +------------+---->i (1, 0, 0)
//!      |            | C |
//!      |     A      |  /
//!      |            | /
//!      |            |/
//!      +------------+
//!      |
//!      |
//!      v
//!      j (0, 1, 0)
//!
//! ```
//!
//! Then for each neighbouring face we can find its `i`, `j` and `k` vectors depending on which
//! edge it shares in common. For example if we move from face A to face B along the top edge
//! then the new vectors are:
//!
//! * `i` (1, 0, 0) Remains unchanged
//! * `j` (0, 0, -1) Minus previous `k`
//! * `k` (0, 1, 0) Previous `j`
//!
//! If face B and C are connected then the vectors for face C are:
//!
//! * `i` (0, 1, 0)
//! * `j` (0, 0, -1)
//! * `k` (-1, 0, 0)
//!
//! However if A and C were connected then the vectors for face C are:
//!
//! * `i` (0, 0, 1)
//! * `j` (0, 1, 0)
//! * `k` (-1, 0, 0)
//!
//! The really neat part is that when we leave the edge of a cube face the next
//! 3D vector *is always `k`* no matter which edge. We can find the new direction by comparing
//! the previous `k` against the new `i` and `j` vectors.
//!
//! For example say we transition from face `A` to face `B`. Our `k` is (0, 1, 0) which is
//! equal to minus the new `j`, so we know that we're travelling upwards from the bottom edge.
//! Then we can use this information to figure out the two dimensional offsets into the new face.
use crate::util::hash::*;
use crate::util::math::*;
use crate::util::parse::*;
use crate::util::point::*;
use std::collections::VecDeque;
use std::ops::Neg;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tile {
    None,
    Open,
    Wall,
}

enum Move {
    Left,
    Right,
    Forward(u32),
}

pub struct Grid {
    width: usize,
    height: usize,
    tiles: Vec<Tile>,
    start: i32,
    block: i32,
}

/// Return [`Tile::None`] for any point out of bounds.
impl Grid {
    fn tile(&self, point: Point) -> Tile {
        let x = point.x as usize;
        let y = point.y as usize;
        if (0..self.width).contains(&x) && (0..self.height).contains(&y) {
            self.tiles[y * self.width + x]
        } else {
            Tile::None
        }
    }
}

/// Minimal 3D vector implementation
#[derive(Copy, Clone, Hash, PartialEq, Eq)]
struct Vector {
    x: i32,
    y: i32,
    z: i32,
}

// Syntactic sugar to implement the `-` operator.
impl Neg for Vector {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vector { x: -self.x, y: -self.y, z: -self.z }
    }
}

/// 2D coordinates of the top left corner plus 3D vectors for the cube face.
#[derive(Clone, Copy)]
struct Face {
    corner: Point,
    i: Vector,
    j: Vector,
    k: Vector,
}

pub struct Input {
    grid: Grid,
    moves: Vec<Move>,
}

pub fn parse(input: &str) -> Input {
    let (prefix, suffix) = input.rsplit_once("\n\n").unwrap();
    let grid = parse_grid(prefix);
    let moves = parse_moves(suffix);
    Input { grid, moves }
}

pub fn part1(input: &Input) -> i32 {
    let grid = &input.grid;
    let block = grid.block;

    // Wrap around to the other side of the row or column depending on direction.
    let handle_none = |position, direction| {
        let reverse = direction * -block;
        let mut next = position + reverse;

        while grid.tile(next) != Tile::None {
            next += reverse;
        }

        next += direction;
        (next, direction)
    };

    password(input, handle_none)
}

pub fn part2(input: &Input) -> i32 {
    let grid = &input.grid;
    let block = grid.block;
    let edge = block - 1;

    // Build the cube map dynamically.
    let start = Face {
        corner: Point::new(grid.start - grid.start % block, 0),
        i: Vector { x: 1, y: 0, z: 0 },
        j: Vector { x: 0, y: 1, z: 0 },
        k: Vector { x: 0, y: 0, z: 1 },
    };
    let mut todo = VecDeque::from([start]);
    let mut faces = FastMap::build([(start.k, start)]);
    let mut corners = FastMap::build([(start.corner, start)]);

    while let Some(next) = todo.pop_front() {
        let Face { corner, i, j, k } = next;

        // Define the transitions from each edge.
        let neighbors = [
            Face { corner: corner + Point::new(-block, 0), i: -k, j, k: i }, // Left
            Face { corner: corner + Point::new(block, 0), i: k, j, k: -i },  // Right
            Face { corner: corner + Point::new(0, -block), i, j: -k, k: j }, // Up
            Face { corner: corner + Point::new(0, block), i, j: k, k: -j },  // Down
        ];

        // Potentially add the candidate edge to the frontier.
        for next in neighbors {
            if grid.tile(next.corner) != Tile::None && !faces.contains_key(&next.k) {
                todo.push_back(next);
                faces.insert(next.k, next);
                corners.insert(next.corner, next);
            }
        }
    }

    let handle_none = |position: Point, direction| {
        // Our (x, y) offset within the face.
        let offset = Point::new(position.x % block, position.y % block);
        // The (x, y) coordinate of the top left corner of the face.
        let corner = position - offset;
        // Lookup the 3D vectors associated with the current face.
        let Face { i, j, k, .. } = corners[&corner];
        // These transitions are the same as used during the BFS above.
        let next_k = match direction {
            LEFT => i,
            RIGHT => -i,
            UP => j,
            DOWN => -j,
            _ => unreachable!(),
        };
        let Face { corner: next_corner, i: next_i, j: next_j, .. } = faces[&next_k];
        // Here's the really neat part. Our new 3D direction will *always* be `k`.
        // We can find the relative orientation in the plane of the face by checking against
        // `i` and `j`. This also tells us which edge we're entering.
        let next_direction = if k == next_i {
            RIGHT
        } else if k == -next_i {
            LEFT
        } else if k == next_j {
            DOWN
        } else if k == -next_j {
            UP
        } else {
            unreachable!()
        };
        // 4 possible leaving edges and 4 possible entering edges gives 16 total possible
        // combinations.
        let next_offset = match (direction, next_direction) {
            (LEFT, LEFT) => Point::new(edge, offset.y),
            (LEFT, RIGHT) => Point::new(0, edge - offset.y),
            (LEFT, DOWN) => Point::new(offset.y, 0),
            (LEFT, UP) => Point::new(edge - offset.y, edge),
            (RIGHT, LEFT) => Point::new(edge, edge - offset.y),
            (RIGHT, RIGHT) => Point::new(0, offset.y),
            (RIGHT, DOWN) => Point::new(edge - offset.y, 0),
            (RIGHT, UP) => Point::new(offset.y, edge),
            (DOWN, LEFT) => Point::new(edge, offset.x),
            (DOWN, RIGHT) => Point::new(0, edge - offset.x),
            (DOWN, DOWN) => Point::new(offset.x, 0),
            (DOWN, UP) => Point::new(edge - offset.x, edge),
            (UP, LEFT) => Point::new(edge, edge - offset.x),
            (UP, RIGHT) => Point::new(0, offset.x),
            (UP, DOWN) => Point::new(edge - offset.x, 0),
            (UP, UP) => Point::new(offset.x, edge),
            _ => unreachable!(),
        };
        let next_position = next_corner + next_offset;
        (next_position, next_direction)
    };

    password(input, handle_none)
}

fn parse_grid(input: &str) -> Grid {
    let raw: Vec<_> = input.lines().map(str::as_bytes).collect();
    // Width is the maximum width of any row
    let width = raw.iter().map(|line| line.len()).max().unwrap();
    let height = raw.len();
    let mut tiles = vec![Tile::None; width * height];

    // Convert ASCII to enums.
    for (y, row) in raw.iter().enumerate() {
        for (x, &col) in row.iter().enumerate() {
            tiles[y * width + x] = match col {
                b'.' => Tile::Open,
                b'#' => Tile::Wall,
                _ => Tile::None,
            };
        }
    }

    // Find the first open tile in the top row.
    let start = tiles.iter().position(|&t| t == Tile::Open).unwrap() as i32;
    // Find the size of each face (4 in the sample or 50 in the actual input).
    let block = width.gcd(height) as i32;
    Grid { width, height, tiles, start, block }
}

fn parse_moves(input: &str) -> Vec<Move> {
    let mut moves = Vec::new();
    let mut numbers = input.iter_unsigned();
    let mut letters = input.bytes().filter(u8::is_ascii_uppercase);

    // Numbers and letters alternate, with numbers first.
    loop {
        let Some(n) = numbers.next() else {
            break;
        };
        moves.push(Move::Forward(n));

        let Some(d) = letters.next() else {
            break;
        };
        moves.push(if d == b'L' { Move::Left } else { Move::Right });
    }

    moves
}

/// Common code shared between part one and two. The `handle_none` closure defines how
/// to transition when we leave an edge.
fn password(input: &Input, handle_none: impl Fn(Point, Point) -> (Point, Point)) -> i32 {
    let Input { grid, moves } = input;
    let mut position = Point::new(grid.start, 0);
    let mut direction = Point::new(1, 0);

    for command in moves {
        match command {
            Move::Left => direction = direction.counter_clockwise(),
            Move::Right => direction = direction.clockwise(),
            Move::Forward(n) => {
                for _ in 0..*n {
                    let next = position + direction;
                    match grid.tile(next) {
                        // Not possible to move any further so we can break out of the loop.
                        Tile::Wall => break,
                        // Move within the 2D cube map.
                        Tile::Open => position = next,
                        Tile::None => {
                            let (next_position, next_direction) = handle_none(position, direction);
                            // The new position on a different face may be a wall.
                            if grid.tile(next_position) == Tile::Open {
                                position = next_position;
                                direction = next_direction;
                            } else {
                                break;
                            }
                        }
                    }
                }
            }
        }
    }

    // Calculate the final score.
    let position_score = 1000 * (position.y + 1) + 4 * (position.x + 1);
    let direction_score = match direction {
        RIGHT => 0,
        DOWN => 1,
        LEFT => 2,
        UP => 3,
        _ => unreachable!(),
    };
    position_score + direction_score
}
