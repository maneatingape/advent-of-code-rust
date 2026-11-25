use aoc::year2023::day13::*;

const EXAMPLE: &str = "\
#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

#[test]
fn part1_test() {
    assert_eq!(part1(&parse(EXAMPLE)), 405);
}

#[test]
fn part2_test() {
    assert_eq!(part2(&parse(EXAMPLE)), 400);
}
