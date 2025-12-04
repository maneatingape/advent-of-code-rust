use aoc::year2025::day05::*;

const EXAMPLE: &str = "\
3-5
10-14
16-20
12-18

1
5
8
11
17
32";

#[test]
fn part1_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part1(&input), 3);
}

#[test]
fn part2_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part2(&input), 14);
}
