use aoc::year2020::day23::*;

const EXAMPLE: &str = "389125467";

#[test]
fn part1_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part1(&input), 67384529);
}

#[test]
fn part2_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part2(&input), 149245887792);
}
