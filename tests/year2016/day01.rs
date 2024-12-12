use aoc::year2016::day01::*;

const FIRST_EXAMPLE: &str = "R5, L5, R5, R3";
const SECOND_EXAMPLE: &str = "R8, R4, R4, R8";

#[test]
fn part1_test() {
    let input = parse(FIRST_EXAMPLE);
    assert_eq!(part1(&input), 12);
}

#[test]
fn part2_test() {
    let input = parse(SECOND_EXAMPLE);
    assert_eq!(part2(&input), 4);
}
