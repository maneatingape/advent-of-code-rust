use aoc::year2017::day11::*;

const EXAMPLE: &str = "se,sw,se,sw,sw,s,n";

#[test]
fn part1_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part1(&input), 3);
}

#[test]
fn part2_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part2(&input), 4);
}
