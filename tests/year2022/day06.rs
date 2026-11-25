use aoc::year2022::day06::*;

const EXAMPLE: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";

#[test]
fn part1_test() {
    assert_eq!(part1(parse(EXAMPLE)), 7);
}

#[test]
fn part2_test() {
    assert_eq!(part2(parse(EXAMPLE)), 19);
}
