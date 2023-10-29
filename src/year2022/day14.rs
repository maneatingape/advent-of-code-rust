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
//! neighbors until we have a conclusive result then propagating that back up the call stack,
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
//! * `Falling` Grains of sand that will continue to fall continously forever.
//! * `Stopped` Both original rock walls and any grains of sand that have come to rest.
use crate::util::parse::*;

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
    size: usize,
    kind: Vec<Kind>,
    floor: Kind,
    count: u32,
}

impl Cave {
    fn fall(&mut self, index: usize) -> Kind {
        // Check in order: center, left then right
        let result = self.check(index + self.width)
            && self.check(index + self.width - 1)
            && self.check(index + self.width + 1);

        // If all 3 bottom neighbors are stopped then so are we.
        // Cache the result into the grid then propagate result back up the call stack.
        if result {
            self.count += 1;
            self.kind[index] = Kind::Stopped;
            Kind::Stopped
        } else {
            self.kind[index] = Kind::Falling;
            Kind::Falling
        }
    }

    // Returns `true` if cell is stopped.
    fn check(&mut self, index: usize) -> bool {
        let kind = if index >= self.size {
            // If we've reached the "floor" then return that.
            self.floor
        } else if self.kind[index] == Kind::Air {
            // If we're unknown then recursively check our own underneath neighbors
            self.fall(index)
        } else {
            // Otherwise use the cached value.
            self.kind[index]
        };
        kind == Kind::Stopped
    }
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
    let size = width * height;
    let mut kind = vec![Kind::Air; size];

    // Draw each of the walls.
    for row in points {
        for window in row.windows(4).step_by(2) {
            if let &[x1, y1, x2, y2] = window {
                for x in x1.min(x2)..=x1.max(x2) {
                    for y in y1.min(y2)..=y1.max(y2) {
                        kind[(width * y) + (x + height - 500)] = Kind::Stopped;
                    }
                }
            }
        }
    }

    Cave { width, height, size, kind, floor: Kind::Air, count: 0 }
}

/// If a grain of sand reaches the floor it will fall forever.
pub fn part1(input: &Cave) -> u32 {
    simulate(input, Kind::Falling)
}

/// The floor is solid rock.
pub fn part2(input: &Cave) -> u32 {
    simulate(input, Kind::Stopped)
}

fn simulate(input: &Cave, floor: Kind) -> u32 {
    let mut cave = input.clone();
    cave.floor = floor;
    // Height is also the x coordinate of the central starting location for grains.
    cave.fall(cave.height);
    cave.count
}
