use aoc::year2017::day10::*;

const EXAMPLE: &str = "1,2,3";

#[test]
fn part1_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part1(input), 0);
}

#[test]
fn part2_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part2(input), "3efbe78a8d82f29979031a4aa0b16a9d");
}
