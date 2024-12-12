use aoc::year2017::day14::*;

const EXAMPLE: &str = "flqrgnkx";

#[test]
fn part1_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part1(&input), 8108);
}

#[test]
fn part2_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part2(&input), 1242);
}
