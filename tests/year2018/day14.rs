use aoc::year2018::day14::*;

const EXAMPLE: &str = "594142";

#[test]
fn part1_test() {
    assert_eq!(part1(&parse(EXAMPLE)), "1291321443");
}

#[test]
fn part2_test() {
    assert_eq!(part2(&parse(EXAMPLE)), 2018);
}
