use aoc::year2023::day01::*;

const FIRST_EXAMPLE: &str = "\
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

const SECOND_EXAMPLE: &str = "\
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

#[test]
fn part1_test() {
    let input = parse(FIRST_EXAMPLE);
    assert_eq!(part1(&input), 142);
}

#[test]
fn part2_test() {
    let input = parse(SECOND_EXAMPLE);
    assert_eq!(part2(&input), 281);
}
