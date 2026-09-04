#![allow(unstable_features)]
#![feature(test)]
extern crate test;

macro_rules! benchmark {
    ($($year:ident $description:literal $($day:ident)*),*) => {
        $(mod $year {
            $(mod $day {
                use std::fs::read_to_string;
                use std::sync::LazyLock;

                use test::Bencher;

                use aoc::util::ansi::*;
                use aoc::$year::$day::*;

                static DATA: LazyLock<String> = LazyLock::new(|| {
                    let path = format!("input/{}/{}.txt", stringify!($year), stringify!($day));
                    read_to_string(&path)
                        .unwrap_or_else(|_| panic!("Missing input file {BOLD}{WHITE}{path}{RESET}"))
                });

                #[bench]
                fn parse_bench(b: &mut Bencher) {
                    let input = &DATA;
                    b.iter(|| parse(input));
                }

                #[bench]
                fn part1_bench(b: &mut Bencher) {
                    let input = parse(&DATA);
                    b.iter(|| part1(&input));
                }

                #[bench]
                fn part2_bench(b: &mut Bencher) {
                    let input = parse(&DATA);
                    b.iter(|| part2(&input));
                }
            })*
        })*
    }
}

aoc::solutions!(benchmark);
