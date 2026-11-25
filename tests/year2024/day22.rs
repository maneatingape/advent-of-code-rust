use aoc::year2024::day22::*;

const EXAMPLE1: &str = "\
1
10
100
2024";

const EXAMPLE2: &str = "\
1
2
3
2024";

#[test]
fn part1_test() {
    assert_eq!(part1(&parse(EXAMPLE1)), 37327623);
}

#[test]
fn part2_test() {
    assert_eq!(part2(&parse(EXAMPLE2)), 23);
}
