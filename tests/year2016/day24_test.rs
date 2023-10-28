use aoc::year2016::day24::*;

const EXAMPLE: &str = "\
###########
#0.1.....2#
#.#######.#
#4.......3#
###########";

#[test]
fn part1_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part1(&input), 14);
}

#[test]
fn part2_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part2(&input), 20);
}
