use aoc::year2015::day03::*;

const EXAMPLE: &str = "^v^v^v^v^v";

#[test]
fn part1_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part1(&input), 2);
}

#[test]
fn part2_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part2(&input), 11);
}
