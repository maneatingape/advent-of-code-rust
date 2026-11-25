use aoc::year2024::day19::*;

const EXAMPLE: &str = "\
r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";

#[test]
fn part1_test() {
    assert_eq!(part1(&parse(EXAMPLE)), 6);
}

#[test]
fn part2_test() {
    assert_eq!(part2(&parse(EXAMPLE)), 16);
}
