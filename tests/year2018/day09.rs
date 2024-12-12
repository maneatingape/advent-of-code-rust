use aoc::year2018::day09::*;

const EXAMPLE: &str = "10 players; last marble is worth 1618 points";

#[test]
fn part1_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part1(&input), 8317);
}

#[test]
fn part2_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part2(&input), 74765078);
}
