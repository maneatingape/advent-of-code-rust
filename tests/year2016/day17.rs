use aoc::year2016::day17::*;

const EXAMPLE: &str = "ihgpwlah";

#[test]
fn part1_test() {
    assert_eq!(part1(&parse(EXAMPLE)), "DDRRRD");
}

#[test]
fn part2_test() {
    assert_eq!(part2(&parse(EXAMPLE)), 370);
}
