use aoc::year2015::day11::*;

const EXAMPLE: &str = "ghijklmn";

#[test]
fn part1_test() {
    assert_eq!(part1(&parse(EXAMPLE)), "ghjaabcc");
}

#[test]
fn part2_test() {
    assert_eq!(part2(&parse(EXAMPLE)), "ghjbbcdd");
}
