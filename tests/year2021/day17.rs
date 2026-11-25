use aoc::year2021::day17::*;

const EXAMPLE: &str = "target area: x=20..30, y=-10..-5";

#[test]
fn part1_test() {
    assert_eq!(part1(&parse(EXAMPLE)), 45);
}

#[test]
fn part2_test() {
    assert_eq!(part2(&parse(EXAMPLE)), 112);
}
