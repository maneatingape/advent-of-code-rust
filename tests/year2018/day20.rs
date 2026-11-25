use aoc::year2018::day20::*;

const EXAMPLE: &str = "^WSSEESWWWNW(S|NENNEEEENN(ESSSSW(NWSW|SSEN)|WSWWN(E|WWS(E|SS))))$";

#[test]
fn part1_test() {
    assert_eq!(part1(&parse(EXAMPLE)), 31);
}

#[test]
fn part2_test() {
    assert_eq!(part2(&parse(EXAMPLE)), 0);
}
