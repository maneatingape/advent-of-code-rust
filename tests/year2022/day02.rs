use aoc::year2022::day02::*;

const EXAMPLE: &str = "\
A Y
B X
C Z
";

#[test]
fn part1_test() {
    assert_eq!(part1(&parse(EXAMPLE)), 15);
}

#[test]
fn part2_test() {
    assert_eq!(part2(&parse(EXAMPLE)), 12);
}
