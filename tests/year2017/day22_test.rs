use aoc::year2017::day22::*;

const EXAMPLE: &str = "\
..#
#..
...";

#[test]
fn part1_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part1(&input), 5587);
}

#[test]
fn part2_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part2(&input), 2511944);
}
