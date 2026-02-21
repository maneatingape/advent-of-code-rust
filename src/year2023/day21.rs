//! # Step Counter
//!
//! This solution uses a geometric approach. Looking at the input data reveals several crucial
//! insights:
//!
//! * The sample data is a decoy and will not work with this solution.
//! * The real input data has two special properties:
//!     * Vertical and horizontal "roads" run from the center.
//!     * The edge of the input is completely free of obstructions.
//!
//! These properties mean that we can always cross a tile in exactly 131 steps. We start in the
//! middle of a tile and need 65 steps to reach the edge. Part two asks how many plots can be
//! reached in 26501365 steps.
//!
//! ```none
//!     26501365 => 65 + 131 * n => n = 202300
//! ```
//!
//! The number of tiles that we can reach forms a rough diamond 202300 tiles wide.
//! For example `n = 2` looks like:
//!
//! ```none
//!       #
//!      ###
//!     #####
//!      ###
//!       #
//! ```
//!
//! The next insight is that if we can reach a plot in `x` steps then we can also reach it in
//! `x + 2, x + 4...` steps by repeatedly stepping back and forth 1 tile. This means the
//! number of tiles reachable depends on the *parity* of a plot from the center,
//! i.e. whether it is an odd or even number of steps. As the 131 width of the tile is an odd
//! number of plots, the number of plots reachable flips from odd to even each time we cross a
//! whole tile. There are `n²` even plots and `(n + 1)²` odd plots in the diamond.
//!
//! ```none
//!       O
//!      OEO
//!     OEOEO
//!      OEO
//!       O
//! ```
//!
//! Lastly, we can only partially reach some tiles on the edges. Solid triangles represent corners
//! that can be reached and hollow triangle represents corners that are too far away.
//!
//! ```none
//!          ┌--┐
//!          |◸◹|
//!         ◢|  |◣
//!       ┌--┼--┼--┐
//!       |◸ |  | ◹|
//!      ◢|  |  |  |◣
//!    ┌--┼--┼--┼--┼--┐
//!    |◸ |  |  |  | ◹|
//!    |◺ |  |  |  | ◿|
//!    └--┼--┼--┼--┼--┘
//!      ◥|  |  |  |◤
//!       |◺ |  | ◿|
//!       └--┼--┼--┘
//!         ◥|  |◤
//!          |◺◿|
//!          └--┘
//! ```
//!
//! The total area is adjusted by:
//! * Adding `n` extra even corners
//!     ```none
//!         ◤◥
//!         ◣◢
//!     ```
//! * Subtracting `n + 1` odd corners
//!     ```none
//!         ◸◹
//!         ◺◿
//!     ```
//!
//! To find the values for the total number of odd, even plots and the unreachable odd corners
//! we BFS from the center tile, counting odd and even plots separately. Any plots more than
//! 65 steps from the center will be unreachable at the edges of the diamond.
//!
//! One nuance is that to always correctly find the extra reachable even corner plots requires a
//! *second* BFS starting from the corners and working inwards. All tiles within 64 steps are
//! reachable at the edges of the diamond. For some inputs this happens to be the same as the number
//! of tiles greater than 65 steps from the center by coincidence, however this is not guaranteed so
//! a second BFS is more reliable solution.
use crate::util::grid::*;
use crate::util::point::*;
use std::collections::VecDeque;

const CENTER: Point = Point::new(65, 65);
const CORNERS: [Point; 4] =
    [Point::new(0, 0), Point::new(130, 0), Point::new(0, 130), Point::new(130, 130)];

type Input = (u64, u64);

pub fn parse(input: &str) -> Input {
    let grid = Grid::parse(input);

    // Search from the center tile outwards.
    let (even_inner, even_outer, odd_inner, odd_outer) = bfs(&grid, &[CENTER], 130);
    let part_one = even_inner;
    let even_full = even_inner + even_outer;
    let odd_full = odd_inner + odd_outer;
    let remove_corners = odd_outer;

    // Search from the 4 corners inwards.
    let (even_inner, ..) = bfs(&grid, &CORNERS, 64);
    let add_corners = even_inner;

    // Sum the components of the diamond.
    let n = 202300;
    let first = n * n * even_full;
    let second = (n + 1) * (n + 1) * odd_full;
    let third = n * add_corners;
    let fourth = (n + 1) * remove_corners;
    let part_two = first + second + third - fourth;

    (part_one, part_two)
}

pub fn part1(input: &Input) -> u64 {
    input.0
}

pub fn part2(input: &Input) -> u64 {
    input.1
}

/// Breadth-first search from any number of starting locations with a limit on maximum steps.
fn bfs(grid: &Grid<u8>, starts: &[Point], limit: u32) -> (u64, u64, u64, u64) {
    let mut grid = grid.clone();
    let mut todo = VecDeque::new();

    let mut even_inner = 0;
    let mut even_outer = 0;
    let mut odd_inner = 0;
    let mut odd_outer = 0;

    for &start in starts {
        grid[start] = b'#';
        todo.push_back((start, 0));
    }

    while let Some((position, cost)) = todo.pop_front() {
        // First split by odd or even parity then by distance from the starting point.
        if cost % 2 == 1 {
            if position.manhattan(CENTER) <= 65 {
                odd_inner += 1;
            } else {
                odd_outer += 1;
            }
        } else if cost <= 64 {
            even_inner += 1;
        } else {
            even_outer += 1;
        }

        if cost < limit {
            for next in ORTHOGONAL.map(|o| position + o) {
                if grid.contains(next) && grid[next] != b'#' {
                    grid[next] = b'#';
                    todo.push_back((next, cost + 1));
                }
            }
        }
    }

    (even_inner, even_outer, odd_inner, odd_outer)
}
