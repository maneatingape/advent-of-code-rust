use aoc::year2017::day02::*;

const FIRST_EXAMPLE: &str = "\
5 1 9 5
7 5 3
2 4 6 8";

const SECOND_EXAMPLE: &str = "\
5 9 2 8
9 4 7 3
3 8 6 5";

#[test]
fn part1_test() {
    let input = parse(FIRST_EXAMPLE);
    assert_eq!(part1(&input), 18);
}

#[test]
fn part2_test() {
    let input = parse(SECOND_EXAMPLE);
    assert_eq!(part2(&input), 9);
}
