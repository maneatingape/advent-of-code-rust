use aoc::year2017::day12::*;

const EXAMPLE: &str = "\
0 <-> 2
1 <-> 1
2 <-> 0, 3, 4
3 <-> 2, 4
4 <-> 2, 3, 6
5 <-> 6
6 <-> 4, 5";

#[test]
fn part1_test() {
    assert_eq!(part1(&parse(EXAMPLE)), 6);
}

#[test]
fn part2_test() {
    assert_eq!(part2(&parse(EXAMPLE)), 2);
}
