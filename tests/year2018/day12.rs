use aoc::year2018::day12::*;

const EXAMPLE: &str = "\
initial state: #..#.#..##......###...###

...## => #
..#.. => #
.#... => #
.#.#. => #
.#.## => #
.##.. => #
.#### => #
#.#.# => #
#.### => #
##.#. => #
##.## => #
###.. => #
###.# => #
####. => #";

#[test]
fn part1_test() {
    assert_eq!(part1(&parse(EXAMPLE)), 325);
}

#[test]
fn part2_test() {
    assert_eq!(part2(&parse(EXAMPLE)), 999999999374);
}
