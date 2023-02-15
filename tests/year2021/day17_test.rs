use aoc::year2021::day17::*;

const EXAMPLE: &str = "target area: x=20..30, y=-10..-5";

#[test]
fn part1_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part1(&input), 45);
}

#[test]
fn part2_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part2(&input), 112);
}
