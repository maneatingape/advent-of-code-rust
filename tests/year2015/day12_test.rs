use aoc::year2015::day12::*;

const EXAMPLE: &str = r#"[1,{"c":"red","b":2},3]"#;

#[test]
fn part1_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part1(input), 6);
}

#[test]
fn part2_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part2(input), 4);
}
