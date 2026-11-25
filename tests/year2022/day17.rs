use aoc::year2022::day17::*;

const EXAMPLE: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

#[test]
fn part1_test() {
    assert_eq!(part1(parse(EXAMPLE)), 3068);
}

#[test]
fn part2_test() {
    assert_eq!(part2(parse(EXAMPLE)), 1514285714288);
}
