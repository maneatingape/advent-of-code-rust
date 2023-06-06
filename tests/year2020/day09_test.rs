use aoc::year2020::day09::*;

const EXAMPLE: &str = "\
35 20 15 25 47
40 62 55 65 95
102 117 150 182 127
219 299 277 309 576";

#[test]
fn part1_test() {
    let result = decrypt::<5>(EXAMPLE);
    assert_eq!(result.0, 127);
}

#[test]
fn part2_test() {
    let result = decrypt::<5>(EXAMPLE);
    assert_eq!(result.1, 62);
}
