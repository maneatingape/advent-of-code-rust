use aoc::year2019::day06::*;

const EXAMPLE: &str = "\
COM)BBB
BBB)CCC
CCC)DDD
DDD)EEE
EEE)FFF
BBB)GGG
GGG)HHH
DDD)III
EEE)JJJ
JJJ)KKK
KKK)LLL
KKK)YOU
III)SAN";

#[test]
fn part1_test() {
    assert_eq!(part1(&parse(EXAMPLE)), 54);
}

#[test]
fn part2_test() {
    assert_eq!(part2(&parse(EXAMPLE)), 4);
}
