use aoc::year2016::day09::*;

const EXAMPLE: &str = "X(8x2)(3x3)ABCY";

#[test]
fn part1_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part1(input), 18);
}

#[test]
fn part2_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part2(input), 20);
}
