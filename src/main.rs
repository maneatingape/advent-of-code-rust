use aoc::*;
use aoc::util::macros::*;
use std::time::Instant;

const RESET: &str = "\u{001b}[0m";
const BOLD: &str = "\u{001b}[1m";
const RED: &str = "\u{001b}[31m";
const GREEN: &str = "\u{001b}[32m";
const YELLOW: &str = "\u{001b}[33m";

fn main() {
    let now = Instant::now();
    let mut total = 0;

    for Solution { year, day, input, wrapper } in solutions() {
        if year == 2022 && day == 14 {
            let (answer1, answer2) = (wrapper)(input);
            total += 1;

            println!("{YELLOW}{BOLD}{year} Day {day:02}{RESET}");
            println!("    Part 1: {answer1}");
            println!("    Part 2: {answer2}");
        }
    }

    println!("{BOLD}{RED}Solutions: {total}{RESET}");
    println!("{BOLD}{GREEN}Elapsed: {} ms{RESET}", now.elapsed().as_millis());
}

fn solutions() -> Vec<Solution> {
    vec![
        // 2015
        solution!(year2015, day01),
        solution!(year2015, day02),
        solution!(year2015, day03),
        //solution!(year2015, day04), // Very slow

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
    ]
}
