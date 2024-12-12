use aoc::year2018::day11::*;

const EXAMPLE: &str = "18";

#[test]
fn part1_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part1(&input), "33,45");
}

#[test]
fn part2_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part2(&input), "90,269,16");
}
