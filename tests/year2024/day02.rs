use aoc::year2024::day02::*;

const EXAMPLE: &str = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

#[test]
fn part1_test() {
    assert_eq!(part1(&parse(EXAMPLE)), 2);
}

#[test]
fn part2_test() {
    assert_eq!(part2(&parse(EXAMPLE)), 4);
}
