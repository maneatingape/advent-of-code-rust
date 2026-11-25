use aoc::year2017::day23::*;

const EXAMPLE: &str = "123";

#[test]
fn part1_test() {
    assert_eq!(part1(&parse(EXAMPLE)), 14641);
}

#[test]
fn part2_test() {
    assert_eq!(part2(&parse(EXAMPLE)), 913);
}
