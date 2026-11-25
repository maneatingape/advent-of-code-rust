use aoc::year2024::day12::*;

const EXAMPLE1: &str = "\
RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

const EXAMPLE2: &str = "\
AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA";

#[test]
fn part1_test() {
    assert_eq!(part1(&parse(EXAMPLE1)), 1930);
}

#[test]
fn part2_test() {
    assert_eq!(part2(&parse(EXAMPLE1)), 1206);
    assert_eq!(part2(&parse(EXAMPLE2)), 368);
}
