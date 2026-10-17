//! # Ceres Search
//!
//! For part one we search each vertical, horizontal and diagonal direction one at a time over the
//! whole grid, checking for both "XMAS" and "SAMX" at every position. The newline padding prevents
//! false positives at boundaries.
//!
//! For part two the difference of the ASCII values of "M" and "S" is 6. No other combination of
//! letters has this value, so if both diagonals are 6 then we have a match.
//!
//! Both parts are written without branches, so the compiler auto-vectorizes each loop.
pub fn parse(input: &str) -> &[u8] {
    input.as_bytes()
}

pub fn part1(bytes: &[u8]) -> u32 {
    let width = bytes.iter().position(|&b| b == b'\n').unwrap() + 1;

    // Offsets for horizontal, vertical and both diagonals. Sweep the whole whole grid so that each
    // loop auto-vectorizes. The newline separators prevent false positives on boundaries.
    [1, width, width - 1, width + 1]
        .iter()
        .flat_map(|&step| {
            bytes.iter().zip(&bytes[step..]).zip(&bytes[2 * step..]).zip(&bytes[3 * step..])
        })
        .map(|(((&a, &b), &c), &d)| {
            let forward = a == b'X' && b == b'M' && c == b'A' && d == b'S';
            let backward = a == b'S' && b == b'A' && c == b'M' && d == b'X';
            u32::from(forward || backward)
        })
        .sum()
}

pub fn part2(bytes: &[u8]) -> u32 {
    let width = bytes.iter().position(|&b| b == b'\n').unwrap() + 1;

    // ASCII "M" is 77 and "S" is 83 so the absolute difference is 6.
    // No other combination of letters causes this difference.
    // "MS" on both diagonals is a match.
    bytes
        .iter()
        .zip(&bytes[2..])
        .zip(&bytes[width + 1..])
        .zip(&bytes[2 * width..])
        .zip(&bytes[2 * width + 2..])
        .map(|((((&up_left, &up_right), &center), &down_left), &down_right)| {
            let first = u32::from(up_left.abs_diff(down_right) == 6);
            let second = u32::from(up_right.abs_diff(down_left) == 6);
            u32::from(center == b'A') & first & second
        })
        .sum()
}
