use aoc::year2018::day11::*;

const EXAMPLE: &str = "18";

#[test]
fn part1_test() {
    assert_eq!(part1(&parse(EXAMPLE)), "33,45");
}

#[test]
fn part2_test() {
    assert_eq!(part2(&parse(EXAMPLE)), "90,269,16");
}
