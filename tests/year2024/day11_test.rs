use aoc::year2024::day11::*;

const EXAMPLE: &str = "125 17";

#[test]
fn part1_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part1(&input), 55312);
}

#[test]
fn part2_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part2(&input), 65601038650482);
}
