use ansi::*;
use aoc::*;
use std::env::args;
use std::time::Instant;

/// ANSI escape codes
mod ansi {
    pub const RESET: &str = "\x1b[0m";
    pub const BOLD: &str = "\x1b[1m";
    pub const RED: &str = "\x1b[31m";
    pub const GREEN: &str = "\x1b[32m";
    pub const YELLOW: &str = "\x1b[33m";
}

struct Config {
    year: Option<u32>,
    day: Option<u32>,
}

struct Solution {
    year: u32,
    day: u32,
    input: &'static str,
    wrapper: fn(&str) -> (String, String),
}

fn main() {
    match parse_config() {
        Ok(config) => run(config),
        Err(message) => eprintln!("{message}"),
    }
}

fn parse_config() -> Result<Config, String> {
    let args: Vec<String> = args().skip(1).collect();
    let args: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
    let mut config = Config { year: None, day: None };

    for chunk in args.chunks(2) {
        match chunk {
            ["--year", year] => {
                let year = parse_range(year, 2015, 2022)?;
                config.year = year;
            }
            ["--day", day] => {
                let day = parse_range(day, 1, 25)?;
                config.day = day;
            }
            _ => return Err("Usage: [--year YYYY] [--day DD]".to_string()),
        }
    }

    Ok(config)
}

fn parse_range(s: &str, lower: u32, upper: u32) -> Result<Option<u32>, String> {
    let x = s.parse().map_err(|_| format!("{s} should be a number"))?;

    if lower <= x && x <= upper {
        Ok(Some(x))
    } else {
        Err(format!("{} should be from {} to {}", x, lower, upper))
    }
}

fn run(config: Config) {
    let solutions: Vec<_> = all_solutions()
        .into_iter()
        .filter(|s| if let Some(year) = config.year { year == s.year } else { true })
        .filter(|s| if let Some(day) = config.day { day == s.day } else { true })
        .collect();

    let total_size = solutions.len();
    let total_time = Instant::now();

    for Solution { year, day, input, wrapper } in solutions {
        let time = Instant::now();
        let (answer1, answer2) = (wrapper)(input);
        let duration = time.elapsed().as_micros();

        println!("{BOLD}{YELLOW}{year} Day {day:02}{RESET}");
        println!("    Part 1: {answer1}");
        println!("    Part 2: {answer2}");
        println!("    Duration: {duration} μs");
    }

    let elapsed = total_time.elapsed().as_millis();
    println!("{BOLD}{RED}Solutions: {total_size}{RESET}");
    println!("{BOLD}{GREEN}Elapsed: {elapsed} ms{RESET}");
}

macro_rules! solution {
    ($year:tt, $day:tt) => {
        Solution {
            year: { stringify!($year)[4..8].parse().unwrap() },
            day: { stringify!($day)[3..5].parse().unwrap() },
            input: {
                include_str!(concat!["../input/", stringify!($year), "/", stringify!($day), ".txt"])
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

fn all_solutions() -> Vec<Solution> {
    vec![
        // 2022
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
        solution!(year2022, day11),
        solution!(year2022, day12),
        solution!(year2022, day13),
        solution!(year2022, day14),
        solution!(year2022, day15),
        solution!(year2022, day16),
        solution!(year2022, day17),
        solution!(year2022, day18),
        solution!(year2022, day19),
        solution!(year2022, day20),
        solution!(year2022, day21),
        solution!(year2022, day22),
        solution!(year2022, day23),
        solution!(year2022, day24),
        solution!(year2022, day25),
        // 2021
        solution!(year2021, day01),
        solution!(year2021, day02),
        solution!(year2021, day03),
        solution!(year2021, day04),
        solution!(year2021, day05),
        solution!(year2021, day06),
        solution!(year2021, day07),
        solution!(year2021, day08),
        solution!(year2021, day09),
        solution!(year2021, day10),
        solution!(year2021, day11),
        solution!(year2021, day12),
        solution!(year2021, day13),
        solution!(year2021, day14),
        solution!(year2021, day15),
        solution!(year2021, day16),
        solution!(year2021, day17),
        solution!(year2021, day18),
        solution!(year2021, day19),
        solution!(year2021, day20),
        solution!(year2021, day21),
        solution!(year2021, day22),
        solution!(year2021, day23),
        solution!(year2021, day24),
        solution!(year2021, day25),
        // 2020
        solution!(year2020, day01),
        solution!(year2020, day02),
        solution!(year2020, day03),
        solution!(year2020, day04),
        solution!(year2020, day05),
        solution!(year2020, day06),
        solution!(year2020, day07),
        solution!(year2020, day08),
        solution!(year2020, day09),
        solution!(year2020, day10),
        solution!(year2020, day11),
        solution!(year2020, day12),
        solution!(year2020, day13),
        solution!(year2020, day14),
        // 2015
        solution!(year2015, day01),
        solution!(year2015, day02),
        solution!(year2015, day03),
        // solution!(year2015, day04),  // Very slow
        solution!(year2015, day05),
        solution!(year2015, day06),
        solution!(year2015, day07),
        solution!(year2015, day08),
        solution!(year2015, day09),
        solution!(year2015, day10),
    ]
}
