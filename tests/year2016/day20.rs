use aoc::year2016::day20::*;

const EXAMPLE: &str = "\
5-8
0-2
4-7
10-4294967295";

#[test]
fn part1_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part1(&input), 3);
}

#[test]
fn part2_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part2(&input), 2);
}
