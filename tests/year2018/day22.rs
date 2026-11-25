use aoc::year2018::day22::*;

const EXAMPLE: &str = "\
depth: 510
target: 10,10";

// Alternate example found by random exploration. Not known to match any actual input file,
// but would get the wrong answer if SLOP_HEIGHT is too small.
const ALTERNATE: &str = "\
depth: 2451
target: 6,797";

#[test]
fn part1_test() {
    assert_eq!(part1(&parse(EXAMPLE)), 114);
}

#[test]
fn part2_test() {
    assert_eq!(part2(&parse(EXAMPLE)), 45);
    assert_eq!(part2(&parse(ALTERNATE)), 1100);
}
