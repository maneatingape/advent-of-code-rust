use aoc::year2019::day18::*;

const EXAMPLE1: &str = "\
#################
#i.G..c...e..H.p#
########.########
#j.A..b...f..D.o#
########@########
#k.E..a...g..B.n#
########.########
#l.F..d...h..C.m#
#################";

const EXAMPLE2: &str = "\
#############
#g#f.D#..h#l#
#F###e#E###.#
#dCba@#@BcIJ#
#############
#nK.L@#@G...#
#M###N#H###.#
#o#m..#i#jk.#
#############";

#[test]
fn part1_test() {
    assert_eq!(part1(&parse(EXAMPLE1)), 136);
}

#[test]
fn part2_test() {
    assert_eq!(part2(&parse(EXAMPLE2)), 72);
}
