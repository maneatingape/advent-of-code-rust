use aoc::year2020::day22::*;

const EXAMPLE: &str = "\
Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10";

#[test]
fn part1_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part1(&input), 306);
}

#[test]
fn part2_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part2(&input), 291);
}
