use aoc::year2020::day14::*;

const EXAMPLE1: &str = "\
mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0";

const EXAMPLE2: &str = "\
mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1";

#[test]
fn part1_test() {
    assert_eq!(part1(&parse(EXAMPLE1)), 165);
}

#[test]
fn part2_test() {
    assert_eq!(part2(&parse(EXAMPLE2)), 208);
}
