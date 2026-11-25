use aoc::year2021::day06::*;

const EXAMPLE: &str = "3,4,3,1,2";

#[test]
fn part1_test() {
    assert_eq!(part1(&parse(EXAMPLE)), 5934);
}

#[test]
fn part2_test() {
    assert_eq!(part2(&parse(EXAMPLE)), 26984457539);
}
