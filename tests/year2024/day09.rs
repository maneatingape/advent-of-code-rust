use aoc::year2024::day09::*;

const EXAMPLE: &str = "2333133121414131402";

#[test]
fn part1_test() {
    assert_eq!(part1(&parse(EXAMPLE)), 1928);
}

#[test]
fn part2_test() {
    assert_eq!(part2(&parse(EXAMPLE)), 2858);
}
