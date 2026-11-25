use aoc::year2021::day21::*;

const EXAMPLE: &str = "\
Player 1 starting position: 4
Player 2 starting position: 8";

#[test]
fn part1_test() {
    assert_eq!(part1(&parse(EXAMPLE)), 739785);
}

#[test]
fn part2_test() {
    assert_eq!(part2(&parse(EXAMPLE)), 444356092776315);
}
