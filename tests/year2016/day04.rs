use aoc::year2016::day04::*;

const EXAMPLE: &str = "\
aaaaa-bbb-z-y-x-123[abxyz]
a-b-c-d-e-f-g-h-987[abcde]
not-a-real-room-404[oarel]
totally-real-room-200[decoy]";

// The problem merely states "five most common letters in the encrypted name, in order, with
// ties broken by alphabetization".  This is ambiguous on whether the tie must have been broken
// by the first such letter, or whether we merely need the letters present in the checksum in
// order regardless of the order of skipped letters.  Go with the looser interpretation, since
// it gives slightly faster code, and no one has reported an input file where it fails.
const EXAMPLE2: &str = "\
a-b-c-d-e-f-g-h-001[bcdef]
a-b-c-d-e-f-g-h-002[abcef]
aaaaa-bbbbb-c-d-e-f-g-h-004[acdef]";

#[test]
fn part1_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part1(&input), 1514);
    let input = parse(EXAMPLE2);
    assert_eq!(part1(&input), 3);
}

#[test]
fn part2_test() {
    // No example data
}
