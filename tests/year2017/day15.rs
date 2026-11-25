use aoc::year2017::day15::*;

const EXAMPLE: &str = "\
Generator A starts with 65
Generator B starts with 8921";

#[test]
fn part1_test() {
    assert_eq!(part1(&parse(EXAMPLE)), 588);
}

#[test]
fn part2_test() {
    assert_eq!(part2(&parse(EXAMPLE)), 309);
}
