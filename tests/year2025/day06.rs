use aoc::year2025::day06::*;

const EXAMPLE: &str = "\
123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";

#[test]
fn part1_test() {
    assert_eq!(part1(&parse(EXAMPLE)), 4277556);
}

#[test]
fn part2_test() {
    assert_eq!(part2(&parse(EXAMPLE)), 3263827);
}
