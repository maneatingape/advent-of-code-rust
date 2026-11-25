use aoc::year2019::day22::*;

const EXAMPLE: &str = "\
deal into new stack
cut -2
deal with increment 7
cut 8
cut -4
deal with increment 7
cut 3
deal with increment 9
deal with increment 3
cut -1";

#[test]
fn part1_test() {
    assert_eq!(part1(parse(EXAMPLE)), 1219);
}

#[test]
fn part2_test() {
    assert_eq!(part2(parse(EXAMPLE)), 117607927195067);
}
