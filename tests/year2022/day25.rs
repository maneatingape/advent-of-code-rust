use aoc::year2022::day25::*;

const EXAMPLE: &str = "\
1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122";

#[test]
fn part1_test() {
    assert_eq!(part1(&parse(EXAMPLE)), "2=-1=0");
}

#[test]
fn part2_test() {
    assert_eq!(part2(&parse(EXAMPLE)), "n/a");
}
