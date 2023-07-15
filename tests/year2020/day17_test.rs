use aoc::year2020::day17::*;

const EXAMPLE: &str = "\
.#.
..#
###";

#[test]
fn part1_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part1(&input), 112);
}

#[test]
fn part2_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part2(&input), 848);
}
