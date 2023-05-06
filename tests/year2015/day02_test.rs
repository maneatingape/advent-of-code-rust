use aoc::year2015::day02::*;

const EXAMPLE: &str = "\
2x3x4
1x1x10";

#[test]
fn part1_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part1(&input), 101);
}

#[test]
fn part2_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part2(&input), 48);
}
