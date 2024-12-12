use aoc::year2019::day03::*;

const EXAMPLE: &str = "\
R8,U5,L5,D3
U7,R6,D4,L4";

#[test]
fn part1_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part1(&input), 6);
}

#[test]
fn part2_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part2(&input), 30);
}
