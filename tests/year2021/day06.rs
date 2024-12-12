use aoc::year2021::day06::*;

const EXAMPLE: &str = "3,4,3,1,2";

#[test]
fn part1_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part1(&input), 5934);
}

#[test]
fn part2_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part2(&input), 26984457539);
}
