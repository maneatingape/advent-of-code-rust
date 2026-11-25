use aoc::year2021::day23::*;

const EXAMPLE: &str = "\
#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########";

#[test]
fn part1_test() {
    assert_eq!(part1(&parse(EXAMPLE)), 12521);
}

#[test]
fn part2_test() {
    assert_eq!(part2(&parse(EXAMPLE)), 44169);
}
