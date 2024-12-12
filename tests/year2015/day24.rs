use aoc::year2015::day24::*;

const EXAMPLE: &str = "1 2 3 4 5 7 8 9 10 11";

#[test]
fn part1_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part1(&input), 99);
}

#[test]
fn part2_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part2(&input), 44);
}
