use aoc::year2015::day01::*;

const EXAMPLE: &str = "()())";

#[test]
fn part1_test() {
    assert_eq!(part1(&parse(EXAMPLE)), -1);
}

#[test]
fn part2_test() {
    assert_eq!(part2(&parse(EXAMPLE)), 5);
}
