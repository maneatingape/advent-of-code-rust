use aoc::year2022::day04::*;

const EXAMPLE: &str =
"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

#[test]
fn part1_example() {
    let input = parse(EXAMPLE);
    assert_eq!(part1(&input), 2);
}

#[test]
fn part2_example() {
    let input = parse(EXAMPLE);
    assert_eq!(part2(&input), 4);
}
