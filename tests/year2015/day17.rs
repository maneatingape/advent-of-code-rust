use aoc::year2015::day17::*;

const EXAMPLE: &str = "20,15,10,5,5";

#[test]
fn part1_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part1_testable(&input, 25), 4);
}

#[test]
fn part2_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part2_testable(&input, 25), 3);
}
