use aoc::year2025::day03::*;

const EXAMPLE: &str = "\
987654321111111
811111111111119
234234234234278
818181911112111";

#[test]
fn part1_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part1(&input), 357);
}

#[test]
fn part2_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part2(&input), 3121910778619);
}
