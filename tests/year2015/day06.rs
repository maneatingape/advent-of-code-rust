use aoc::year2015::day06::*;

const EXAMPLE1: &str = "\
turn on 0,0 through 999,999
toggle 0,0 through 999,0
turn off 499,499 through 500,500";

const EXAMPLE2: &str = "\
turn on 0,0 through 0,0
toggle 0,0 through 999,999";

#[test]
fn part1_test() {
    assert_eq!(part1(&parse(EXAMPLE1)), 998996);
}

#[test]
fn part2_test() {
    assert_eq!(part2(&parse(EXAMPLE2)), 2000001);
}
