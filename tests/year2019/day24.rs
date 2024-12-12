use aoc::year2019::day24::*;

const EXAMPLE: &str = "\
....#
#..#.
#..##
..#..
#....";

#[test]
fn part1_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part1(&input), 2129920);
}

#[test]
fn part2_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part2_testable(&input, 10), 99);
}
