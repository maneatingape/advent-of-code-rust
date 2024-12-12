use aoc::year2015::day06::*;

const FIRST_EXAMPLE: &str = "\
turn on 0,0 through 999,999
toggle 0,0 through 999,0
turn off 499,499 through 500,500";

const SECOND_EXAMPLE: &str = "\
turn on 0,0 through 0,0
toggle 0,0 through 999,999";

#[test]
fn part1_test() {
    let input = parse(FIRST_EXAMPLE);
    assert_eq!(part1(&input), 998996);
}

#[test]
fn part2_test() {
    let input = parse(SECOND_EXAMPLE);
    assert_eq!(part2(&input), 2000001);
}
