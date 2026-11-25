use aoc::year2018::day02::*;

const EXAMPLE1: &str = "\
abcdef
bababc
abbcde
abcccd
aabcdd
abcdee
ababab";

const EXAMPLE2: &str = "\
abcde
fghij
klmno
pqrst
fguij
axcye
wvxyz";

#[test]
fn part1_test() {
    assert_eq!(part1(&parse(EXAMPLE1)), 12);
}

#[test]
fn part2_test() {
    assert_eq!(part2(&parse(EXAMPLE2)), "fgij");
}
