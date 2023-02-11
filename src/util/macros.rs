pub struct Solution {
    pub year: u32,
    pub day: u8,
    pub input: &'static str,
    pub wrapper: fn(&str) -> (String, String),
}

#[macro_export]
macro_rules! solution {
    ($year:tt, $day:tt) => {
        Solution {
            year: { stringify!($year)[4..8].parse().unwrap() },
            day: { stringify!($day)[3..5].parse().unwrap() },
            input: {
                include_str!(concat![
                    "../input/",
                    stringify!($year),
                    "/",
                    stringify!($day),
                    ".txt"
                ])
            },
            wrapper: {
                |raw: &str| {
                    use $year::$day::*;
                    let input = parse(raw);
                    let part1 = part1(&input).to_string();
                    let part2 = part2(&input).to_string();
                    (part1, part2)
                }
            },
        }
    };
}

#[macro_export]
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
