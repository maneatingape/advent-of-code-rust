use aoc::year2024::day07::*;

const EXAMPLE: &str = "\
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

const EXAMPLE2: &str = "\
190: 10 19
11174: 15 8 9 79 74
729: 6 6 7 37 650";

#[test]
fn part1_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part1(&input), 3749);
    let input2 = parse(EXAMPLE2);
    assert_eq!(part1(&input2), 190);
}

#[test]
fn part2_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part2(&input), 11387);
    let input2 = parse(EXAMPLE2);
    assert_eq!(part2(&input2), 11364);
}
