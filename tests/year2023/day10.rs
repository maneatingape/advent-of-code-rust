use aoc::year2023::day10::*;

const EXAMPLE1: &str = "\
.....
.S-7.
.|.|.
.L-J.
.....";

const EXAMPLE2: &str = "\
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
    assert_eq!(part1(&parse(EXAMPLE1)), 4);
}

#[test]
fn part2_test() {
    assert_eq!(part2(&parse(EXAMPLE2)), 4);
}
