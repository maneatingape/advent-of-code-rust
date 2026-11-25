use aoc::year2025::day04::*;

const EXAMPLE: &str = "\
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

#[test]
fn part1_test() {
    assert_eq!(part1(&parse(EXAMPLE)), 13);
}

#[test]
fn part2_test() {
    assert_eq!(part2(&parse(EXAMPLE)), 43);
}
