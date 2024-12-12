use aoc::year2023::day10::*;

const FIRST_EXAMPLE: &str = "\
.....
.S-7.
.|.|.
.L-J.
.....";

const SECOND_EXAMPLE: &str = "\
...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";

#[test]
fn part1_test() {
    let input = parse(FIRST_EXAMPLE);
    assert_eq!(part1(&input), 4);
}

#[test]
fn part2_test() {
    let input = parse(SECOND_EXAMPLE);
    assert_eq!(part2(&input), 4);
}
