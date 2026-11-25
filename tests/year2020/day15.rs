use aoc::year2020::day15::*;

const EXAMPLE: &str = "0,3,6";

#[test]
fn part1_test() {
    assert_eq!(part1(&parse(EXAMPLE)), 436);
}

#[test]
fn part2_test() {
    assert_eq!(part2(&parse(EXAMPLE)), 175594);
}
