use aoc::year2020::day12::*;

const EXAMPLE: &str = "\
F10
N3
F7
R90
F11";

#[test]
fn part1_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part1(&input), 25);
}

#[test]
fn part2_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part2(&input), 286);
}
