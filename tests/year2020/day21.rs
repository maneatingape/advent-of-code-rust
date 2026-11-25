use aoc::year2020::day21::*;

const EXAMPLE: &str = "\
mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)";

#[test]
fn part1_test() {
    assert_eq!(part1(&parse(EXAMPLE)), 5);
}

#[test]
fn part2_test() {
    assert_eq!(part2(&parse(EXAMPLE)), "mxmxvkd,sqjhc,fvjkl");
}
