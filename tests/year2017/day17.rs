use aoc::year2017::day17::*;

const EXAMPLE: &str = "3";

#[test]
fn part1_test() {
    assert_eq!(part1(&parse(EXAMPLE)), 638);
}

#[test]
fn part2_test() {
    assert_eq!(part2(&parse(EXAMPLE)), 1222153);
}
