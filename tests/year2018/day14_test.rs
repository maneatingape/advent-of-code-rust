use aoc::year2018::day14::*;

const EXAMPLE: &str = "594142";

#[test]
fn part1_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part1(&input), "1291321443");
}

#[test]
fn part2_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part2(&input), 2018);
}
