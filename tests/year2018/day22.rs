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
    let input = parse(EXAMPLE);
    assert_eq!(part1(&input), 114);
}

#[test]
fn part2_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part2(&input), 45);

    let input2 = parse(ALTERNATE);
    assert_eq!(part2(&input2), 1100);
}
