use aoc::year2017::day10::*;

const EXAMPLE: &str = "1,2,3";

#[test]
fn part1_test() {
    assert_eq!(part1(parse(EXAMPLE)), 0);
}

#[test]
fn part2_test() {
    assert_eq!(part2(parse(EXAMPLE)), "3efbe78a8d82f29979031a4aa0b16a9d");
}
