//! # Space Image Format
const WIDTH: usize = 25;
const HEIGHT: usize = 6;
const LAYER_SIZE: usize = WIDTH * HEIGHT;

pub fn parse(input: &str) -> &[u8] {
    input.as_bytes()
}

/// Each layer is 25 * 6 = 150 bytes and there are 100 layers total.
pub fn part1(input: &[u8]) -> u32 {
    let mut most = 0;
    let mut result = 0;

    for layer in input.chunks_exact(LAYER_SIZE) {
        let mut ones = 0;
        let mut twos = 0;

        for &b in layer {
            ones += u32::from(b & 1);
            twos += u32::from((b >> 1) & 1);
        }

        if ones + twos > most {
            most = ones + twos;
            result = ones * twos;
        }
    }

    result
}

/// Since a black or white pixel covers those in lower layers, it's faster to check each pixel
/// stopping as soon as we hit a non-transparent value.
pub fn part2(input: &[u8]) -> String {
    // Ensure enough capacity including newlines.
    let mut result = String::with_capacity((WIDTH + 1) * HEIGHT);

    for y in 0..HEIGHT {
        result.push('\n');

        for x in 0..WIDTH {
            let mut i = WIDTH * y + x;

            while input[i] == b'2' {
                i += LAYER_SIZE;
            }

            result.push(if input[i] == b'1' { '#' } else { '.' });
        }
    }

    result
}
