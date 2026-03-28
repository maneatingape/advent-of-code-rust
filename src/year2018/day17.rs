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
    let clay: Vec<_> = first
        .zip(second)
        .map(|(d, [a, b, c])| if d == b'x' { (a, a, b, c) } else { (b, c, a, a) })
        .collect();

    // Find boundaries of the 2D scan.
    let min_x = clay.iter().map(|c| c.0).min().unwrap();
    let max_x = clay.iter().map(|c| c.1).max().unwrap();
    let min_y = clay.iter().map(|c| c.2).min().unwrap();
    let max_y = clay.iter().map(|c| c.3).max().unwrap();

    // Leave room for water on either side.
    let width = max_x - min_x + 3;
    let top = width * min_y;
    let bottom = width * (max_y + 1);
    let mut kind = vec![Sand; bottom];

    // Draw each of the clay veins.
    for (x1, x2, y1, y2) in clay {
        if x1 == x2 {
            for y in y1..y2 + 1 {
                kind[width * y + x1 - min_x + 1] = Stopped;
            }
        } else {
            for x in x1..x2 + 1 {
                kind[width * y1 + x - min_x + 1] = Stopped;
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
        let left = (0..index).rev().find(|&i| !spread(scan, i)).unwrap();
        let right = (index + 1..scan.bottom).find(|&i| !spread(scan, i)).unwrap();

        if scan.kind[left] == Stopped && scan.kind[right] == Stopped {
            scan.kind[left + 1..right].fill(Stopped);
            scan.stopped += right - left - 1;
            Stopped
        } else {
            // Open on one or both sides, continue to flow off the sides.
            flow(scan, left);
            flow(scan, right);
            scan.kind[left + 1..right].fill(Moving);
            if index >= scan.top {
                scan.moving += right - left - 1;
            }
            Moving
        }
    }
}

/// Check the neighboring horizontal tile first to see if we can keep spreading, then check the
/// tile underneath. Speed things up by first checking via the middle condition if the tile
/// is already stopped water or clay.
fn spread(scan: &mut Scan, index: usize) -> bool {
    scan.kind[index] == Sand
        && (scan.kind[index + scan.width] == Stopped || flow(scan, index + scan.width) == Stopped)
}
