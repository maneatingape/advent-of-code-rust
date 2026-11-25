use aoc::year2025::day11::*;

const EXAMPLE1: &str = "\
aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out";

const EXAMPLE2: &str = "\
svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out";

#[test]
fn part1_test() {
    assert_eq!(part1(&parse(EXAMPLE1)), 5);
}

#[test]
fn part2_test() {
    assert_eq!(part2(&parse(EXAMPLE2)), 2);
}
