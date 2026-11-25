use aoc::year2020::day25::*;

const EXAMPLE: &str = "\
5764801
17807724";

#[test]
fn part1_test() {
    assert_eq!(part1(&parse(EXAMPLE)), 14897079);
}

#[test]
fn part2_test() {
    assert_eq!(part2(&parse(EXAMPLE)), "n/a");
}
