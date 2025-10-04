//! # Treetop Tree House
//!
//! Part 1 is solved with an efficient `O(n)` algorithm. Part 2 is also solved with an efficient `O(n)`
//! algorithm, using a bit manipulation trick to make the complexity independent of the number of digits.

const ONES: u64 = 0x0041041041041041;
const MASK: u64 = 0x0fffffffffffffc0;

type Input = (usize, Vec<i8>);

/// Convert a 2D grid of ASCII digits into a 1D `vec` of heights.
///
/// Each height is multipled by 6. For part 1 this makes no difference, but for part 2 this helps
/// with the bit manipulation.
///
/// To convert from 2D co-ordinates to an index, the formula is `y * width + x`. For the sample grid
/// of width 5, the top middle point at `(2, 0)` is at index `0 * 5 + 2 = 2` and the point directly
/// below `(2, 1)` is at index `1 * 5 + 2 = 7`.
///
/// Using a 1D `vec` instead of a `vec` of `vec`s is faster for 2 reasons:
/// * Avoids an intermediate pointer lookup for each access.
/// * Better cache locality as the memory locations are adjacent and not potentially
///   scattered all over the heap.
pub fn parse(input: &str) -> Input {
    let raw: Vec<_> = input.lines().collect();
    let width = raw[0].len();
    let mut digits = Vec::new();

    for line in &raw {
        let iter = line.bytes().map(|b| 6 * (b - b'0') as i8);
        digits.extend(iter);
    }

    (width, digits)
}

/// Calculate visible trees using a rolling maximum for each row and column in left, right, up
/// and down directions.
///
/// Using the top row of the sample and going left to right:
///
/// | Tree | Max | Visible |
/// |---|---|---|
/// | 3 | -1 | true |
/// | 0 | 3 | false |
/// | 3 | 3 | false
/// | 7 | 3 | true |
///
/// The last tree in each row and column doesn't need to be checked since it's covered
/// by the loop in the opposite direction.
///
/// A tree is visible if it can be seen from any direction. As a minor optimization, rather
/// than have 4 separate loops pairs, the left, right, up and down loops are all rolled into
/// one pair, to amortise the cost of loop logic.
///
/// The 4 corners trees don't need to be checked since they're always visible
/// so they're added directly to the total.
pub fn part1(input: &Input) -> usize {
    let (width, digits) = input;
    let width = *width;
    let mut visible = vec![false; digits.len()];

    for i in 1..(width - 1) {
        let mut left_max = -1;
        let mut right_max = -1;
        let mut top_max = -1;
        let mut bottom_max = -1;

        for j in 0..(width - 1) {
            let left = (i * width) + j;
            if digits[left] > left_max {
                visible[left] = true;
                left_max = digits[left];
            }

            let right = (i * width) + (width - j - 1);
            if digits[right] > right_max {
                visible[right] = true;
                right_max = digits[right];
            }

            let top = (j * width) + i;
            if digits[top] > top_max {
                visible[top] = true;
                top_max = digits[top];
            }

            let bottom = (width - j - 1) * width + i;
            if digits[bottom] > bottom_max {
                visible[bottom] = true;
                bottom_max = digits[bottom];
            }
        }
    }

    4 + visible.iter().filter(|&&b| b).count()
}

/// Calculate the distance visible in each direction by using 10 rolling maximum values for each
/// height packed into a single `u64`.
///
/// Part 2 is similar to part 1, but instead of keeping a single maximum for each direction, we
/// need to keep an *array* of 10 values, one for each possible height.
///
/// For each tree its score is the current value at the same index as its height. Then we increment
/// the value of previously seen trees greater than the current height by one
/// and reset the values of trees less than or equal than the current height to one.
///
/// We skip processing the edge and corner trees. Strictly speaking their score should be zero, but
/// as the maximum value will always be greater than or equal to one, it's fine to leave them
/// bulk initialized to one.
///
/// Using the fourth row of the sample and going left to right:
///
/// | Tree | Scenic Array | Score |
/// |---|---|---|
/// | 3 | [1, 1, 1, (1), 1, 1, 1, 1, 1, 1] | 1
/// | 5 | [1, 1, 1, 1, 2, (2), 2, 2, 2, 2] | 2
/// | 4 | [1, 1, 1, 1, (1), 1, 3, 3, 3, 3] | 1
///
/// Instead of using an array and iterating over it to update the values, we can achieve the same
/// result much faster by packing the ten values into a single `u64` in blocks of 6 bits. 6 bits
/// gives a maximum value of `2^6 = 64` that's a bit of a gamble. It's less than the maximum
/// possible value of 98 that could be theoretically encountered but should work for most inputs.
///
/// To obtain the current value we right shift by the current height times 6 (this is why we
/// multiplied by 6 in the [`parse`] function) and mask only the least significant 6 bits.
///
/// To update the next value, we first use [`MASK`] left shifted to zero out all bits less than
/// or equal to the current height then add 1 to all values in parallel using the [`ONES`] pattern.
///
/// For example going from 3 to 5 in the sample above:
/// ```none
///   scenic:        000001_000001_000001_000001_000001_000001_000001_000001_000001_000001
///   mask:          111111_111111_111111_111111_111111_111111_000000_000000_000000_000000
///   scenic & mask: 000001_000001_000001_000001_000001_000001_000000_000000_000000_000000
///   scenic + ones: 000010_000010_000010_000010_000010_000010_000001_000001_000001_000001
/// ```
pub fn part2(input: &Input) -> u64 {
    let (width, digits) = input;
    let width = *width;
    let mut scenic = vec![1; digits.len()];

    for i in 1..(width - 1) {
        let mut left_max = ONES;
        let mut right_max = ONES;
        let mut top_max = ONES;
        let mut bottom_max = ONES;

        for j in 1..(width - 1) {
            let left = (i * width) + j;
            scenic[left] *= (left_max >> digits[left]) & 0x3f;
            left_max = (left_max & (MASK << digits[left])) + ONES;

            let right = (i * width) + (width - j - 1);
            scenic[right] *= (right_max >> digits[right]) & 0x3f;
            right_max = (right_max & (MASK << digits[right])) + ONES;

            let top = (j * width) + i;
            scenic[top] *= (top_max >> digits[top]) & 0x3f;
            top_max = (top_max & (MASK << digits[top])) + ONES;

            let bottom = (width - j - 1) * width + i;
            scenic[bottom] *= (bottom_max >> digits[bottom]) & 0x3f;
            bottom_max = (bottom_max & (MASK << digits[bottom])) + ONES;
        }
    }

    *scenic.iter().max().unwrap()
}
