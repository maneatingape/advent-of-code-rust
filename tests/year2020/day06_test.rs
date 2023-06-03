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

b";

#[test]
fn part1_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part1(&input), 11);
}

#[test]
fn part2_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part2(&input), 6);
}
