use aoc::year2020::day08::*;

const EXAMPLE: &str = "\
nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";

#[test]
fn part1_test() {
    assert_eq!(part1(&parse(EXAMPLE)), 5);
}

#[test]
fn part2_test() {
    assert_eq!(part2(&parse(EXAMPLE)), 8);
}
