use aoc::year2015::day10::*;

const EXAMPLE: &str = "12";

#[test]
fn part1_test() {
    assert_eq!(part1(&parse(EXAMPLE)), 124496);
}

#[test]
fn part2_test() {
    assert_eq!(part2(&parse(EXAMPLE)), 1766402);
}
