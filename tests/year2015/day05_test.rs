use aoc::year2015::day05::*;

const FIRST_EXAMPLE: &str = "\
ugknbfddgicrmopn
aaa
jchzalrnumimnmhp
haegwjzuvuyypxyu
dvszwmarrgswjxmb";

const SECOND_EXAMPLE: &str = "\
qjhvhtzxzqqjkmpb
xxyxx
uurcxstgmygtbstg
ieodomkazucvgmuy";

#[test]
fn part1_test() {
    let input = parse(FIRST_EXAMPLE);
    assert_eq!(part1(&input), 2);
}

#[test]
fn part2_test() {
    let input = parse(SECOND_EXAMPLE);
    assert_eq!(part2(&input), 2);
}
