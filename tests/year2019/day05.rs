use aoc::year2019::day05::*;

const EXAMPLE1: &str = "3,0,4,0,99";

const EXAMPLE2: &str = "\
3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,
1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99";

#[test]
fn part1_test() {
    assert_eq!(part1(&parse(EXAMPLE1)), 1);
}

#[test]
fn part2_test() {
    assert_eq!(part2(&parse(EXAMPLE2)), 999);
}
