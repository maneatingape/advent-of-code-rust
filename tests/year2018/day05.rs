use aoc::year2018::day05::*;

const EXAMPLE: &str = "dabAcCaCBAcCcaDA";

#[test]
fn part1_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part1(&input), 10);
}

#[test]
fn part2_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part2(&input), 4);
}
