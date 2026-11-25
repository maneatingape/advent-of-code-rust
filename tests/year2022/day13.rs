use aoc::year2022::day13::*;

const EXAMPLE: &str = "\
[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

#[test]
fn part1_test() {
    assert_eq!(part1(&parse(EXAMPLE)), 13);
}

#[test]
fn part2_test() {
    assert_eq!(part2(&parse(EXAMPLE)), 140);
}
