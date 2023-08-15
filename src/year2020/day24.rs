//! # Lobby Layout
//!
//! Hex grid parsing and navigation uses
//! [Axial Coordinates](https://www.redblobgames.com/grids/hexagons/#coordinates-cube)
//! exactly as described in the excellent [Red Blob Games](https://www.redblobgames.com/) blog.
//!
//! Part two uses exactly the same approach as [`day 17`] and most of the code is identical.
//!
//! As the black tiles are very sparse (about 8% for my input) it's faster to switch from
//! a "pull" model where we check the surroundings neighbors of each tile, to a "push" model
//! where we update the neighbors of each black tile instead.
//!
//! [`day 17`]: crate::year2020::day17
use crate::util::hash::*;
use std::array::from_fn;

#[derive(PartialEq, Eq, Hash)]
pub struct Hex {
    q: i32,
    r: i32,
}

pub fn parse(input: &str) -> FastSet<Hex> {
    let mut tiles = FastSet::new();

    for line in input.lines() {
        let mut iter = line.bytes();
        let mut q = 0;
        let mut r = 0;

        while let Some(b) = iter.next() {
            match b {
                b'e' => q += 1,
                b'w' => q -= 1,
                b'n' => {
                    if b'e' == iter.next().unwrap() {
                        q += 1;
                        r -= 1;
                    } else {
                        r -= 1;
                    }
                }
                b's' => {
                    if b'e' == iter.next().unwrap() {
                        r += 1;
                    } else {
                        q -= 1;
                        r += 1;
                    }
                }
                _ => unreachable!(),
            }
        }

        let tile = Hex { q, r };
        if tiles.contains(&tile) {
            tiles.remove(&tile);
        } else {
            tiles.insert(tile);
        }
    }

    tiles
}

pub fn part1(input: &FastSet<Hex>) -> usize {
    input.len()
}

pub fn part2(input: &FastSet<Hex>) -> usize {
    // Determine bounds
    let mut q1 = i32::MAX;
    let mut q2 = i32::MIN;
    let mut r1 = i32::MAX;
    let mut r2 = i32::MIN;

    for hex in input {
        q1 = q1.min(hex.q);
        q2 = q2.max(hex.q);
        r1 = r1.min(hex.r);
        r2 = r2.max(hex.r);
    }

    // Create array with enough space to allow expansion for 100 generations.
    // 2 * (100 generations + 1 buffer) + Origin = 203 extra in each dimension
    let width = q2 - q1 + 203;
    let height = r2 - r1 + 203;
    let neighbors: [i32; 6] = [-1, 1, -width, width, 1 - width, width - 1];
    let neighbors: [usize; 6] = from_fn(|i| neighbors[i] as usize);

    let mut active = Vec::with_capacity(5_000);
    let mut candidates = Vec::with_capacity(5_000);
    let mut next_active = Vec::with_capacity(5_000);

    // Create initial active state, offsetting tiles so that all indices are positive.
    for hex in input {
        let index = width * (hex.r - r1 + 101) + (hex.q - q1 + 101);
        active.push(index as usize);
    }

    for _ in 0..100 {
        let mut state: Vec<u8> = vec![0; (width * height) as usize];

        for &tile in &active {
            for &offset in &neighbors {
                // Earlier we converted the offsets from signed `i32` to unsigned `usize`. To
                // achieve subtraction for negative indices, we use a `wrapping_add` that performs
                // [two's complement](https://en.wikipedia.org/wiki/Two%27s_complement) arithmetic.
                let index = tile.wrapping_add(offset);
                state[index] += 1;

                if state[index] == 2 {
                    candidates.push(index);
                }
            }
        }

        // Active tiles remain active with both one and two neighbors.
        for &tile in &active {
            if state[tile] == 1 {
                next_active.push(tile);
            }
        }

        // Check that the neighbor count for inactive tiles hasn't exceeded two.
        for &tile in &candidates {
            if state[tile] == 2 {
                next_active.push(tile);
            }
        }

        // Swap to make next generation the current generation.
        (active, next_active) = (next_active, active);
        candidates.clear();
        next_active.clear();
    }

    active.len()
}
