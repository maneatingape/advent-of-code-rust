use aoc::year2020::day25::*;

const EXAMPLE: &str = "\
5764801
17807724";

#[test]
fn part1_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part1(&input), 14897079);
}

#[test]
fn part2_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part2(&input), "n/a");
}
