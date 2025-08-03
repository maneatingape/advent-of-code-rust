use aoc::year2015::day08::*;

const EXAMPLE: &str = r#"""
"abc"
"aaa\"aaa"
"\x27"
"#;

#[test]
fn part1_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part1(input), 12);
}

#[test]
fn part2_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part2(input), 19);
}
