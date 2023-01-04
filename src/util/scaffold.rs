use crate::*;

pub struct Solution {
    pub year: u32,
    pub day: u8,
    pub input: &'static str,
    pub wrapper: fn(&str) -> (String, String),
}

macro_rules! solution {
    ($year:tt, $day:tt) => {
        Solution {
            year: {
                stringify!($year)[4..8].parse().unwrap()
            },
            day: {
                stringify!($day)[3..5].parse().unwrap()
            },
            input: {
                include_str!(concat!["../../input/", stringify!($year), "/", stringify!($day), ".txt"])
            },
            wrapper: |raw: &str| {
                use $year::$day::*;
                let input = parse(raw);
                let part1 = part1(&input).to_string();
                let part2 = part2(&input).to_string();
                (part1, part2)
            },
        }
    }
}

pub fn solutions() -> Vec<Solution> {
    vec![
        solution!(year2015, day01),
        solution!(year2015, day02),
        solution!(year2015, day03),
        //solution!(year2015, day04), // Very slow

        solution!(year2022, day01),
        solution!(year2022, day02),
        solution!(year2022, day03),
        solution!(year2022, day04),
        solution!(year2022, day05),
        solution!(year2022, day06),
    ]
}
