use aoc::year2022::day02::*;

const EXAMPLE: &str = "A Y\nB X\nC Z";

#[test]
fn part1_example() {
    let input = parse(EXAMPLE);
    assert_eq!(part1(&input), 15);
}

#[test]
fn part2_example() {
    let input = parse(EXAMPLE);
    assert_eq!(part2(&input), 12);
}
