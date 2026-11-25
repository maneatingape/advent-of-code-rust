use aoc::year2022::day09::*;

const EXAMPLE: &str = "\
R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

#[test]
fn part1_test() {
    assert_eq!(part1(&parse(EXAMPLE)), 13);
}

#[test]
fn part2_test() {
    assert_eq!(part2(&parse(EXAMPLE)), 1);
}
