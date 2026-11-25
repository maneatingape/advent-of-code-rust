use aoc::year2019::day04::*;

const EXAMPLE: &str = "100000-200000";

#[test]
fn part1_test() {
    assert_eq!(part1(&parse(EXAMPLE)), 1231);
}

#[test]
fn part2_test() {
    assert_eq!(part2(&parse(EXAMPLE)), 898);
}
