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
    let input = parse(EXAMPLE);
    assert_eq!(part1(&input), 514579);
}

#[test]
fn part2_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part2(&input), 241861950);
}
