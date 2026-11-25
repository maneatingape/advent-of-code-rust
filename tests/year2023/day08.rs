use aoc::year2023::day08::*;

const EXAMPLE1: &str = "\
LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

const EXAMPLE2: &str = "\
LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

#[test]
fn part1_test() {
    assert_eq!(part1(&parse(EXAMPLE1)), 6);
}

#[test]
fn part2_test() {
    assert_eq!(part2(&parse(EXAMPLE2)), 6);
}
