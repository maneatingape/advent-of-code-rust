use aoc::year2024::day11::*;

const EXAMPLE: &str = "125 17";

#[test]
fn part1_test() {
    assert_eq!(part1(&parse(EXAMPLE)), 55312);
}

#[test]
fn part2_test() {
    assert_eq!(part2(&parse(EXAMPLE)), 65601038650482);
}
