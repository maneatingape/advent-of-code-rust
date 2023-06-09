use aoc::year2020::day16::*;

const FIRST_EXAMPLE: &str = "\
class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12";

const SECOND_EXAMPLE: &str = "\
departure: 0-1 or 4-19
row: 0-5 or 8-19
seat: 0-13 or 16-19

your ticket:
11,12,13

nearby tickets:
3,9,18
15,1,5
5,14,9";

#[test]
fn part1_test() {
    let input = parse(FIRST_EXAMPLE);
    assert_eq!(part1(&input), 71);
}

#[test]
fn part2_test() {
    let input = parse(SECOND_EXAMPLE);
    assert_eq!(part2(&input), 12);
}
