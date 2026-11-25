use aoc::year2024::day17::*;

const EXAMPLE1: &str = "\
Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";

const EXAMPLE2: &str = "\
Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0";

#[test]
fn part1_test() {
    assert_eq!(part1(&parse(EXAMPLE1)), "4,6,3,5,6,3,5,2,1,0");
}

#[test]
fn part2_test() {
    assert_eq!(part2(&parse(EXAMPLE2)), 117440);
}
