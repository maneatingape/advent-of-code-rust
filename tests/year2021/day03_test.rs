use aoc::year2021::day03::*;

const EXAMPLE: &str = "\
00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

#[test]
fn part1_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part1(&input), 198);
}

#[test]
fn part2_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part2(&input), 230);
}
