use aoc::year2024::day12::*;

const FIRST_EXAMPLE: &str = "\
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

const SECOND_EXAMPLE: &str = "\
AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA";

#[test]
fn part1_test() {
    let input = parse(FIRST_EXAMPLE);
    assert_eq!(part1(&input), 1930);
}

#[test]
fn part2_test() {
    let input = parse(FIRST_EXAMPLE);
    assert_eq!(part2(&input), 1206);
    let input = parse(SECOND_EXAMPLE);
    assert_eq!(part2(&input), 368);
}
