use aoc::year2015::day05::*;

const EXAMPLE1: &str = "\
ugknbfddgicrmopn
aaa
jchzalrnumimnmhp
haegwjzuvuyypxyu
dvszwmarrgswjxmb";

const EXAMPLE2: &str = "\
qjhvhtzxzqqjkmpb
xxyxx
uurcxstgmygtbstg
ieodomkazucvgmuy";

#[test]
fn part1_test() {
    assert_eq!(part1(&parse(EXAMPLE1)), 2);
}

#[test]
fn part2_test() {
    assert_eq!(part2(&parse(EXAMPLE2)), 2);
}
