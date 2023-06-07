use aoc::year2020::day13::*;

const EXAMPLE: &str = "\
939
7,13,x,x,59,x,31,19";

#[test]
fn part1_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part1(&input), 295);
}

#[test]
fn part2_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part2(&input), 1068781);
}
