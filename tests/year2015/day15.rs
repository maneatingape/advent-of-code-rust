use aoc::year2015::day15::*;

const EXAMPLE: &str = "\
Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3
Filler 1: capacity -200, durability -300, flavor -600, texture -300, calories 0
Filler 2: capacity -200, durability -300, flavor -600, texture -300, calories 0";

#[test]
fn part1_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part1(&input), 62842880);
}

#[test]
fn part2_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part2(&input), 57600000);
}
