use aoc::year2018::day01::*;

const EXAMPLE: &str = "\
+1
-2
+3
+1";

#[test]
fn part1_test() {
    assert_eq!(part1(&parse(EXAMPLE)), 3);
}

#[test]
fn part2_test() {
    assert_eq!(part2(&parse(EXAMPLE)), 2);
}
