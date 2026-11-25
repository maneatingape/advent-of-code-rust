use aoc::year2017::day05::*;

const EXAMPLE: &str = "\
0
3
0
1
-3";

#[test]
fn part1_test() {
    assert_eq!(part1(&parse(EXAMPLE)), 5);
}

#[test]
fn part2_test() {
    assert_eq!(part2(&parse(EXAMPLE)), 10);
}
