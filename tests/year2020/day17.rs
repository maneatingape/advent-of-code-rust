use aoc::year2020::day17::*;

const EXAMPLE: &str = "\
.#.
..#
###";

#[test]
fn part1_test() {
    assert_eq!(part1(&parse(EXAMPLE)), 112);
}

#[test]
fn part2_test() {
    assert_eq!(part2(&parse(EXAMPLE)), 848);
}
