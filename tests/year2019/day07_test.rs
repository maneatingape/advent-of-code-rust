use aoc::year2019::day07::*;

const FIRST_EXAMPLE: &str = "\
3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0";

const SECOND_EXAMPLE: &str = "\
3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,
27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5";

#[test]
fn part1_test() {
    let input = parse(FIRST_EXAMPLE);
    assert_eq!(part1(&input), 43210);
}

#[test]
fn part2_test() {
    let input = parse(SECOND_EXAMPLE);
    assert_eq!(part2(&input), 139629729);
}
