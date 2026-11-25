use aoc::year2015::day17::*;

const EXAMPLE: &str = "20,15,10,5,5";

#[test]
fn part1_test() {
    assert_eq!(part1_testable(&parse(EXAMPLE), 25), 4);
}

#[test]
fn part2_test() {
    assert_eq!(part2_testable(&parse(EXAMPLE), 25), 3);
}
