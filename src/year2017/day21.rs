//! # Fractal Art
//!
//! The image size starts at 3x3, growing exponentially to 18x18 after 5 generations and 2187x2187
//! after 18 generations. The first insight to solving efficiently is realizing that we don't need
//! to compute the entire image, instead only the *count* of each pattern is needed. Multiplying
//! the count of each pattern by the number of set bits in each pattern gives the result.
//!
//! The second insight is that after 3 generations, the 9x9 image can be split into nine 3x3
//! images that are independent of each other and the enhancement cycle can start over.
//! Interestingly most of the 3x3 patterns in the input are not needed, only the starting 3x3
//! pattern and the six 2x2 to 3x3 patterns.
//!
//! Adding a few extra made up rules:
//!
//! ```none
//!     ##/#. => ###/#.#/###
//!     .#/.# => .#./###/.#.
//!     ../.. => #.#/.#./#.#
//! ```
//!
//! then using the example:
//!
//! ```none
//!     .#.    #..#    ##.##.    ###|.#.|##.
//!     ..# => .... => #..#.. => #.#|###|#..
//!     ###    ....    ......    ###|.#.|...
//!            #..#    ##.##.    ---+---+---
//!                    #..#..    .#.|##.|##.
//!                    ......    ###|#..|#..
//!                              .#.|...|...
//!                              ---+---+---
//!                              ##.|##.|#.#
//!                              #..|#..|.#.
//!                              ...|...|#.#
//! ```
//!
//! Splitting the 9x9 grid results in:
//!
//! ```none
//!
//!     1 x ###    2 x .#.    5 x ##.    1 x #.#
//!         # #        ###        #..        .#.
//!         ###        .#.        ...        #.#
//! ```
//!
//! The enhancement cycle can start again with each 3x3 image. This means that we only need to
//! calculate 2 generations for the starting image and each 2x2 to 3x3 rule.
struct Pattern {
    three: u32,
    four: u32,
    six: u32,
    nine: [usize; 9],
}

pub fn parse(input: &str) -> Vec<u32> {
    // 2⁴ = 16 possible 2x2 patterns
    let mut pattern_lookup = [0; 16];
    let mut two_to_three = [[0; 9]; 16];
    // 2⁹ = 512 possible 3x3 patterns
    let mut three_to_four = [[0; 16]; 512];

    // Starting pattern .#./..#/### => 010/001/111 => b010001111 => 143
    let mut todo = vec![143];

    for line in input.lines().map(str::as_bytes) {
        // The ASCII code for "#" 35 is odd and the code for "." 46 is even
        // so we can convert to a 1 or 0 bit using bitwise AND with 1.
        let bit = |i: usize| line[i] & 1;

        if line.len() == 20 {
            // 2x2 to 3x3
            let indices = [0, 1, 3, 4];
            let from = indices.map(bit);

            let indices = [9, 10, 11, 13, 14, 15, 17, 18, 19];
            let value = indices.map(bit);

            let pattern = todo.len();
            todo.push(to_index(&value));

            for key in two_by_two_permutations(from) {
                two_to_three[key] = value;
                pattern_lookup[key] = pattern;
            }
        } else {
            // 3x3 to 4x4
            let indices = [0, 1, 2, 4, 5, 6, 8, 9, 10];
            let from = indices.map(bit);

            let indices = [15, 16, 17, 18, 20, 21, 22, 23, 25, 26, 27, 28, 30, 31, 32, 33];
            let value = indices.map(bit);

            for key in three_by_three_permutations(from) {
                three_to_four[key] = value;
            }
        }
    }

    let patterns: Vec<_> = todo
        .iter()
        .map(|&index| {
            // Lookup 4x4 pattern then map to 6x6
            let four = three_to_four[index];
            let mut six = [0; 36];

            for (src, dst) in [(0, 0), (2, 3), (8, 18), (10, 21)] {
                let index = to_index(&[four[src], four[src + 1], four[src + 4], four[src + 5]]);
                let replacement = two_to_three[index];
                six[dst..dst + 3].copy_from_slice(&replacement[0..3]);
                six[dst + 6..dst + 9].copy_from_slice(&replacement[3..6]);
                six[dst + 12..dst + 15].copy_from_slice(&replacement[6..9]);
            }

            // Map 6x6 pattern to nine 3x3 patterns.
            let nine = [0, 2, 4, 12, 14, 16, 24, 26, 28].map(|i| {
                let index = to_index(&[six[i], six[i + 1], six[i + 6], six[i + 7]]);
                pattern_lookup[index]
            });

            let three = index.count_ones();
            let four = four.iter().sum::<u8>() as u32;
            let six = six.iter().sum::<u8>() as u32;

            Pattern { three, four, six, nine }
        })
        .collect();

    let mut current = vec![0; patterns.len()];
    let mut result = Vec::new();

    // Begin with single starting pattern
    current[0] = 1;

    // Calculate generations 0 to 20 inclusive.
    for _ in 0..7 {
        let mut three = 0;
        let mut four = 0;
        let mut six = 0;
        let mut next = vec![0; patterns.len()];

        for (count, pattern) in current.iter().zip(patterns.iter()) {
            three += count * pattern.three;
            four += count * pattern.four;
            six += count * pattern.six;
            // Each 6x6 grid splits into nine 3x3 grids.
            pattern.nine.iter().for_each(|&i| next[i] += count);
        }

        result.push(three);
        result.push(four);
        result.push(six);
        current = next;
    }

    result
}

pub fn part1(input: &[u32]) -> u32 {
    input[5]
}

pub fn part2(input: &[u32]) -> u32 {
    input[18]
}

/// Generate an array of the 8 possible transformations possible from rotating and flipping
/// the 2x2 input.
fn two_by_two_permutations(mut a: [u8; 4]) -> [usize; 8] {
    let mut indices = [0; 8];

    for (i, index) in indices.iter_mut().enumerate() {
        // Convert pattern to binary to use as lookup index.
        *index = to_index(&a);
        // Rotate clockwise
        // 0 1 => 2 0
        // 2 3    3 1
        a = [a[2], a[0], a[3], a[1]];
        // Flip vertical
        // 0 1 => 2 3
        // 2 3    0 1
        if i == 3 {
            a = [a[2], a[3], a[0], a[1]];
        }
    }

    indices
}

/// Generate an array of the 8 possible transformations possible from rotating and flipping
/// the 3x3 input.
fn three_by_three_permutations(mut a: [u8; 9]) -> [usize; 8] {
    let mut indices = [0; 8];

    for (i, index) in indices.iter_mut().enumerate() {
        // Convert pattern to binary to use as lookup index.
        *index = to_index(&a);
        // Rotate clockwise
        // 0 1 2 => 6 3 0
        // 3 4 5    7 4 1
        // 6 7 8    8 5 2
        a = [a[6], a[3], a[0], a[7], a[4], a[1], a[8], a[5], a[2]];
        // Flip vertical
        // 0 1 2 => 6 7 8
        // 3 4 5    3 4 5
        // 6 7 8    0 1 2
        if i == 3 {
            a = [a[6], a[7], a[8], a[3], a[4], a[5], a[0], a[1], a[2]];
        }
    }

    indices
}

/// Convert a pattern slice of ones and zeroes to a binary number.
fn to_index(a: &[u8]) -> usize {
    a.iter().fold(0, |acc, &n| (acc << 1) | n as usize)
}
