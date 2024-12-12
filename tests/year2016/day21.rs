use aoc::year2016::day21::*;

const EXAMPLE: &str = "\
swap position 4 with position 0
swap letter d with letter b
reverse positions 0 through 4
rotate left 1 step
move position 1 to position 4
move position 3 to position 0
rotate based on position of letter b
rotate based on position of letter d";

#[test]
fn part1_test() {
    let input = parse(EXAMPLE);
    assert_eq!(scramble(&input, b"abcde"), "decab");
}

#[test]
fn part2_test() {
    let input = parse(EXAMPLE);
    assert_eq!(unscramble(&input, b"decab"), "abcde");
}
