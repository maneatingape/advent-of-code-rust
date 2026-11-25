use aoc::year2018::day13::*;

const EXAMPLE1: &str = r"/->-\         .
|   |  /----\ .
| /-+--+-\  | .
| | |  | v  | .
\-+-/  \-+--/ .
  \------/    .";

const EXAMPLE2: &str = r"/>-<\   .
|   |   .
| /<+-\ .
| | | v .
\>+</ | .
  |   ^ .
  \<->/ .";

#[test]
fn part1_test() {
    assert_eq!(part1(&parse(EXAMPLE1)), "7,3");
}

#[test]
fn part2_test() {
    assert_eq!(part2(&parse(EXAMPLE2)), "6,4");
}
