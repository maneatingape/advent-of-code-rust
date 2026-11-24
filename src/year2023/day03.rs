//! # Gear Ratios
use crate::util::grid::*;
use crate::util::parse::*;
use crate::util::point::*;

pub struct Input {
    grid: Grid<u8>,
    seen: Grid<usize>,
    parts: Vec<u32>,
}

pub fn parse(input: &str) -> Input {
    // The newline at the end of each row stops numbers wrapping onto the next row
    // and the border keeps neighbor lookups in bounds.
    let grid = Grid::parse_with_border(input);
    // In order to tell if we've already seen a number before, store its index at the location
    // of every digit, using zero to indicate no value. For example:
    // `467..114..` => `1110022200`
    let mut seen = grid.same_size_with(0);
    // Stores each unique part number. The first value is a dummy placeholder.
    let mut parts = vec![0];
    let mut number = 0;

    for (i, &b) in grid.bytes.iter().enumerate() {
        if b.is_ascii_digit() {
            // Parse contiguous groups of digits.
            seen.bytes[i] = parts.len();
            number = 10 * number + b.to_decimal::<u32>();
        } else if number > 0 {
            // If not a digit then finish the current number.
            parts.push(number);
            number = 0;
        }
    }

    Input { grid, seen, parts }
}

/// Sum all part numbers adjacent to at least one symbol.
pub fn part1(input: &Input) -> u32 {
    let Input { grid, seen, parts } = input;
    let mut parts = parts.clone();
    let mut result = 0;

    // Skip the newline border.
    for y in 1..grid.height - 1 {
        for x in 1..grid.width {
            let p = Point::new(x, y);
            let b = grid[p];

            if !b.is_ascii_digit() && b != b'.' {
                for d in DIAGONAL {
                    let index = seen[p + d];
                    if index != 0 {
                        result += parts[index];
                        // Only count each number once when it is adjacent to multiple symbols.
                        parts[index] = 0;
                    }
                }
            }
        }
    }

    result
}

/// Sum all gears adjacent to exactly two part numbers.
pub fn part2(input: &Input) -> u32 {
    let Input { grid, seen, parts } = input;
    let mut result = 0;

    for y in 1..grid.height - 1 {
        for x in 1..grid.width {
            let p = Point::new(x, y);

            if grid[p] == b'*' {
                let mut previous = 0;
                let mut distinct = 0;
                let mut subtotal = 1;

                // Rely on the left to right and top to bottom order of DIAGONAL
                // to detect distinct numbers.
                for d in DIAGONAL {
                    let index = seen[p + d];
                    if index != 0 && index != previous {
                        previous = index;
                        distinct += 1;
                        subtotal *= parts[index];
                    }
                }

                if distinct == 2 {
                    result += subtotal;
                }
            }
        }
    }

    result
}
