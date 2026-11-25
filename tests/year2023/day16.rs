use aoc::year2023::day16::*;

const EXAMPLE: &str = "\
.|...\\....
|.-.\\.....
.....|-...
........|.
..........
.........\\
..../.\\\\..
.-.-/..|..
.|....-|.\\
..//.|....";

#[test]
fn part1_test() {
    assert_eq!(part1(&parse(EXAMPLE)), 46);
}

#[test]
fn part2_test() {
    assert_eq!(part2(&parse(EXAMPLE)), 51);
}
