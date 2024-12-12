use aoc::year2019::day09::*;

const FIRST_EXAMPLE: &str = "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99";
const SECOND_EXAMPLE: &str = "1102,34915192,34915192,7,4,7,99,0";

#[test]
fn part1_test() {
    let input = parse(FIRST_EXAMPLE);
    assert_eq!(part1(&input), 109);
}

#[test]
fn part2_test() {
    let input = parse(SECOND_EXAMPLE);
    assert_eq!(part2(&input), 1219070632396864);
}
