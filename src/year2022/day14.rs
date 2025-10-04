//! # Regolith Reservoir
//!
//! We could simulate each grain of sand falling one at a time, for example:
//!
//! ```none
//!     #o#    # #    # #    # #
//!     # #    #o#    # #    # #
//!     # # => # # => #o# => # #
//!     # #    # #    # #    #o#
//!     ###    ###    ###    ###
//! ```
//!
//! In this example it would take 4 steps to simulate the first grain, 3 steps for the second
//! and so on. More generally it would take `∑n` = `n * (n + 1) / 2` or `O(n²)` complexity for
//! the whole pile.
//!
//! We instead simulate in `O(n)` complexity by recursively check each grain's underneath
//! neighbors until we have a conclusive result then propagating that back up the stack,
//! for example:
//!
//! ```none
//!     #?#    #?#    #?#    #?#    #?#    #?#    #?#    #o#
//!     # #    #?#    #?#    #?#    #?#    #?#    #o#    #o#
//!     # # => # # => #?# => #?# => #?# => #o# => #o# => #o#
//!     # #    # #    # #    #?#    #o#    #o#    #o#    #o#
//!     ###    ###    ###    ###    ###    ###    ###    ###
//! ```
//!
//! We model the cave as a grid in the possible states:
//! * `Air` Empty blocks, treated as unknown status when checking underneath neighbors.
//! * `Falling` Grains of sand that will continue to fall continuously forever.
//! * `Stopped` Both original rock walls and any grains of sand that have come to rest.
use crate::util::parse::*;
use Kind::*;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Kind {
    Air,
    Falling,
    Stopped,
}

#[derive(Clone)]
pub struct Cave {
    width: usize,
    height: usize,
    kind: Vec<Kind>,
}

/// Creates a 2D grid cave exactly the maximum possible size.
pub fn parse(input: &str) -> Cave {
    let unsigned = |line: &str| line.iter_unsigned().collect();
    let points: Vec<Vec<usize>> = input.lines().map(unsigned).collect();
    let max_y = points.iter().flat_map(|row| row.iter().skip(1).step_by(2)).max().unwrap();

    // Floor is 2 below the bottommost wall.
    let height = max_y + 2;
    // Allow enough horizontal room to spread out.
    let width = 2 * height + 1;

    // Draw each of the walls.
    let mut kind = vec![Air; width * height];

    for row in points {
        for window in row.windows(4).step_by(2) {
            let &[x1, y1, x2, y2] = window else { unreachable!() };

            if x1 == x2 {
                for y in y1.min(y2)..=y1.max(y2) {
                    kind[width * y + x1 + height - 500] = Stopped;
                }
            } else {
                for x in x1.min(x2)..=x1.max(x2) {
                    kind[width * y1 + x + height - 500] = Stopped;
                }
            }
        }
    }

    Cave { width, height, kind }
}

/// If a grain of sand reaches the floor it will fall forever.
pub fn part1(input: &Cave) -> u32 {
    simulate(input, Falling)
}

/// The floor is solid rock.
pub fn part2(input: &Cave) -> u32 {
    simulate(input, Stopped)
}

fn simulate(input: &Cave, floor: Kind) -> u32 {
    let Cave { width, height, mut kind } = input.clone();
    let mut count = 0;

    // Height is also the x coordinate of the central starting location for grains.
    let mut todo = Vec::with_capacity(1_000);
    todo.push(height);

    'outer: while let Some(index) = todo.pop() {
        // Check in order: center, left then right
        for next in [index + width, index + width - 1, index + width + 1] {
            // If we've reached the "floor" then return that.
            let tile = if next >= kind.len() { floor } else { kind[next] };

            match tile {
                // If we're unknown then check underneath neighbors first then re-check this tile.
                Air => {
                    todo.push(index);
                    todo.push(next);
                    continue 'outer;
                }
                // Any falling tile underneath means that this tile is also falling.
                Falling => {
                    kind[index] = Falling;
                    continue 'outer;
                }
                Stopped => (),
            }
        }

        // If all 3 tiles underneath are stopped then this tile is also stopped.
        kind[index] = Stopped;
        count += 1;
    }

    count
}
