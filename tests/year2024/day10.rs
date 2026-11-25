use aoc::year2024::day10::*;

const EXAMPLE: &str = "\
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

#[test]
fn part1_test() {
    assert_eq!(part1(&parse(EXAMPLE)), 36);
}

#[test]
fn part2_test() {
    assert_eq!(part2(&parse(EXAMPLE)), 81);
}
