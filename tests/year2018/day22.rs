use aoc::year2018::day22::*;

const EXAMPLE: &str = "\
depth: 510
target: 10,10";

#[test]
fn part1_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part1(&input), 114);
}

#[test]
fn part2_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part2(&input), 45);
}
