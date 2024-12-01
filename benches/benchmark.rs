#![allow(unstable_features)]
#![feature(test)]
extern crate test;

macro_rules! benchmark {
    ($year:tt, $day:tt) => {
        mod $day {
            use aoc::$year::$day::*;
            use std::fs::read_to_string;
            use std::path::Path;
            use std::sync::LazyLock;
            use test::Bencher;

            static DATA: LazyLock<String> = LazyLock::new(|| {
                let year = stringify!($year);
                let day = stringify!($day);
                let path = Path::new("input").join(year).join(day).with_extension("txt");
                read_to_string(path).unwrap()
            });

            #[bench]
            fn parse_bench(b: &mut Bencher) {
                let data = &DATA;
                b.iter(|| parse(data));
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
    benchmark!(year2017, day15);
    benchmark!(year2017, day16);
    benchmark!(year2017, day17);
    benchmark!(year2017, day18);
    benchmark!(year2017, day19);
    benchmark!(year2017, day20);
    benchmark!(year2017, day21);
    benchmark!(year2017, day22);
    benchmark!(year2017, day23);
    benchmark!(year2017, day24);
    benchmark!(year2017, day25);
}

mod year2018 {
    benchmark!(year2018, day01);
    benchmark!(year2018, day02);
    benchmark!(year2018, day03);
    benchmark!(year2018, day04);
    benchmark!(year2018, day05);
    benchmark!(year2018, day06);
    benchmark!(year2018, day07);
    benchmark!(year2018, day08);
    benchmark!(year2018, day09);
    benchmark!(year2018, day10);
    benchmark!(year2018, day11);
    benchmark!(year2018, day12);
    benchmark!(year2018, day13);
    benchmark!(year2018, day14);
    benchmark!(year2018, day15);
    benchmark!(year2018, day16);
    benchmark!(year2018, day17);
    benchmark!(year2018, day18);
    benchmark!(year2018, day19);
    benchmark!(year2018, day20);
    benchmark!(year2018, day21);
    benchmark!(year2018, day22);
    benchmark!(year2018, day23);
    benchmark!(year2018, day24);
    benchmark!(year2018, day25);
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
    benchmark!(year2023, day13);
    benchmark!(year2023, day14);
    benchmark!(year2023, day15);
    benchmark!(year2023, day16);
    benchmark!(year2023, day17);
    benchmark!(year2023, day18);
    benchmark!(year2023, day19);
    benchmark!(year2023, day20);
    benchmark!(year2023, day21);
    benchmark!(year2023, day22);
    benchmark!(year2023, day23);
    benchmark!(year2023, day24);
    benchmark!(year2023, day25);
}

mod year2024 {
    benchmark!(year2024, day01);
    benchmark!(year2024, day02);
}
