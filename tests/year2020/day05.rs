use aoc::year2020::day05::*;

const EXAMPLE: &str = "\
FBFBBFFLRR
FBFBBFFRLR";

#[test]
fn part1_test() {
    assert_eq!(part1(&parse(EXAMPLE)), 357);
}

#[test]
fn part2_test() {
    assert_eq!(part2(&parse(EXAMPLE)), 356);
}
