use aoc::year2017::day24::*;

const EXAMPLE: &str = "\
0/2
2/2
2/3
3/4
3/5
0/1
10/1
9/10";

#[test]
fn part1_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part1(&input), 31);
}

#[test]
fn part2_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part2(&input), 19);
}
