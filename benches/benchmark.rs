#![feature(test)]
extern crate test;

macro_rules! bench {
    ($prefix:ident, $year:tt, $day:tt) => {
        mod $prefix {
            use test::Bencher;
            use aoc::$year::$day::*;

            const RAW: &str = include_str!(concat!["../input/", stringify!($year), "/", stringify!($day), ".txt"]);

            #[bench]
            fn parse_bench(b: &mut Bencher) {
                b.iter(|| parse(RAW));
            }

            #[bench]
            fn part1_bench(b: &mut Bencher) {
                let input = parse(RAW);
                b.iter(|| part1(&input));
            }

            #[bench]
            fn part2_bench(b: &mut Bencher) {
                let input = parse(RAW);
                b.iter(|| part2(&input));
            }
        }
    }
}

bench!(year2015_day01, year2022, day01);
bench!(year2015_day02, year2022, day02);
bench!(year2015_day03, year2022, day03);
//bench!(year2015_day04, year2022, day04); // Too slow for benchmark

bench!(year2022_day01, year2022, day01);
bench!(year2022_day02, year2022, day02);
bench!(year2022_day03, year2022, day03);
bench!(year2022_day04, year2022, day04);
bench!(year2022_day05, year2022, day05);
bench!(year2022_day06, year2022, day06);
bench!(year2022_day07, year2022, day07);
bench!(year2022_day08, year2022, day08);
bench!(year2022_day09, year2022, day09);
bench!(year2022_day10, year2022, day10);
bench!(year2022_day11, year2022, day11);
