use aoc::year2015::day08::*;

const EXAMPLE: &str = r#"""
"abc"
"aaa\"aaa"
"\x27"
"#;

#[test]
fn part1_test() {
    assert_eq!(part1(parse(EXAMPLE)), 12);
}

#[test]
fn part2_test() {
    assert_eq!(part2(parse(EXAMPLE)), 19);
}
