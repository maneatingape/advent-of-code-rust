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
    let input = parse(EXAMPLE);
    assert_eq!(part1(&input), 54);
}

#[test]
fn part2_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part2(&input), 4);
}
