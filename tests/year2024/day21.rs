use aoc::year2024::day21::*;

const EXAMPLE: &str = "\
029A
980A
179A
456A
379A";

#[test]
fn part1_test() {
    assert_eq!(part1(&parse(EXAMPLE)), 126384);
}

#[test]
fn part2_test() {
    assert_eq!(part2(&parse(EXAMPLE)), 154115708116294);
}
