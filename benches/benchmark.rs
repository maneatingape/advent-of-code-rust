#![feature(test)]
extern crate test;

macro_rules! benchmark {
    ($year:tt, $day:tt) => {
        mod $day {
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

mod year2022 {
    benchmark!(year2022, day01);
    benchmark!(year2022, day02);
    benchmark!(year2022, day03);
    benchmark!(year2022, day04);
    benchmark!(year2022, day05);
    benchmark!(year2022, day06);
    benchmark!(year2022, day07);
    benchmark!(year2022, day08);
    benchmark!(year2022, day09);
    benchmark!(year2022, day10);
    benchmark!(year2022, day11);
    benchmark!(year2022, day12);
    benchmark!(year2022, day13);
    benchmark!(year2022, day14);
    benchmark!(year2022, day15);
    benchmark!(year2022, day16);
    benchmark!(year2022, day17);
    benchmark!(year2022, day18);
    benchmark!(year2022, day19);
    benchmark!(year2022, day20);
    benchmark!(year2022, day21);
    benchmark!(year2022, day22);
    benchmark!(year2022, day23);
    benchmark!(year2022, day24);
    benchmark!(year2022, day25);
}

mod year2021 {
    benchmark!(year2021, day01);
    benchmark!(year2021, day02);
    benchmark!(year2021, day03);
    benchmark!(year2021, day04);
    benchmark!(year2021, day05);
    benchmark!(year2021, day06);
    benchmark!(year2021, day07);
    benchmark!(year2021, day08);
    benchmark!(year2021, day09);
    benchmark!(year2021, day10);
    benchmark!(year2021, day11);
    benchmark!(year2021, day12);
    benchmark!(year2021, day13);
    benchmark!(year2021, day14);
    benchmark!(year2021, day15);
    benchmark!(year2021, day16);
    benchmark!(year2021, day17);
    benchmark!(year2021, day18);
    benchmark!(year2021, day19);
    benchmark!(year2021, day20);
    benchmark!(year2021, day21);
    benchmark!(year2021, day22);
    benchmark!(year2021, day23);
    benchmark!(year2021, day24);
    benchmark!(year2021, day25);
}

mod year2020 {
    benchmark!(year2020, day01);
    benchmark!(year2020, day02);
    benchmark!(year2020, day03);
    benchmark!(year2020, day04);
    benchmark!(year2020, day05);
    benchmark!(year2020, day06);
    benchmark!(year2020, day07);
    benchmark!(year2020, day08);
    benchmark!(year2020, day09);
    benchmark!(year2020, day10);
    benchmark!(year2020, day11);
    benchmark!(year2020, day12);
    benchmark!(year2020, day13);
    benchmark!(year2020, day14);
    // benchmark!(year2020, day15); // Very slow
    benchmark!(year2020, day16);
}

mod year2015 {
    benchmark!(year2015, day01);
    benchmark!(year2015, day02);
    benchmark!(year2015, day03);
    //benchmark!(year2015, day04); // Very slow
    benchmark!(year2015, day05);
    benchmark!(year2015, day06);
    benchmark!(year2015, day07);
    benchmark!(year2015, day08);
    benchmark!(year2015, day09);
    benchmark!(year2015, day10);
}
