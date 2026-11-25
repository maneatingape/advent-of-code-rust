use aoc::year2020::day02::*;

const EXAMPLE: &str = "\
1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc";

#[test]
fn part1_test() {
    assert_eq!(part1(&parse(EXAMPLE)), 2);
}

#[test]
fn part2_test() {
    assert_eq!(part2(&parse(EXAMPLE)), 1);
}
