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
    assert_eq!(part1(&parse(EXAMPLE)), "CABDFE");
}

#[test]
fn part2_test() {
    assert_eq!(part2_testable(&parse(EXAMPLE), 2, 0), 15);
}
