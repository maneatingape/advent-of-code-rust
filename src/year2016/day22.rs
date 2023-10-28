//! # Grid Computing
//!
//! Part two is a decoy that can be solved in constant time with some analysis.
//! Printing the actual node layout shows a structure similar to:
//!
//! ```none
//!     O......G
//!     ........
//!     ..######
//!     ........
//!     .....-..
//! ```
//!
//! * `O` is our destination
//! * `G` is the data
//! * `#` are large nodes that can't be moved to neighbours, effectively acting as walls.
//! * `-` is the empty node.
//!
//! First we move the empty spot in front of the data:
//!
//! ```none
//!     O>>>>>>G
//!     .^......
//!     .^######
//!     .^......
//!     .<<<<-..
//! ```
//!
//! Then we move the data into the empty spot.
//!
//! ```none
//!     O.....G_
//!     ........
//!     ..######
//!     ........
//!     ........
//! ```
//!
//! Finally we move the data to the origin by repeating the same sequence of 5 moves.
//! First moving the empty spot back around to in front of the data in 4 moves.
//!
//! ```none
//!     O....^G_
//!     .....^<v
//!     ..######
//!     ........
//!     ........
//! ```
//!
//! Then moving the data another spot to the left.
//!
//! ```none
//!     O....G_.
//!     ........
//!     ..######
//!     ........
//!     ........
//! ```
//!
//! To find the minimum number of steps we only need to find the `(x, y)` coordinates of the empty
//! spot and the width of the wall, then add up the sequence of moves.
use crate::util::iter::*;
use crate::util::parse::*;

pub struct Node {
    x: u32,
    y: u32,
    used: u32,
    avail: u32,
}

pub fn parse(input: &str) -> Vec<Node> {
    input
        .iter_unsigned()
        .chunk::<6>()
        .map(|[x, y, _, used, avail, _]| Node { x, y, used, avail })
        .collect()
}

/// Filter the used and available space in ascending order to find the viable pairs efficiently.
pub fn part1(input: &[Node]) -> usize {
    let mut used: Vec<_> = input.iter().map(|n| n.used).filter(|&n| n > 0).collect();
    used.sort_unstable();

    let mut avail: Vec<_> = input.iter().map(|n| n.avail).collect();
    avail.sort_unstable();

    let mut i = 0;
    let mut viable = 0;

    for next in used {
        while i < avail.len() && avail[i] < next {
            i += 1;
        }
        viable += avail.len() - i;
    }

    viable
}

pub fn part2(input: &[Node]) -> u32 {
    let mut width = 0;
    let mut empty_x = 0;
    let mut empty_y = 0;
    let mut wall_x = u32::MAX;

    for &Node { x, y, used, .. } in input {
        width = width.max(x + 1);

        if used == 0 {
            empty_x = x;
            empty_y = y;
        }

        // Large nodes are bigger than 100T.
        if used >= 100 {
            wall_x = wall_x.min(x - 1);
        }
    }

    // Move left to avoid wall.
    let a = empty_x - wall_x;
    // Move up to first row.
    let b = empty_y;
    // Move right to spot in front of data.
    let c = width - 2 - wall_x;
    // Move data into empty spot.
    let d = 1;
    // Repeatedly move empty spot 4 places around from behind data then move data one spot left.
    let e = 5 * (width - 2);

    a + b + c + d + e
}
