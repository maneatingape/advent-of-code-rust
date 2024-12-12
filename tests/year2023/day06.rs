use aoc::year2023::day06::*;

const EXAMPLE: &str = "\
Time:      7  15   30
Distance:  9  40  200";

#[test]
fn part1_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part1(&input), 288);
}

#[test]
fn part2_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part2(&input), 71503);
}
