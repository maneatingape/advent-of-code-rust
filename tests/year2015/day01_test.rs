use aoc::year2015::day01::*;

const EXAMPLE: &str = "()())";

#[test]
fn part1_example() {
    let input = parse(EXAMPLE);
    assert_eq!(part1(&input), -1);
}

#[test]
fn part2_example() {
    let input = parse(EXAMPLE);
    assert_eq!(part2(&input), 5);
}
