use crate::*;

pub struct Solution {
    pub year: u32,
    pub day: u8,
    pub raw: &'static str,
    pub wrapper: fn(&str) -> (String, String),
}

macro_rules! solution {
    ($year:literal, $day:literal, $file:literal, $year_token:tt, $day_token:tt) => {
        Solution {
            year: $year,
            day: $day,
            raw: include_str!($file),
            wrapper: |raw: &str| {
                let input = $year_token::$day_token::parse(raw);
                let part1 = $year_token::$day_token::part1(&input).to_string();
                let part2 = $year_token::$day_token::part2(&input).to_string();
                (part1, part2)
            },
        }
    }
}

pub fn solutions() -> Vec<Solution> {
    vec![
        solution!(2015,  1, "../../input/year2015/day01.txt", year2015, day01),
        solution!(2015,  2, "../../input/year2015/day02.txt", year2015, day02),
        solution!(2015,  3, "../../input/year2015/day03.txt", year2015, day03),
        //solution!(2015,  4, "../../input/year2015/day04.txt", year2015, day04), // Very slow

        solution!(2022,  1, "../../input/year2022/day01.txt", year2022, day01),
        solution!(2022,  2, "../../input/year2022/day02.txt", year2022, day02),
    ]
}
