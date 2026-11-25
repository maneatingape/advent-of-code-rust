use aoc::year2016::day24::*;

const EXAMPLE: &str = "\
###########
#0.1.....2#
#.#######.#
#4.......3#
###########";

#[test]
fn part1_test() {
    assert_eq!(part1(&parse(EXAMPLE)), 14);
}

#[test]
fn part2_test() {
    assert_eq!(part2(&parse(EXAMPLE)), 20);
}
