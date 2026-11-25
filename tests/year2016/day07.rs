use aoc::year2016::day07::*;

const EXAMPLE1: &str = "\
abba[mnop]qrst
abcd[bddb]xyyx
aaaa[qwer]tyui
ioxxoj[asdfgh]zxcvbn";

const EXAMPLE2: &str = "\
aba[bab]xyz
xyx[xyx]xyx
aaa[kek]eke
zazbz[bzb]cdb";

#[test]
fn part1_test() {
    assert_eq!(part1(&parse(EXAMPLE1)), 2);
}

#[test]
fn part2_test() {
    assert_eq!(part2(&parse(EXAMPLE2)), 3);
}
