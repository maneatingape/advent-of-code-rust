use aoc::year2024::day04::*;

const EXAMPLE: &str = "\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

#[test]
fn part1_test() {
    assert_eq!(part1(parse(EXAMPLE)), 18);
}

#[test]
fn part2_test() {
    assert_eq!(part2(parse(EXAMPLE)), 9);
}
