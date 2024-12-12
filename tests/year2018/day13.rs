use aoc::year2018::day13::*;

const FIRST_EXAMPLE: &str = r"/->-\         .
|   |  /----\ .
| /-+--+-\  | .
| | |  | v  | .
\-+-/  \-+--/ .
  \------/    .";

const SECOND_EXAMPLE: &str = r"/>-<\   .
|   |   .
| /<+-\ .
| | | v .
\>+</ | .
  |   ^ .
  \<->/ .";

#[test]
fn part1_test() {
    let input = parse(FIRST_EXAMPLE);
    assert_eq!(part1(&input), "7,3");
}

#[test]
fn part2_test() {
    let input = parse(SECOND_EXAMPLE);
    assert_eq!(part2(&input), "6,4");
}
