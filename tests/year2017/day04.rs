use aoc::year2017::day04::*;

const EXAMPLE1: &str = "\
aa bb cc dd ee
aa bb cc dd aa
aa bb cc dd aaa";

const EXAMPLE2: &str = "\
abcde fghij
abcde xyz ecdab
a ab abc abd abf abj
iiii oiii ooii oooi oooo
oiii ioii iioi iiio";

#[test]
fn part1_test() {
    assert_eq!(part1(&parse(EXAMPLE1)), 2);
}

#[test]
fn part2_test() {
    assert_eq!(part2(&parse(EXAMPLE2)), 3);
}
