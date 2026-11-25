use aoc::year2021::day25::*;

const EXAMPLE: &str = "\
v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>";

#[test]
fn part1_test() {
    assert_eq!(part1(&parse(EXAMPLE)), 58);
}

#[test]
fn part2_test() {
    assert_eq!(part2(&parse(EXAMPLE)), "n/a");
}
