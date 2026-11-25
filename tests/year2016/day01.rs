use aoc::year2016::day01::*;

const EXAMPLE1: &str = "R5, L5, R5, R3";
const EXAMPLE2: &str = "R8, R4, R4, R8";

#[test]
fn part1_test() {
    assert_eq!(part1(&parse(EXAMPLE1)), 12);
}

#[test]
fn part2_test() {
    assert_eq!(part2(&parse(EXAMPLE2)), 4);
}
