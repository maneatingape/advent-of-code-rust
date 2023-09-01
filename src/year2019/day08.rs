//! # Space Image Format

pub fn parse(input: &str) -> &str {
    input
}

/// Each layer is 25 * 6 = 150 bytes and there are 100 layers total.
/// It's faster to count pixels 8 at a time by parsing the bytes as `u64` then using bitwise logic
/// and the [`count_ones`] intrinsic. The only minor wrinkle is that 8 does not divide 150 evenly
/// so we must handle the last 6 bytes specially.
pub fn part1(input: &str) -> u32 {
    let bytes = input.as_bytes();
    let mut index = 0;
    let mut ones = 0;
    let mut twos = 0;
    let mut most = 0;
    let mut result = 0;

    for _ in 0..100 {
        // First 144 of 150 bytes.
        for _ in 0..18 {
            let slice = &bytes[index..(index + 8)];
            let n = u64::from_be_bytes(slice.try_into().unwrap());
            ones += (n & 0x0101010101010101).count_ones();
            twos += (n & 0x0202020202020202).count_ones();
            index += 8;
        }

        // Handle remaining 6 bytes.
        // The masks exclude the most significant 2 bytes to prevent double counting.
        let slice = &bytes[(index - 2)..(index + 6)];
        let n = u64::from_be_bytes(slice.try_into().unwrap());
        ones += (n & 0x0000010101010101).count_ones();
        twos += (n & 0x0000020202020202).count_ones();
        index += 6;

        if ones + twos > most {
            most = ones + twos;
            result = ones * twos;
        }

        ones = 0;
        twos = 0;
    }

    result
}

/// Since a black or white pixel covers those in lower layers, it's faster to check each pixel
/// stopping as soon as we hit a non-transparent value.
pub fn part2(input: &str) -> String {
    let bytes = input.as_bytes();
    let mut image = ['.'; 150];

    for (i, pixel) in image.iter_mut().enumerate() {
        let mut j = i;

        while bytes[j] == b'2' {
            j += 150;
        }

        if bytes[j] == b'1' {
            *pixel = '#';
        }
    }

    let mut result =
        image.chunks_exact(25).map(|row| row.iter().collect()).collect::<Vec<String>>().join("\n");
    result.insert(0, '\n');
    result
}
