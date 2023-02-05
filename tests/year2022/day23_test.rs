use aoc::year2022::day23::*;

const EXAMPLE: &str =
"....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#..";

#[test]
fn part1_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part1(&input), 110);
}

#[test]
fn part2_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part2(&input), 20);
}
