use aoc::year2018::day03::*;

const EXAMPLE: &str = "\
#1 @ 1,3: 4x4
#2 @ 3,1: 4x4
#3 @ 5,5: 2x2";

#[test]
fn part1_test() {
    assert_eq!(part1(&parse(EXAMPLE)), 4);
}

#[test]
fn part2_test() {
    assert_eq!(part2(&parse(EXAMPLE)), 3);
}
