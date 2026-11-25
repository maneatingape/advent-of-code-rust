use aoc::year2025::day03::*;

const EXAMPLE: &str = "\
987654321111111
811111111111119
234234234234278
818181911112111";

#[test]
fn part1_test() {
    assert_eq!(part1(&parse(EXAMPLE)), 357);
}

#[test]
fn part2_test() {
    assert_eq!(part2(&parse(EXAMPLE)), 3121910778619);
}
