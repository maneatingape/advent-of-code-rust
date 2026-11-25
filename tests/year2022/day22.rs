use aoc::year2022::day22::*;

const EXAMPLE: &str = "\
@       ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5";

#[test]
fn part1_test() {
    assert_eq!(part1(&parse(EXAMPLE)), 6032);
}

#[test]
fn part2_test() {
    assert_eq!(part2(&parse(EXAMPLE)), 5031);
}
