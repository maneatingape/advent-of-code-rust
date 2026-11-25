use aoc::year2015::day04::*;

const EXAMPLE: &str = "cxsaadws";

#[test]
fn part1_test() {
    assert_eq!(part1(&parse(EXAMPLE)), 218);
}

#[test]
fn part2_test() {
    assert_eq!(part2(&parse(EXAMPLE)), 455);
}
