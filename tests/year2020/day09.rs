use aoc::year2020::day09::*;

const EXAMPLE: &str = "\
35 20 15 25 47
40 62 55 65 95
102 117 150 182 127
219 299 277 309 576";

#[test]
fn part1_test() {
    let (part_one, _) = decrypt::<5>(EXAMPLE);
    assert_eq!(part_one, 127);
}

#[test]
fn part2_test() {
    let (_, part_two) = decrypt::<5>(EXAMPLE);
    assert_eq!(part_two, 62);
}
