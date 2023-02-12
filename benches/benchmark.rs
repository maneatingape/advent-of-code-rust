#![feature(test)]
extern crate test;

macro_rules! benchmark {
    ($prefix:ident, $year:tt, $day:tt) => {
        mod $prefix {
            use aoc::$year::$day::*;
            use test::Bencher;

            const INPUT: &str = include_str!(concat![
                "../input/",
                stringify!($year),
                "/",
                stringify!($day),
                ".txt"
            ]);

            #[bench]
            fn parse_bench(b: &mut Bencher) {
                b.iter(|| parse(INPUT));
            }

            #[bench]
            fn part1_bench(b: &mut Bencher) {
                let input = parse(INPUT);
                b.iter(|| part1(&input));
            }

            #[bench]
            fn part2_bench(b: &mut Bencher) {
                let input = parse(INPUT);
                b.iter(|| part2(&input));
            }
        }
    };
}

// 2022
benchmark!(year2022_day01, year2022, day01);
benchmark!(year2022_day02, year2022, day02);
benchmark!(year2022_day03, year2022, day03);
benchmark!(year2022_day04, year2022, day04);
benchmark!(year2022_day05, year2022, day05);
benchmark!(year2022_day06, year2022, day06);
benchmark!(year2022_day07, year2022, day07);
benchmark!(year2022_day08, year2022, day08);
benchmark!(year2022_day09, year2022, day09);
benchmark!(year2022_day10, year2022, day10);
benchmark!(year2022_day11, year2022, day11);
benchmark!(year2022_day12, year2022, day12);
benchmark!(year2022_day13, year2022, day13);
benchmark!(year2022_day14, year2022, day14);
benchmark!(year2022_day15, year2022, day15);
benchmark!(year2022_day16, year2022, day16);
benchmark!(year2022_day17, year2022, day17);
benchmark!(year2022_day18, year2022, day18);
benchmark!(year2022_day19, year2022, day19);
benchmark!(year2022_day20, year2022, day20);
benchmark!(year2022_day21, year2022, day21);
benchmark!(year2022_day22, year2022, day22);
benchmark!(year2022_day23, year2022, day23);
benchmark!(year2022_day24, year2022, day24);
benchmark!(year2022_day25, year2022, day25);

// 2021
benchmark!(year2021_day01, year2021, day01);
benchmark!(year2021_day02, year2021, day02);
benchmark!(year2021_day03, year2021, day03);
benchmark!(year2021_day04, year2021, day04);
benchmark!(year2021_day05, year2021, day05);
benchmark!(year2021_day06, year2021, day06);
benchmark!(year2021_day07, year2021, day07);
benchmark!(year2021_day08, year2021, day08);
benchmark!(year2021_day09, year2021, day09);
benchmark!(year2021_day10, year2021, day10);
benchmark!(year2021_day11, year2021, day11);
benchmark!(year2021_day13, year2021, day13);
benchmark!(year2021_day14, year2021, day14);

// 2015
benchmark!(year2015_day01, year2015, day01);
benchmark!(year2015_day02, year2015, day02);
benchmark!(year2015_day03, year2015, day03);
//benchmark!(year2015_day04, year2015, day04); // Very slow
