use aoc::year2018::day08::*;

const EXAMPLE: &str = "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2";

#[test]
fn part1_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part1(&input), 138);
}

#[test]
fn part2_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part2(&input), 66);
}
