use aoc::year2024::day09::*;

const EXAMPLE: &str = "2333133121414131402";

#[test]
fn part1_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part1(&input), 1928);
}

#[test]
fn part2_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part2(&input), 2858);
}
