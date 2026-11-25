use aoc::year2017::day11::*;

const EXAMPLE: &str = "se,sw,se,sw,sw,s,n";

#[test]
fn part1_test() {
    assert_eq!(part1(&parse(EXAMPLE)), 3);
}

#[test]
fn part2_test() {
    assert_eq!(part2(&parse(EXAMPLE)), 4);
}
