use aoc::year2023::day03::*;

const EXAMPLE: &str = "\
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

#[test]
fn part1_test() {
    assert_eq!(part1(&parse(EXAMPLE)), 4361);
}

#[test]
fn part2_test() {
    assert_eq!(part2(&parse(EXAMPLE)), 467835);
}
