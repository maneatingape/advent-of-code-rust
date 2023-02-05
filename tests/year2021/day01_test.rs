use aoc::year2021::day01::*;

const EXAMPLE: &str =
"199
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
    let input = parse(EXAMPLE);
    assert_eq!(part1(&input), 7);
}

#[test]
fn part2_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part2(&input), 5);
}
