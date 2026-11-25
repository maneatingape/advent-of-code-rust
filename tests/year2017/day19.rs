use aoc::year2017::day19::*;

const EXAMPLE: &str = "\
.     |          .
.     |  +--+    .
.     A  |  C    .
. F---|----E|--+ .
.     |  |  |  D .
.     +B-+  +--+ .
.                .";

#[test]
fn part1_test() {
    assert_eq!(part1(&parse(EXAMPLE)), "ABCDEF");
}

#[test]
fn part2_test() {
    assert_eq!(part2(&parse(EXAMPLE)), 38);
}
