use aoc::util::ansi::*;
use aoc::util::parse::*;
use aoc::*;
use std::env::args;
use std::time::Instant;

fn main() {
    // Parse command line options
    let (year, day) = match args().nth(1) {
        Some(arg) => {
            let str = arg.as_str();
            let mut iter = str.iter_unsigned();
            (iter.next(), iter.next())
        }
        None => (None, None),
    };

    // Filter solutions
    let solutions: Vec<_> = all_solutions()
        .into_iter()
        .filter(|solution| year == Some(solution.year) || year.is_none())
        .filter(|solution| day == Some(solution.day) || day.is_none())
        .collect();

    // Pretty print output and timing for each solution
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

    // Print totals
    let elapsed = total_time.elapsed().as_millis();
    println!("{BOLD}{RED}Solutions: {total_size}{RESET}");
    println!("{BOLD}{GREEN}Elapsed: {elapsed} ms{RESET}");
}

struct Solution {
    year: u32,
    day: u32,
    input: &'static str,
    wrapper: fn(&str) -> (String, String),
}

macro_rules! solution {
    ($year:tt, $day:tt) => {
        Solution {
            year: (&stringify!($year)).unsigned(),
            day: (&stringify!($day)).unsigned(),
            input: include_str!(concat![
                "../input/",
                stringify!($year),
                "/",
                stringify!($day),
                ".txt"
            ]),
            wrapper: |raw: &str| {
                use $year::$day::*;
                let input = parse(raw);
                let part1 = part1(&input).to_string();
                let part2 = part2(&input).to_string();
                (part1, part2)
            },
        }
    };
}

fn all_solutions() -> Vec<Solution> {
    vec![
        // 2015
        solution!(year2015, day01),
        solution!(year2015, day02),
        solution!(year2015, day03),
        solution!(year2015, day04),
        solution!(year2015, day05),
        solution!(year2015, day06),
        solution!(year2015, day07),
        solution!(year2015, day08),
        solution!(year2015, day09),
        solution!(year2015, day10),
        solution!(year2015, day11),
        solution!(year2015, day12),
        // 2019
        solution!(year2019, day01),
        solution!(year2019, day02),
        solution!(year2019, day03),
        solution!(year2019, day04),
        solution!(year2019, day05),
        solution!(year2019, day06),
        solution!(year2019, day07),
        solution!(year2019, day08),
        solution!(year2019, day09),
        solution!(year2019, day10),
        solution!(year2019, day11),
        solution!(year2019, day12),
        solution!(year2019, day13),
        solution!(year2019, day14),
        solution!(year2019, day15),
        solution!(year2019, day16),
        solution!(year2019, day17),
        solution!(year2019, day18),
        solution!(year2019, day19),
        solution!(year2019, day20),
        solution!(year2019, day21),
        solution!(year2019, day22),
        solution!(year2019, day23),
        solution!(year2019, day24),
        solution!(year2019, day25),
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
        solution!(year2020, day15),
        solution!(year2020, day16),
        solution!(year2020, day17),
        solution!(year2020, day18),
        solution!(year2020, day19),
        solution!(year2020, day20),
        solution!(year2020, day21),
        solution!(year2020, day22),
        solution!(year2020, day23),
        solution!(year2020, day24),
        solution!(year2020, day25),
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
    ]
}
