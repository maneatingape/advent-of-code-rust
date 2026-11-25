use aoc::year2019::day24::*;

const EXAMPLE: &str = "\
....#
#..#.
#..##
..#..
#....";

#[test]
fn part1_test() {
    assert_eq!(part1(&parse(EXAMPLE)), 2129920);
}

#[test]
fn part2_test() {
    assert_eq!(part2_testable(&parse(EXAMPLE), 10), 99);
}
