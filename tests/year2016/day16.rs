use aoc::year2016::day16::*;

// From the puzzle, valid for part 1.
const EXAMPLE: &str = "10000";

#[test]
fn part1_test() {
    // 20 is 5 * 2²
    let input = parse(EXAMPLE);
    assert_eq!(checksum(&input, 20), "01100");
}

#[test]
fn part2_test() {
    // No example data
}
