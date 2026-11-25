use aoc::year2018::day15::*;

const EXAMPLE: &str = "\
#######
#.G...#
#...EG#
#.#.#G#
#..G#E#
#.....#
#######";

#[test]
fn part1_test() {
    assert_eq!(part1(&parse(EXAMPLE)), 27730);
}

#[test]
fn part2_test() {
    assert_eq!(part2(&parse(EXAMPLE)), 4988);
}
