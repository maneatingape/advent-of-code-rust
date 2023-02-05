use aoc::year2021::day02::*;

const EXAMPLE: &str =
"forward 5
down 5
forward 8
up 3
down 8
forward 2";

#[test]
fn part1_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part1(&input), 150);
}

#[test]
fn part2_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part2(&input), 900);
}
