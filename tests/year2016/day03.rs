use aoc::year2016::day03::*;

const EXAMPLE: &str = "\
101 201 301
102 102 402
203 203 303";

#[test]
fn part1_test() {
    assert_eq!(part1(&parse(EXAMPLE)), 2);
}

#[test]
fn part2_test() {
    assert_eq!(part2(&parse(EXAMPLE)), 2);
}
