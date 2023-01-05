use aoc::*;

fn main() {
    solutions()
        .iter()
        .filter(|s| s.year == 2022 && s.day == 10)
        .for_each(run);
}

fn run(solution: &Solution) {
    let Solution { year, day, input, wrapper } = solution;
    let (answer1, answer2) = (wrapper)(input);
    println!("Year {year} Day {day:02}");
    println!("    Part 1: {answer1}");
    println!("    Part 2: {answer2}");
}
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
                include_str!(concat!["../input/", stringify!($year), "/", stringify!($day), ".txt"])
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
        solution!(year2022, day07),
        solution!(year2022, day08),
        solution!(year2022, day09),
        solution!(year2022, day10),
    ]
}
