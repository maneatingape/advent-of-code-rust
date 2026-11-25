use aoc::year2024::day25::*;

const EXAMPLE: &str = "\
#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####";

#[test]
fn part1_test() {
    assert_eq!(part1(parse(EXAMPLE)), 3);
}

#[test]
fn part2_test() {
    assert_eq!(part2(parse(EXAMPLE)), "n/a");
}
