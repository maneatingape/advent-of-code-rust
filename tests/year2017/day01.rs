use aoc::year2017::day01::*;

const FIRST_EXAMPLE: &str = "1122";

const SECOND_EXAMPLE: &str = "1212";

#[test]
fn part1_test() {
    let input = parse(FIRST_EXAMPLE);
    assert_eq!(part1(input), 3);
}

#[test]
fn part2_test() {
    let input = parse(SECOND_EXAMPLE);
    assert_eq!(part2(input), 6);
}
