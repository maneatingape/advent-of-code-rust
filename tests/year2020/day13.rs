use aoc::year2020::day13::*;

const EXAMPLE: &str = "\
939
7,13,x,x,59,x,31,19";

#[test]
fn part1_test() {
    assert_eq!(part1(&parse(EXAMPLE)), 295);
}

#[test]
fn part2_test() {
    assert_eq!(part2(&parse(EXAMPLE)), 1068781);
}
