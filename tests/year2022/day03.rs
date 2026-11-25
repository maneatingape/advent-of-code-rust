use aoc::year2022::day03::*;

const EXAMPLE: &str = "\
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

#[test]
fn part1_test() {
    assert_eq!(part1(&parse(EXAMPLE)), 157);
}

#[test]
fn part2_test() {
    assert_eq!(part2(&parse(EXAMPLE)), 70);
}
