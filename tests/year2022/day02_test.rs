use aoc::year2022::day02::*;

const EXAMPLE: &str =
"A Y
B X
C Z";

#[test]
fn part1_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part1(&input), 15);
}

#[test]
fn part2_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part2(&input), 12);
}
