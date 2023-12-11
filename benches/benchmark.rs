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

mod year2015 {
    benchmark!(year2015, day01);
    benchmark!(year2015, day02);
    benchmark!(year2015, day03);
    benchmark!(year2015, day04);
    benchmark!(year2015, day05);
    benchmark!(year2015, day06);
    benchmark!(year2015, day07);
    benchmark!(year2015, day08);
    benchmark!(year2015, day09);
    benchmark!(year2015, day10);
    benchmark!(year2015, day11);
    benchmark!(year2015, day12);
    benchmark!(year2015, day13);
    benchmark!(year2015, day14);
    benchmark!(year2015, day15);
    benchmark!(year2015, day16);
    benchmark!(year2015, day17);
    benchmark!(year2015, day18);
    benchmark!(year2015, day19);
    benchmark!(year2015, day20);
    benchmark!(year2015, day21);
    benchmark!(year2015, day22);
    benchmark!(year2015, day23);
    benchmark!(year2015, day24);
    benchmark!(year2015, day25);
}

mod year2016 {
    benchmark!(year2016, day01);
    benchmark!(year2016, day02);
    benchmark!(year2016, day03);
    benchmark!(year2016, day04);
    benchmark!(year2016, day05);
    benchmark!(year2016, day06);
    benchmark!(year2016, day07);
    benchmark!(year2016, day08);
    benchmark!(year2016, day09);
    benchmark!(year2016, day10);
    benchmark!(year2016, day11);
    benchmark!(year2016, day12);
    benchmark!(year2016, day13);
    benchmark!(year2016, day14);
    benchmark!(year2016, day15);
    benchmark!(year2016, day16);
    benchmark!(year2016, day17);
    benchmark!(year2016, day18);
    benchmark!(year2016, day19);
    benchmark!(year2016, day20);
    benchmark!(year2016, day21);
    benchmark!(year2016, day22);
    benchmark!(year2016, day23);
    benchmark!(year2016, day24);
    benchmark!(year2016, day25);
}

mod year2017 {
    benchmark!(year2017, day01);
    benchmark!(year2017, day02);
    benchmark!(year2017, day03);
    benchmark!(year2017, day04);
    benchmark!(year2017, day05);
    benchmark!(year2017, day06);
    benchmark!(year2017, day07);
    benchmark!(year2017, day08);
    benchmark!(year2017, day09);
    benchmark!(year2017, day10);
    benchmark!(year2017, day11);
    benchmark!(year2017, day12);
    benchmark!(year2017, day13);
    benchmark!(year2017, day14);
}

mod year2019 {
    benchmark!(year2019, day01);
    benchmark!(year2019, day02);
    benchmark!(year2019, day03);
    benchmark!(year2019, day04);
    benchmark!(year2019, day05);
    benchmark!(year2019, day06);
    benchmark!(year2019, day07);
    benchmark!(year2019, day08);
    benchmark!(year2019, day09);
    benchmark!(year2019, day10);
    benchmark!(year2019, day11);
    benchmark!(year2019, day12);
    benchmark!(year2019, day13);
    benchmark!(year2019, day14);
    benchmark!(year2019, day15);
    benchmark!(year2019, day16);
    benchmark!(year2019, day17);
    benchmark!(year2019, day18);
    benchmark!(year2019, day19);
    benchmark!(year2019, day20);
    benchmark!(year2019, day21);
    benchmark!(year2019, day22);
    benchmark!(year2019, day23);
    benchmark!(year2019, day24);
    benchmark!(year2019, day25);
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
    benchmark!(year2020, day15);
    benchmark!(year2020, day16);
    benchmark!(year2020, day17);
    benchmark!(year2020, day18);
    benchmark!(year2020, day19);
    benchmark!(year2020, day20);
    benchmark!(year2020, day21);
    benchmark!(year2020, day22);
    benchmark!(year2020, day23);
    benchmark!(year2020, day24);
    benchmark!(year2020, day25);
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

mod year2023 {
    benchmark!(year2023, day01);
    benchmark!(year2023, day02);
    benchmark!(year2023, day03);
    benchmark!(year2023, day04);
    benchmark!(year2023, day05);
    benchmark!(year2023, day06);
    benchmark!(year2023, day07);
    benchmark!(year2023, day08);
    benchmark!(year2023, day09);
    benchmark!(year2023, day10);
    benchmark!(year2023, day11);
    benchmark!(year2023, day12);
}
