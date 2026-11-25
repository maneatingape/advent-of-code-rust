use aoc::year2020::day10::*;

const EXAMPLE: &str = "\
28 33 18 42 31 14 46 20 48 47
24 23 49 45 19 38 39 11 1 32
25 35 8 17 7 9 4 2 34 10 3";

#[test]
fn part1_test() {
    assert_eq!(part1(&parse(EXAMPLE)), 220);
}

#[test]
fn part2_test() {
    assert_eq!(part2(&parse(EXAMPLE)), 19208);
}
