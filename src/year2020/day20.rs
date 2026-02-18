//! # Jurassic Jigsaw
//!
//! At first this seems like a daunting problem. However a little analysis shows that the input
//! has some nice properties that makes solving this more tractable.
//!
//! * Tile edges match with at most one other tile
//! * The forward and reverse tile edges form two distinct sets of 312 values with no overlap.
//!
//! Tiles can be flipped and rotated for a total of 8 possible permutations each. When parsing
//! the tiles we store all 8 edge possibilities to enable assembling the jigsaw in part two. For
//! performance we avoid transforming the inner 8x8 pixels until we have determined the
//! layout of the grid.

//!
//! ## Part One
//!
//! First we calculate the frequency of each edge, both forward and backward as tiles can be in
//! any orientation. As there are only 2¹⁰ or 1024 possible edge values we can use an array instead of a
//! hash table for speed, converting the edges into a binary number to index the array.
//!
//! This results in 96 values that occur once and 528 values that occur twice. Then for every tile
//! we sum the frequency of each edge. Corner tiles will have two edges that only occur once, not
//! matching with any other tile, for a total of 1 + 1 + 2 + 2 = 6.
//!
//! Other edge tiles have a total of 1 + 2 + 2 + 2 = 7 and inner tiles a total of 2 + 2 + 2 + 2 = 8.
//!
//! ## Part Two
//!
//! First we arbitrarily pick any corner tile that is oriented so that its unique edges are facing
//! top and left. Then we proceed row by row, looking up the next tile to the right. Each time
//! we find a tile we remove it from the remaining tiles, so that looking up a tile is always a
//! very fast constant time `O(1)` operation.
//!
//! The complete picture is stored as an array of `u128` values as the tiles form a square 12 wide,
//! for a total of 12 * 8 = 96 pixels. As we add each tile, we convert its pixels into a `u8` binary
//! number and left shift to add to the existing pixels.
//!
//! When finding the monsters we make some further assumptions about the input:
//!
//! * The monsters will all be oriented the same way
//! * Monsters will not overlap with each other
//!
//! For speed the monster bit patterns are rotated and flipped instead of the image, then stored
//! in hardcoded arrays. The search ends as soon as we find monsters in any orientation.
use crate::util::parse::*;
use std::array::from_fn;

pub struct Tile {
    id: u64,
    top: [usize; 8],
    left: [usize; 8],
    bottom: [usize; 8],
    right: [usize; 8],
    pixels: [[u8; 10]; 10],
}

impl Tile {
    // O = Original
    // H = Flip horizontal
    // V = Flip vertical
    // R = Rotate clockwise 90 degrees
    // Sequence: [O, H, V, HV, R, RH, RV, RHV]
    const COEFFICIENTS: [[i32; 6]; 8] = [
        [1, 0, 1, 0, 1, 1],
        [-1, 0, 8, 0, 1, 1],
        [1, 0, 1, 0, -1, 8],
        [-1, 0, 8, 0, -1, 8],
        [0, 1, 1, -1, 0, 8],
        [0, 1, 1, 1, 0, 1],
        [0, -1, 8, -1, 0, 8],
        [0, -1, 8, 1, 0, 1],
    ];

    fn from(chunk: &[&str]) -> Tile {
        let id = (&chunk[0][5..9]).unsigned();

        let pixels: [[u8; 10]; 10] = from_fn(|i| chunk[i + 1].as_bytes().try_into().unwrap());

        // The ASCII code for "#" 35 is odd and the code for "." 46 is even
        // so we can convert to a 1 or 0 bit using bitwise AND with 1.
        let binary = |row: usize, col: usize| (pixels[row][col] & 1) as usize;
        let mut t = 0;
        let mut l = 0;
        let mut b = 0;
        let mut r = 0;

        for i in 0..10 {
            t = (t << 1) | binary(0, i);
            l = (l << 1) | binary(i, 0);
            b = (b << 1) | binary(9, i);
            r = (r << 1) | binary(i, 9);
        }

        let reverse = |edge: usize| edge.reverse_bits() >> 54;
        let rt = reverse(t);
        let rl = reverse(l);
        let rb = reverse(b);
        let rr = reverse(r);

        // Same transform sequence as coefficients:
        // [O, H, V, HV, R, RH, RV, RHV]
        let top = [t, rt, b, rb, rl, l, rr, r];
        let left = [l, r, rl, rr, b, t, rb, rt];
        let bottom = [b, rb, t, rt, rr, r, rl, l];
        let right = [r, l, rr, rl, t, b, rt, rb];

        Tile { id, top, left, bottom, right, pixels }
    }

