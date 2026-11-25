use aoc::year2019::day12::*;

const EXAMPLE: &str = "
<x=-8, y=-10, z=0>
<x=5, y=5, z=10>
<x=2, y=-7, z=3>
<x=9, y=-8, z=-3>";

#[test]
fn part1_test() {
    assert_eq!(part1(&parse(EXAMPLE)), 14645);
}

#[test]
fn part2_test() {
    assert_eq!(part2(&parse(EXAMPLE)), 4686774924);
}
