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
    let input = parse(EXAMPLE);
    assert_eq!(part1(&input), 17);
}

#[test]
fn part2_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part2_testable(&input, 32), 16);
}
