use aoc::*;
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

fn main() {
    let total_time = Instant::now();
    let mut total_solutions = 0;

    for Solution { year, day, input, wrapper } in solutions() {
        // if year == year
        // && day == day
        {
            let time = Instant::now();
            let (answer1, answer2) = (wrapper)(input);
            let duration = time.elapsed().as_micros();
            total_solutions += 1;

            println!("{BOLD}{YELLOW}{year} Day {day:02}{RESET}");
            println!("    Part 1: {answer1}");
            println!("    Part 2: {answer2}");
            println!("    Duration: {duration} μs");
        }
    }

    let elapsed = total_time.elapsed().as_millis();
    println!("{BOLD}{RED}Solutions: {total_solutions}{RESET}");
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

fn solutions() -> Vec<Solution> {
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
        // 2015
        solution!(year2015, day01),
        solution!(year2015, day02),
        solution!(year2015, day03),
        //solution!(year2015, day04), // Very slow
    ]
}
