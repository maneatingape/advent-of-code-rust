use aoc::year2015::day07::*;

const EXAMPLE: &str = "\
123 -> b
456 -> c
b AND c -> d
b OR c -> e
d LSHIFT 2 -> f
e RSHIFT 2 -> g
f OR g -> h
NOT h -> a";

#[test]
fn part1_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part1(&input), 65153);
}

#[test]
fn part2_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part2(&input), 49165);
}
