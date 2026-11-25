use aoc::year2020::day01::*;

const EXAMPLE: &str = "\
1721
979
366
299
675
1456";

#[test]
fn part1_test() {
    assert_eq!(part1(&parse(EXAMPLE)), 514579);
}

#[test]
fn part2_test() {
    assert_eq!(part2(&parse(EXAMPLE)), 241861950);
}
