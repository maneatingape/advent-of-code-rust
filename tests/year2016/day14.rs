use aoc::year2016::day14::*;

const EXAMPLE: &str = "abc";

#[test]
fn part1_test() {
    assert_eq!(part1(parse(EXAMPLE)), 22728);
}

#[test]
fn part2_test() {
    assert_eq!(part2(parse(EXAMPLE)), 22551);
}
