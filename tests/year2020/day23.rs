use aoc::year2020::day23::*;

const EXAMPLE: &str = "389125467";

#[test]
fn part1_test() {
    assert_eq!(part1(&parse(EXAMPLE)), 67384529);
}

#[test]
fn part2_test() {
    assert_eq!(part2(&parse(EXAMPLE)), 149245887792);
}
