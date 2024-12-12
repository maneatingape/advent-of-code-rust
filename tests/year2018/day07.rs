use aoc::year2018::day07::*;

const EXAMPLE: &str = "\
Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.";

#[test]
fn part1_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part1(&input), "CABDFE");
}

#[test]
fn part2_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part2_testable(&input, 2, 0), 15);
}
