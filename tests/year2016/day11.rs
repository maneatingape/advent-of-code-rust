use aoc::year2016::day11::*;

// From the puzzle, valid for part 1.
const EXAMPLE: &str = "\
The first floor contains a hydrogen-compatible microchip and a lithium-compatible microchip.
The second floor contains a hydrogen generator.
The third floor contains a lithium generator.
The fourth floor contains nothing relevant.";

// Reduced demo where a solution that merely tracks the number of items per floor would result
// in a too-low result, because it picks a step that would lead to fried microchips.
const INTERLOCKED: &str = "\
The first floor contains a lithium generator and a lithium-compatible microchip.
The second floor contains a hydrogen generator and an elerium generator and a dilithium generator.
The third floor contains a hydrogen-compatible microchip and an elerium-compatible microchip and a dilithium-compatible microchip.
The fourth floor contains nothing relevant.";

#[test]
fn part1_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part1(&input), 11);
    let input = parse(INTERLOCKED);
    assert_eq!(part1(&input), 25);
}

#[test]
fn part2_test() {
    // Technically, EXAMPLE has no valid part2 solution. Even if you disregard the unprotected
    // chips on floor 1 when the two new pairs are added at step 0, the only viable move to step
    // 1 would be moving both generators; but since there are no other chips on higher floors,
    // there is nothing available that can be used to get the elevator back to the first floor
    // without frying the unprotected microchips.
    let input = parse(INTERLOCKED);
    assert_eq!(part2(&input), 49);
}
