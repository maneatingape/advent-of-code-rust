use aoc::year2023::day12::*;

const EXAMPLE: &str = "\
???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

#[test]
fn part1_test() {
    assert_eq!(part1(&parse(EXAMPLE)), 21);
}

#[test]
fn part2_test() {
    assert_eq!(part2(&parse(EXAMPLE)), 525152);
}
