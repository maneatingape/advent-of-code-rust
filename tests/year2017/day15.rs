use aoc::year2017::day15::*;

const EXAMPLE: &str = "\
Generator A starts with 65
Generator B starts with 8921";

#[test]
fn part1_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part1(&input), 588);
}

#[test]
fn part2_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part2(&input), 309);
}
