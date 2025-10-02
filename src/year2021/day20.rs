//! # Trench Map
//!
//! This is a cellular automata problem, similar to Conway's Game of Life, except that the rules
//! are encoded in the enhancement algorithm string, instead of being statically specified. Each
//! round the initial square area of cells expands by at most one in each direction, so we can store
//! the cell in a fixed size array with enough space on either side to expand into.
//!
//! The interesting nuance is handling the edge cells when all 9 cells are empty (index 0) or all
//! 9 cell are active (index 511). The sample data encodes a blank cell in both scenarios.
//! My input encoded an active cell for index 0 and a blank cell for index 511, meaning that each
//! turn the edge cells toggle from set to unset.
//!
//! The algorithm keeps track of the bounds of the expanding square and supplies a `default` value,
//! that in the example case is always zero, but in the real data toggles between zero and one.
pub struct Input {
    size: usize,
    algorithm: [u8; 512],
    pixels: [u8; 40_000],
}

pub fn parse(input: &str) -> Input {
    // `#` is odd and `.` is even so we can convert to one or zero by bitwise AND with 1.
    let bits: Vec<Vec<_>> =
        input.lines().map(|line| line.bytes().map(|b| b & 1).collect()).collect();
    let size = bits.len() - 2;
    let algorithm = bits[0][..512].try_into().unwrap();

    // Offset the initial square by 50 cells in both dimensions.
    // The square expands by at most one in each step so this is enough room to stay within bounds.
    let mut pixels = [0; 40_000];
    for (i, row) in bits[2..].iter().enumerate() {
        let start = (i + 50) * 200 + 50;
        let end = start + size;
        pixels[start..end].copy_from_slice(&row[..size]);
    }

    Input { size, algorithm, pixels }
}

pub fn part1(input: &Input) -> usize {
    enhance(input, 2)
}

pub fn part2(input: &Input) -> usize {
    enhance(input, 50)
}

fn enhance(input: &Input, steps: usize) -> usize {
    let algorithm = input.algorithm;
    let mut pixels = input.pixels;
    let mut next = [0; 40_000];

    let mut start = 50;
    let mut end = 50 + input.size as i32;
    let mut default = 0;

    for _ in 0..steps {
        for y in (start - 1)..(end + 1) {
            // If the pixel is within current bounds then return it, or else use the `default`
            // edge value specified by the enhancement algorithm.
            let helper = |sx, sy, shift| {
                let result = if sx < end && sy >= start && sy < end {
                    pixels[(sy * 200 + sx) as usize] as usize
                } else {
                    default as usize
                };
                result << shift
            };

            // If the edge pixels are 1 then the initial edge will look like
            // [##a]
            // [##b]
            // [##c]
            // or 11a11b11c when encoded as an index.
            let mut index = if default == 1 { 0b11011011 } else { 0b00000000 };

            for x in (start - 1)..(end + 1) {
                // Keeps a sliding window of the index, updated as we evaluate the row from
                // left to right. Shift the index left by one each turn, updating the values from
                // the three new rightmost pixels entering the window.
                index = ((index << 1) & 0b110110110)
                    + helper(x + 1, y - 1, 6)
                    + helper(x + 1, y, 3)
                    + helper(x + 1, y + 1, 0);

                next[(y * 200 + x) as usize] = algorithm[index];
            }
        }

        // Boundaries expand by one each turn
        pixels = next;
        start -= 1;
        end += 1;

        // Calculate the next value for edge pixels beyond the boundary.
        if default == 0 {
            default = algorithm[0];
        } else {
            default = algorithm[511];
        }
    }

    pixels.iter().filter(|&&p| p == 1).count()
}
