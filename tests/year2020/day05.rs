use aoc::year2020::day05::*;

const EXAMPLE: &str = "\
FBFBBFFLRR
FBFBBFFRLR";

#[test]
fn part1_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part1(&input), 357);
}

#[test]
fn part2_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part2(&input), 356);
}
