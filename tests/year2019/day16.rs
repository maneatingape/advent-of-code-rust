use aoc::year2019::day16::*;

const FIRST_EXAMPLE: &str = "80871224585914546619083218645595";
const SECOND_EXAMPLE: &str = "03036732577212944063491565474664";

#[test]
fn part1_test() {
    let input = parse(FIRST_EXAMPLE);
    assert_eq!(part1(&input), 24176176);
}

#[test]
fn part2_test() {
    let input = parse(SECOND_EXAMPLE);
    assert_eq!(part2(&input), 84462026);
}
