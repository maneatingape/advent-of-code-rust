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
    let input = parse(EXAMPLE);
    assert_eq!(part1(&input), "ABCDEF");
}

#[test]
fn part2_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part2(&input), 38);
}
