//! # Space Image Format
const WIDTH: usize = 25;
const HEIGHT: usize = 6;
const LAYER_SIZE: usize = WIDTH * HEIGHT;

pub fn parse(input: &str) -> &[u8] {
    input.as_bytes()
}

/// Each layer is 25 * 6 = 150 bytes and there are 100 layers total.
/// It's faster to count pixels 8 at a time by parsing the bytes as `u64` then using bitwise logic
/// and the [`count_ones`] intrinsic. The only minor wrinkle is that 8 does not divide 150 evenly
/// so we must handle the last 6 bytes specially.
///
/// [`count_ones`]: u64::count_ones
pub fn part1(input: &[u8]) -> u32 {
    let mut most = 0;
    let mut result = 0;

    for layer in input.chunks_exact(LAYER_SIZE) {
        let mut ones = 0;
        let mut twos = 0;

        // First 144 of 150 bytes.
        for slice in layer.chunks_exact(8) {
            let n = u64::from_be_bytes(slice.try_into().unwrap());
            ones += (n & 0x0101010101010101).count_ones();
            twos += (n & 0x0202020202020202).count_ones();
        }

        // Handle remaining 6 bytes.
        // The masks exclude the most significant 2 bytes to prevent double counting.
        let slice = &layer[142..150];
        let n = u64::from_be_bytes(slice.try_into().unwrap());
        ones += (n & 0x0000010101010101).count_ones();
        twos += (n & 0x0000020202020202).count_ones();

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
