use aoc::year2023::day14::*;

const EXAMPLE: &str = "\
O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

#[test]
fn part1_test() {
    assert_eq!(part1(&parse(EXAMPLE)), 136);
}

#[test]
fn part2_test() {
    assert_eq!(part2(&parse(EXAMPLE)), 64);
}
