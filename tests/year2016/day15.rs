use aoc::year2016::day15::*;

const EXAMPLE: &str = "\
Disc #1 has 5 positions; at time=0, it is at position 4.
Disc #2 has 2 positions; at time=0, it is at position 1.";

#[test]
fn part1_test() {
    assert_eq!(part1(&parse(EXAMPLE)), 5);
}

#[test]
fn part2_test() {
    assert_eq!(part2(&parse(EXAMPLE)), 85);
}
