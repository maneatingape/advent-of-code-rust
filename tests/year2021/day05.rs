use aoc::year2021::day05::*;

const EXAMPLE: &str = "\
0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

#[test]
fn part1_test() {
    assert_eq!(part1(&parse(EXAMPLE)), 5);
}

#[test]
fn part2_test() {
    assert_eq!(part2(&parse(EXAMPLE)), 12);
}
