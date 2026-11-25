use aoc::year2016::day09::*;

const EXAMPLE: &str = "X(8x2)(3x3)ABCY";

#[test]
fn part1_test() {
    assert_eq!(part1(parse(EXAMPLE)), 18);
}

#[test]
fn part2_test() {
    assert_eq!(part2(parse(EXAMPLE)), 20);
}
