use aoc::year2021::day09::*;

const EXAMPLE: &str = "\
2199943210
3987894921
9856789892
8767896789
9899965678";

#[test]
fn part1_test() {
    assert_eq!(part1(&parse(EXAMPLE)), 15);
}

#[test]
fn part2_test() {
    assert_eq!(part2(&parse(EXAMPLE)), 1134);
}
