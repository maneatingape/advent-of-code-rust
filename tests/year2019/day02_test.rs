use aoc::year2019::day02::*;

const EXAMPLE: &str = "\
1, 0, 0, 0,
2, 32, 0, 0,
2, 33, 1, 1,
2, 34, 2, 2,
1, 35, 0, 0,
1, 1, 0, 0,
1, 2, 0, 0,
99, 0, 0, 0,
0, 1000000, 10000, 7350720,
0, 0, 0, 0,
0, 0, 0, 0,
0, 0, 0, 0,
0, 0, 0, 0,
0, 0, 0, 0,
0, 0, 0, 0,
0, 0, 0, 0,
0, 0, 0, 0,
0, 0, 0, 0,
0, 0, 0, 0,
0, 0, 0, 0,
0, 0, 0, 0,
0, 0, 0, 0,
0, 0, 0, 0,
0, 0, 0, 0,
0, 0, 0, 0";

#[test]
fn part1_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part1(&input), 19370720);
}

#[test]
fn part2_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part2(&input), 1234);
}
