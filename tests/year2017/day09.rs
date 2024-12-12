use aoc::year2017::day09::*;

const FIRST_EXAMPLE: &str = "{{<a!>},{<a!>},{<a!>},{<ab>}}";

const SECOND_EXAMPLE: &str = "<{o\"i!a,<{i<a>";

#[test]
fn part1_test() {
    let input = parse(FIRST_EXAMPLE);
    assert_eq!(part1(&input), 3);
}

#[test]
fn part2_test() {
    let input = parse(SECOND_EXAMPLE);
    assert_eq!(part2(&input), 10);
}
