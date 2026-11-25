use aoc::year2019::day01::*;

const EXAMPLE: &str = "100756";

#[test]
fn part1_test() {
    assert_eq!(part1(&parse(EXAMPLE)), 33583);
}

#[test]
fn part2_test() {
    assert_eq!(part2(&parse(EXAMPLE)), 50346);
}
