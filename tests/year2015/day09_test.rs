use aoc::year2015::day09::*;

const EXAMPLE: &str = "\
London to Dublin = 464
London to Belfast = 518
Dublin to Belfast = 141";

#[test]
fn part1_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part1(&input), 605);
}

#[test]
fn part2_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part2(&input), 982);
}
