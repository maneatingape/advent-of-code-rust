use aoc::year2025::day01::*;

const EXAMPLE: &str = "\
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

#[test]
fn part1_test() {
    assert_eq!(part1(&parse(EXAMPLE)), 3);
}

#[test]
fn part2_test() {
    assert_eq!(part2(&parse(EXAMPLE)), 6);
}
