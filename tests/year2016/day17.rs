use aoc::year2016::day17::*;

const EXAMPLE: &str = "ihgpwlah";

#[test]
fn part1_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part1(&input), "DDRRRD");
}

#[test]
fn part2_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part2(&input), 370);
}
