use aoc::year2021::day01::*;

const EXAMPLE: &str = "\
199
200
208
210
200
207
240
269
260
263";

#[test]
fn part1_test() {
    assert_eq!(part1(&parse(EXAMPLE)), 7);
}

#[test]
fn part2_test() {
    assert_eq!(part2(&parse(EXAMPLE)), 5);
}
