use aoc::year2024::day21::*;

const EXAMPLE: &str = "\
029A
980A
179A
456A
379A";

#[test]
fn part1_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part1(&input), 126384);
}

#[test]
fn part2_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part2(&input), 154115708116294);
}
