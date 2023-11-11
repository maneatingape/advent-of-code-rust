use aoc::year2017::day03::*;

const FIRST_EXAMPLE: &str = "1024";

const SECOND_EXAMPLE: &str = "805";

#[test]
fn part1_test() {
    let input = parse(FIRST_EXAMPLE);
    assert_eq!(part1(&input), 31);
}

#[test]
fn part2_test() {
    let input = parse(SECOND_EXAMPLE);
    assert_eq!(part2(&input), 806);
}
