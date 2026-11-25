use aoc::year2015::day24::*;

const EXAMPLE: &str = "1 2 3 4 5 7 8 9 10 11";

#[test]
fn part1_test() {
    assert_eq!(part1(&parse(EXAMPLE)), 99);
}

#[test]
fn part2_test() {
    assert_eq!(part2(&parse(EXAMPLE)), 44);
}
