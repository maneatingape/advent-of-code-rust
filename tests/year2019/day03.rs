use aoc::year2019::day03::*;

const EXAMPLE: &str = "\
R8,U5,L5,D3
U7,R6,D4,L4";

#[test]
fn part1_test() {
    assert_eq!(part1(&parse(EXAMPLE)), 6);
}

#[test]
fn part2_test() {
    assert_eq!(part2(&parse(EXAMPLE)), 30);
}
