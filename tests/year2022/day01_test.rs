use aoc::year2022::day01::*;

const EXAMPLE: &str =
"1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

#[test]
fn part1_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part1(&input), 24000);
}

#[test]
fn part2_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part2(&input), 45000);
}
