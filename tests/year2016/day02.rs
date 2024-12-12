use aoc::year2016::day02::*;

const EXAMPLE: &str = "\
ULL
RRDDD
LURDL
UUUUD";

#[test]
fn part1_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part1(&input), "1985");
}

#[test]
fn part2_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part2(&input), "5DB3");
}
