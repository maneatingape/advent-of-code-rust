use aoc::year2021::day12::*;

const EXAMPLE: &str = "\
start-A
start-b
A-c
A-b
b-d
A-end
b-end";

#[test]
fn part1_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part1(&input), 10);
}

#[test]
fn part2_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part2(&input), 36);
}
