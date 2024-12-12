use aoc::year2015::day14::*;

const EXAMPLE: &str = "\
Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.";

#[test]
fn part1_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part1_testable(&input, 1000), 1120);
}

#[test]
fn part2_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part2_testable(&input, 1000), 689);
}
