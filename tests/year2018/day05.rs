use aoc::year2018::day05::*;

const EXAMPLE: &str = "dabAcCaCBAcCcaDA";

#[test]
fn part1_test() {
    assert_eq!(part1(&parse(EXAMPLE)), 10);
}

#[test]
fn part2_test() {
    assert_eq!(part2(&parse(EXAMPLE)), 4);
}
