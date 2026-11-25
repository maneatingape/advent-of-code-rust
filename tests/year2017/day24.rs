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
    assert_eq!(part1(&parse(EXAMPLE)), 31);
}

#[test]
fn part2_test() {
    assert_eq!(part2(&parse(EXAMPLE)), 19);
}
