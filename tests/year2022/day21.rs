use aoc::year2022::day21::*;

const EXAMPLE: &str = "\
root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32";

#[test]
fn part1_test() {
    assert_eq!(part1(&parse(EXAMPLE)), 152);
}

#[test]
fn part2_test() {
    assert_eq!(part2(&parse(EXAMPLE)), 301);
}
