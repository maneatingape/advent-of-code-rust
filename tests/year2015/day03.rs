use aoc::year2015::day03::*;

const EXAMPLE: &str = "^v^v^v^v^v";

#[test]
fn part1_test() {
    assert_eq!(part1(&parse(EXAMPLE)), 2);
}

#[test]
fn part2_test() {
    assert_eq!(part2(&parse(EXAMPLE)), 11);
}
