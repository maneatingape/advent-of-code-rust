use aoc::year2020::day06::*;

const EXAMPLE: &str = "\
abc

a
b
c

ab
ac

a
a
a
a

b
";

#[test]
fn part1_test() {
    assert_eq!(part1(&parse(EXAMPLE)), 11);
}

#[test]
fn part2_test() {
    assert_eq!(part2(&parse(EXAMPLE)), 6);
}
