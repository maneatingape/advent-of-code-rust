use aoc::year2017::day09::*;

const EXAMPLE1: &str = "{{<a!>},{<a!>},{<a!>},{<ab>}}";

const EXAMPLE2: &str = "<{o\"i!a,<{i<a>";

#[test]
fn part1_test() {
    assert_eq!(part1(&parse(EXAMPLE1)), 3);
}

#[test]
fn part2_test() {
    assert_eq!(part2(&parse(EXAMPLE2)), 10);
}
