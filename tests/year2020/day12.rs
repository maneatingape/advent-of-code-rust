use aoc::year2020::day12::*;

const EXAMPLE: &str = "\
F10
N3
F7
R90
F11";

#[test]
fn part1_test() {
    assert_eq!(part1(&parse(EXAMPLE)), 25);
}

#[test]
fn part2_test() {
    assert_eq!(part2(&parse(EXAMPLE)), 286);
}
