use aoc::year2024::day22::*;

const FIRST_EXAMPLE: &str = "\
1
10
100
2024";

const SECOND_EXAMPLE: &str = "\
1
2
3
2024";

#[test]
fn part1_test() {
    let input = parse(FIRST_EXAMPLE);
    assert_eq!(part1(&input), 37327623);
}

#[test]
fn part2_test() {
    let input = parse(SECOND_EXAMPLE);
    assert_eq!(part2(&input), 23);
}
