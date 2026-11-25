use aoc::year2016::day06::*;

const EXAMPLE: &str = "\
eedadn
drvtee
eandsr
raavrd
atevrs
tsrnev
sdttsa
rasrtv
nssdts
ntnada
svetve
tesnvt
vntsnd
vrdear
dvrsen
enarar";

#[test]
fn part1_test() {
    assert_eq!(part1(&parse(EXAMPLE)), "easter");
}

#[test]
fn part2_test() {
    assert_eq!(part2(&parse(EXAMPLE)), "advent");
}
