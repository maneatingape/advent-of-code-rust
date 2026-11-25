use aoc::year2015::day12::*;

const EXAMPLE: &str = r#"[1,{"c":"red","b":2},3]"#;

#[test]
fn part1_test() {
    assert_eq!(part1(parse(EXAMPLE)), 6);
}

#[test]
fn part2_test() {
    assert_eq!(part2(parse(EXAMPLE)), 4);
}