    // Coefficients allow us to reuse the loop logic for each of the 8 possible permutations.
    fn transform(&self, image: &mut [u128], permutation: usize) {
        let [a, b, c, d, e, f] = Self::COEFFICIENTS[permutation];

        for row in 0..8 {
            let mut acc = 0;

            for col in 0..8 {
                let x = a * col + b * row + c;
                let y = d * col + e * row + f;
                let b = self.pixels[y as usize][x as usize];
                acc = (acc << 1) | (b & 1);
            }

            image[row as usize] = (image[row as usize] << 8) | (acc as u128);
        }
    }
}

pub fn parse(input: &str) -> Vec<Tile> {
    let lines: Vec<_> = input.lines().collect();
    lines.chunks(12).map(Tile::from).collect()
}

pub fn part1(input: &[Tile]) -> u64 {
    let mut freq = [0; 1024];
    let mut result = 1;

    for tile in input {
        for edge in tile.top {
            freq[edge] += 1;
        }
    }

    for tile in input {
        // Any orientation will do, pick the first.
        let total =
            freq[tile.top[0]] + freq[tile.left[0]] + freq[tile.bottom[0]] + freq[tile.right[0]];
        if total == 6 {
            result *= tile.id;
        }
    }

    result
}

pub fn part2(input: &[Tile]) -> u32 {
    // Store mapping of tile edges to tile index in order to allow
    // constant time lookup by edge when assembling the jigsaw.
    let mut edge_to_tile = [[0; 2]; 1024];
    let mut freq = [0; 1024];
    let mut placed = [false; 1024];

    for (i, tile) in input.iter().enumerate() {
        for edge in tile.top {
            edge_to_tile[edge][freq[edge]] = i;
            freq[edge] += 1;
        }
    }

    let mut find_arbitrary_corner = || {
        for tile in input {
            for j in 0..8 {
                if freq[tile.top[j]] == 1 && freq[tile.left[j]] == 1 {
                    freq[tile.top[j]] += 1;
                    return tile.top[j];
                }
            }
        }
        unreachable!();
    };
    let mut find_matching_tile = |edge: usize| {
        let [first, second] = edge_to_tile[edge];
        let next = if placed[first] { second } else { first };
        placed[next] = true;
        &input[next]
    };

    // Assemble the image
    let mut next_top = find_arbitrary_corner();
    let mut image = [0; 96];
    let mut index = 0;

    while freq[next_top] == 2 {
        let tile = find_matching_tile(next_top);
        let permutation = (0..8).position(|i| tile.top[i] == next_top).unwrap();
        tile.transform(&mut image[index..], permutation);
        next_top = tile.bottom[permutation];

        let mut next_left = tile.right[permutation];

        while freq[next_left] == 2 {
            let tile = find_matching_tile(next_left);
            let permutation = (0..8).position(|i| tile.left[i] == next_left).unwrap();
            tile.transform(&mut image[index..], permutation);
            next_left = tile.right[permutation];
        }

        index += 8;
    }

    // Common search logic
    let sea: u32 = image.iter().map(|n| n.count_ones()).sum();
    let find = |monster: &mut [u128], width: usize, height: usize| {
        let mut rough = sea;

        for _ in 0..(96 - width + 1) {
            for window in image.windows(height) {
                if monster.iter().enumerate().all(|(i, &n)| n & window[i] == n) {
                    rough -= 15;
                }
            }
            monster.iter_mut().for_each(|n| *n <<= 1);
        }

        (rough < sea).then_some(rough)
    };

    // Transform the monsters instead of the image.
    // Hardcoded bit patterns for [O, H, V, HV].
    let mut monsters = [
        [0b00000000000000000010, 0b10000110000110000111, 0b01001001001001001000],
        [0b01001001001001001000, 0b10000110000110000111, 0b00000000000000000010],
        [0b01000000000000000000, 0b11100001100001100001, 0b00010010010010010010],
        [0b00010010010010010010, 0b11100001100001100001, 0b01000000000000000000],
    ];

    for monster in &mut monsters {
        if let Some(rough) = find(monster, 20, 3) {
            return rough;
        }
    }

    // Hardcoded bit patterns [R, RH, RV, RHV].
    let mut monsters = [
        [2, 4, 0, 0, 4, 2, 2, 4, 0, 0, 4, 2, 2, 4, 0, 0, 4, 2, 3, 2],
        [2, 3, 2, 4, 0, 0, 4, 2, 2, 4, 0, 0, 4, 2, 2, 4, 0, 0, 4, 2],
        [2, 1, 0, 0, 1, 2, 2, 1, 0, 0, 1, 2, 2, 1, 0, 0, 1, 2, 6, 2],
        [2, 6, 2, 1, 0, 0, 1, 2, 2, 1, 0, 0, 1, 2, 2, 1, 0, 0, 1, 2],
    ];

    for monster in &mut monsters {
        if let Some(rough) = find(monster, 3, 20) {
            return rough;
        }
    }

    unreachable!()
}
