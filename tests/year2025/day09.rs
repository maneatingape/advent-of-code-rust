use aoc::year2025::day09::*;

const EXAMPLE: &str = "\
7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";

#[test]
fn part1_test() {
    assert_eq!(part1(&parse(EXAMPLE)), 50);
}

#[test]
fn part2_test() {
    assert_eq!(part2(&parse(EXAMPLE)), 24);
}
