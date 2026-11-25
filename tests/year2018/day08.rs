use aoc::year2018::day08::*;

const EXAMPLE: &str = "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2";

#[test]
fn part1_test() {
    assert_eq!(part1(&parse(EXAMPLE)), 138);
}

#[test]
fn part2_test() {
    assert_eq!(part2(&parse(EXAMPLE)), 66);
}
