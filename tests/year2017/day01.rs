use aoc::year2017::day01::*;

const EXAMPLE1: &str = "1122";

const EXAMPLE2: &str = "1212";

#[test]
fn part1_test() {
    assert_eq!(part1(parse(EXAMPLE1)), 3);
}

#[test]
fn part2_test() {
    assert_eq!(part2(parse(EXAMPLE2)), 6);
}
