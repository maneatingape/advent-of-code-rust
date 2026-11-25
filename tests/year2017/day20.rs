use aoc::year2017::day20::*;

const EXAMPLE: &str = "\
p=<-6,0,0>, v=< 3,0,0>, a=< 0,0,0>
p=<-4,0,0>, v=< 2,0,0>, a=< 0,0,0>
p=<-2,0,0>, v=< 1,0,0>, a=< 0,0,0>
p=< 3,0,0>, v=<-1,0,0>, a=< 0,0,0>";

#[test]
fn part1_test() {
    assert_eq!(part1(&parse(EXAMPLE)), 3);
}

#[test]
fn part2_test() {
    assert_eq!(part2(&parse(EXAMPLE)), 1);
}
