use aoc::year2017::day13::*;

const EXAMPLE: &str = "\
0: 3
1: 2
4: 4
6: 4";

#[test]
fn part1_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part1(&input), 24);
}

#[test]
fn part2_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part2(&input), 10);
}
