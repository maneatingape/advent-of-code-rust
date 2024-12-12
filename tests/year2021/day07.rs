use aoc::year2021::day07::*;

const EXAMPLE: &str = "16,1,2,0,4,2,7,1,2,14";

#[test]
fn part1_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part1(&input), 37);
}

#[test]
fn part2_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part2(&input), 168);
}
