use aoc::year2022::day01::*;

const EXAMPLE: &str = "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000";

#[test]
fn part1_example() {
    let input = parse(EXAMPLE);
    assert_eq!(part1(&input), 24000);
}

#[test]
fn part2_example() {
    let input = parse(EXAMPLE);
    assert_eq!(part2(&input), 45000);
}
