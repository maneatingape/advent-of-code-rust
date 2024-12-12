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
    let input = parse(EXAMPLE);
    assert_eq!(part1(&input), 27730);
}

#[test]
fn part2_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part2(&input), 4988);
}
