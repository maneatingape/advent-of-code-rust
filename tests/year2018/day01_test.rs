use aoc::year2018::day01::*;

const EXAMPLE: &str = "\
+1
-2
+3
+1";

#[test]
fn part1_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part1(&input), 3);
}

#[test]
fn part2_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part2(&input), 2);
}
