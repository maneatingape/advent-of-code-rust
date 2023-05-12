use aoc::year2021::day23::*;

const EXAMPLE: &str = "\
#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########";

#[test]
fn part1_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part1(&input), 12521);
}

#[test]
fn part2_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part2(&input), 44169);
}
