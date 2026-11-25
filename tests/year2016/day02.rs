use aoc::year2016::day02::*;

const EXAMPLE: &str = "\
ULL
RRDDD
LURDL
UUUUD";

#[test]
fn part1_test() {
    assert_eq!(part1(&parse(EXAMPLE)), "1985");
}

#[test]
fn part2_test() {
    assert_eq!(part2(&parse(EXAMPLE)), "5DB3");
}
