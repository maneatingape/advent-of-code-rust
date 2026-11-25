use aoc::year2022::day20::*;

const EXAMPLE: &str = "\
1
2
-3
3
-2
0
4";

#[test]
fn part1_test() {
    assert_eq!(part1(&parse(EXAMPLE)), 3);
}

#[test]
fn part2_test() {
    assert_eq!(part2(&parse(EXAMPLE)), 1623178306);
}
