use aoc::year2017::day04::*;

const FIRST_EXAMPLE: &str = "\
aa bb cc dd ee
aa bb cc dd aa
aa bb cc dd aaa";

const SECOND_EXAMPLE: &str = "\
abcde fghij
abcde xyz ecdab
a ab abc abd abf abj
iiii oiii ooii oooi oooo
oiii ioii iioi iiio";

#[test]
fn part1_test() {
    let input = parse(FIRST_EXAMPLE);
    assert_eq!(part1(&input), 2);
}

#[test]
fn part2_test() {
    let input = parse(SECOND_EXAMPLE);
    assert_eq!(part2(&input), 3);
}
