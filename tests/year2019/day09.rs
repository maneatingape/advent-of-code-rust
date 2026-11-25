use aoc::year2019::day09::*;

const EXAMPLE1: &str = "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99";
const EXAMPLE2: &str = "1102,34915192,34915192,7,4,7,99,0";

#[test]
fn part1_test() {
    assert_eq!(part1(&parse(EXAMPLE1)), 109);
}

#[test]
fn part2_test() {
    assert_eq!(part2(&parse(EXAMPLE2)), 1219070632396864);
}
