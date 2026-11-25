use aoc::year2021::day07::*;

const EXAMPLE: &str = "16,1,2,0,4,2,7,1,2,14";

#[test]
fn part1_test() {
    assert_eq!(part1(&parse(EXAMPLE)), 37);
}

#[test]
fn part2_test() {
    assert_eq!(part2(&parse(EXAMPLE)), 168);
}
