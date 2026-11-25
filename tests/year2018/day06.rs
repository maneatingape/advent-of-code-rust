use aoc::year2018::day06::*;

const EXAMPLE: &str = "\
1, 1
1, 6
8, 3
3, 4
5, 5
8, 9";

#[test]
fn part1_test() {
    assert_eq!(part1(&parse(EXAMPLE)), 17);
}

#[test]
fn part2_test() {
    assert_eq!(part2_testable(&parse(EXAMPLE), 32), 16);
}
