//! # Reservoir Research
//!
//! Starting from the spring, recursively works out the kind of each tile, memoizing values for
//! efficiency. Tiles are one of 3 kinds:
//!
//! * `Sand` Indicates a tile of unknown type.
//! * `Moving` Flowing water.
//! * `Stopped` Either clay tile or water that has settled.
//!
//! This problem is similar to [Year 2022 Day 14].
//!
//! [Year 2022 Day 14]: crate::year2022::day14
use crate::util::iter::*;
use crate::util::parse::*;
use Kind::*;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Kind {
    Sand,
    Moving,
    Stopped,
}

pub struct Scan {
    width: usize,
    top: usize,
    bottom: usize,
    kind: Vec<Kind>,
    moving: usize,
    stopped: usize,
}

pub fn parse(input: &str) -> Scan {
    let first = input.lines().map(|line| line.as_bytes()[0]);
    let second = input.iter_unsigned::<usize>().chunk::<3>();
    let clay: Vec<_> = first.zip(second).collect();

    // Find boundaries of the 2D scan.
    let mut min_x = usize::MAX;
    let mut max_x = 0;
    let mut min_y = usize::MAX;
    let mut max_y = 0;

    for &(direction, triple) in &clay {
        let (x1, x2, y1, y2) = if direction == b'x' {
            let [x, y1, y2] = triple;
            (x, x, y1, y2)
        } else {
            let [y, x1, x2] = triple;
            (x1, x2, y, y)
        };

        min_x = min_x.min(x1);
        max_x = max_x.max(x2);
        min_y = min_y.min(y1);
        max_y = max_y.max(y2);
    }

    // Leave room for water on either side.
    let width = max_x - min_x + 3;
    let top = width * min_y;
    let bottom = width * (max_y + 1);
    let mut kind = vec![Sand; bottom];

    // Draw each of the clay veins.
    for (direction, triple) in clay {
        if direction == b'x' {
            let [x, y1, y2] = triple;
            for y in y1..y2 + 1 {
                kind[(width * y) + (x - min_x + 1)] = Stopped;
            }
        } else {
            let [y, x1, x2] = triple;
            for x in x1..x2 + 1 {
                kind[(width * y) + (x - min_x + 1)] = Stopped;
            }
        }
    }

    let mut scan = Scan { width, top, bottom, kind, moving: 0, stopped: 0 };
    flow(&mut scan, 500 - min_x + 1);
    scan
}

pub fn part1(input: &Scan) -> usize {
    input.moving + input.stopped
}

pub fn part2(input: &Scan) -> usize {
    input.stopped
}

/// Recursively work out the kind of each tile, memoizing values for efficiency.
fn flow(scan: &mut Scan, index: usize) -> Kind {
    if index >= scan.bottom {
        // Water has gone past the lowest clay tiles, so will fall for infinity.
        Moving
    } else if scan.kind[index] != Sand {
        // Return memoized value.
        scan.kind[index]
    } else if flow(scan, index + scan.width) == Moving {
        // Tile underneath is moving, so this tile must be moving too.
        scan.kind[index] = Moving;
        if index >= scan.top {
            scan.moving += 1;
        }
        Moving
    } else {
        // Tile is stopped (either clay or still water) so water flows both left and right.
        let mut left = index;
        let mut right = index;

        while scan.kind[left - 1] == Sand && flow(scan, left + scan.width) == Stopped {
            left -= 1;
        }

        while scan.kind[right + 1] == Sand && flow(scan, right + scan.width) == Stopped {
            right += 1;
        }

        if scan.kind[left - 1] == Stopped && scan.kind[right + 1] == Stopped {
            for index in left..right + 1 {
                scan.kind[index] = Stopped;
            }
            if index >= scan.top {
                scan.stopped += right + 1 - left;
            }
            Stopped
        } else {
            for index in left..right + 1 {
                scan.kind[index] = Moving;
            }
            if index >= scan.top {
                scan.moving += right + 1 - left;
            }
            Moving
        }
    }
}
