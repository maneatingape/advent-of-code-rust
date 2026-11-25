use aoc::year2016::day19::*;

const EXAMPLE: &str = "5";

#[test]
fn part1_test() {
    assert_eq!(part1(&parse(EXAMPLE)), 3);
}

#[test]
fn part2_test() {
    assert_eq!(part2(&parse(EXAMPLE)), 2);
}
