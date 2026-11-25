use aoc::year2021::day11::*;

const EXAMPLE: &str = "\
5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";

#[test]
fn part1_test() {
    assert_eq!(part1(&parse(EXAMPLE)), 1656);
}

#[test]
fn part2_test() {
    assert_eq!(part2(&parse(EXAMPLE)), 195);
}
