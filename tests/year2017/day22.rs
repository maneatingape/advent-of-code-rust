use aoc::year2017::day22::*;

const EXAMPLE: &str = "\
..#
#..
...";

#[test]
fn part1_test() {
    assert_eq!(part1(&parse(EXAMPLE)), 5587);
}

#[test]
fn part2_test() {
    assert_eq!(part2(&parse(EXAMPLE)), 2511944);
}
