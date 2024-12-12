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
    let input = parse(EXAMPLE);
    assert_eq!(part1(&input), 3);
}

#[test]
fn part2_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part2(&input), 1623178306);
}
