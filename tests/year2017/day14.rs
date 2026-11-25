use aoc::year2017::day14::*;

const EXAMPLE: &str = "flqrgnkx";

#[test]
fn part1_test() {
    assert_eq!(part1(&parse(EXAMPLE)), 8108);
}

#[test]
fn part2_test() {
    assert_eq!(part2(&parse(EXAMPLE)), 1242);
}
