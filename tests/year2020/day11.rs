use aoc::year2020::day11::*;

const EXAMPLE: &str = "\
L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";

#[test]
fn part1_test() {
    assert_eq!(part1(&parse(EXAMPLE)), 37);
}

#[test]
fn part2_test() {
    assert_eq!(part2(&parse(EXAMPLE)), 26);
}
