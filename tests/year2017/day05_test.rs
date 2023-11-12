use aoc::year2017::day05::*;

const EXAMPLE: &str = "\
0
3
0
1
-3";

#[test]
fn part1_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part1(&input), 5);
}

#[test]
fn part2_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part2(&input), 10);
}
