use aoc::year2015::day02::*;

const EXAMPLE: &str = "\
2x3x4
1x1x10";

#[test]
fn part1_test() {
    assert_eq!(part1(&parse(EXAMPLE)), 101);
}

#[test]
fn part2_test() {
    assert_eq!(part2(&parse(EXAMPLE)), 48);
}
