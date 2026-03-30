use aoc::year2018::day17::*;

const EXAMPLE: &str = "\
x=495, y=2..7
y=7, x=495..501
x=501, y=3..7
x=498, y=2..4
x=506, y=1..2
x=498, y=10..13
x=504, y=10..13
y=13, x=498..504";

/// Some additional corner cases worth getting correct. This input produces a spread that ends
/// adjacent to another column that already contains Moving water, as well as finding a one-column
/// gap adjacent to unconnected clay that should still result in Moving water.
const EXTRA: &str = "\
x=500, y=2..3
x=501, y=5..6
y=7, x=501..502
x=504, y=1..6
y=9, x=503..504";

#[test]
fn part1_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part1(&input), 57);
    let input = parse(EXTRA);
    assert_eq!(part1(&input), 31);
}

#[test]
fn part2_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part2(&input), 29);
    let input = parse(EXTRA);
    assert_eq!(part2(&input), 0);
}
