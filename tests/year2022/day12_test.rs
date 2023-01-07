use aoc::year2022::day12::*;

const EXAMPLE: &str =
"Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

#[test]
fn part1_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part1(&input), 31);
}

#[test]
fn part2_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part2(&input), 29);
}
