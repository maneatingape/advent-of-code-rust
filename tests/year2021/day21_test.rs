use aoc::year2021::day21::*;

const EXAMPLE: &str = "\
Player 1 starting position: 4
Player 2 starting position: 8";

#[test]
fn part1_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part1(&input), 739785);
}

#[test]
fn part2_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part2(&input), 444356092776315);
}
