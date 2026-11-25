use aoc::year2021::day12::*;

const EXAMPLE: &str = "\
start-A
start-b
A-c
A-b
b-d
A-end
b-end";

#[test]
fn part1_test() {
    assert_eq!(part1(&parse(EXAMPLE)), 10);
}

#[test]
fn part2_test() {
    assert_eq!(part2(&parse(EXAMPLE)), 36);
}
