use aoc::year2019::day16::*;

const EXAMPLE1: &str = "80871224585914546619083218645595";
const EXAMPLE2: &str = "03036732577212944063491565474664";

#[test]
fn part1_test() {
    assert_eq!(part1(&parse(EXAMPLE1)), 24176176);
}

#[test]
fn part2_test() {
    assert_eq!(part2(&parse(EXAMPLE2)), 84462026);
}
