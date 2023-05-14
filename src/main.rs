use aoc::*;
use std::env;
use std::ops::Range;
use std::time::Instant;

// ANSI escape codes
const RESET: &str = "\x1b[0m";
const BOLD: &str = "\x1b[1m";
const RED: &str = "\x1b[31m";
const GREEN: &str = "\x1b[32m";
const YELLOW: &str = "\x1b[33m";

struct Solution {
    year: u32,
    day: u32,
    input: &'static str,
    wrapper: fn(&str) -> (String, String),
}

struct Config {
    years: Range<u32>,
    days: Range<u32>,
}

fn main() {
    match parse_config() {
        Ok(config) => run(config),
        Err(message) => eprintln!("{message}"),
    }
}

fn parse_config() -> Result<Config, String> {
    let args: Vec<String> = env::args().collect();
    let args: Vec<&str> = args.iter().map(|s| s.as_str()).collect();

    let all_years = 2015..2023;
    let all_days = 1..26;

    match args[..] {
        [_] => Ok(Config { years: all_years, days: all_days }),
        [_, "--year", year] => {
            let year = parse_number(year)?;
            let years = parse_range(year, all_years)?;
            Ok(Config { years, days: all_days })
        },
        [_, "--year", year, "--day", day] => {
            let year = parse_number(year)?;
            let years = parse_range(year, all_years)?;
            let day = parse_number(day)?;
            let days = parse_range(day, all_days)?;
            Ok(Config { years, days })

        },
        _ => Err("Usage: [--year YYYY] [--day DD]".to_string())
    }
}

fn parse_range(x: u32, valid_range: Range<u32>) -> Result<Range<u32>, String> {
    if valid_range.contains(&x) {
        Ok(x..x + 1)
    } else {
        Err(format!("{} should be from {} to {}", x, valid_range.start, valid_range.end - 1))
    }
}

fn parse_number(s: &str) -> Result<u32, String> {
    s.parse().map_err(|_| format!("{s} should be a number"))
}

fn run(config: Config) {
    let solutions = all_solutions();
    let filtered: Vec<_> = solutions
        .into_iter()
        .filter(|s| config.years.contains(&s.year))
        .filter(|s| config.days.contains(&s.day))
        .collect();

    let total_size = filtered.len();
    let total_time = Instant::now();

    for Solution { year, day, input, wrapper } in filtered {
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
    ]
}
