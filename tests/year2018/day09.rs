use aoc::year2018::day09::*;

const EXAMPLE: &str = "10 players; last marble is worth 1618 points";

#[test]
fn part1_test() {
    assert_eq!(part1(&parse(EXAMPLE)), 8317);
}

#[test]
fn part2_test() {
    assert_eq!(part2(&parse(EXAMPLE)), 74765078);
}
