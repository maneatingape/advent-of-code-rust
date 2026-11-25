use aoc::year2017::day03::*;

const EXAMPLE1: &str = "1024";

const EXAMPLE2: &str = "805";

#[test]
fn part1_test() {
    assert_eq!(part1(&parse(EXAMPLE1)), 31);
}

#[test]
fn part2_test() {
    assert_eq!(part2(&parse(EXAMPLE2)), 806);
}
