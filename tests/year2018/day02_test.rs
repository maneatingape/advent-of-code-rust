use aoc::year2018::day02::*;

const FIRST_EXAMPLE: &str = "\
abcdef
bababc
abbcde
abcccd
aabcdd
abcdee
ababab";

const SECOND_EXAMPLE: &str = "\
abcde
fghij
klmno
pqrst
fguij
axcye
wvxyz";

#[test]
fn part1_test() {
    let input = parse(FIRST_EXAMPLE);
    assert_eq!(part1(&input), 12);
}

#[test]
fn part2_test() {
    let input = parse(SECOND_EXAMPLE);
    assert_eq!(part2(&input), "fgij");
}
