use aoc::year2022::day24::*;

const EXAMPLE: &str = "\
#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#";

#[test]
fn part1_test() {
    assert_eq!(part1(&parse(EXAMPLE)), 18);
}

#[test]
fn part2_test() {
    assert_eq!(part2(&parse(EXAMPLE)), 54);
}
