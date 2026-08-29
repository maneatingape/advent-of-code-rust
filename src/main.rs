use std::env::args;
use std::fs::read_to_string;
use std::time::{Duration, Instant};

use aoc::util::ansi::*;
use aoc::util::parse::*;

struct Solution {
    year: u32,
    day: u32,
    wrapper: fn(&str) -> (String, String),
}

macro_rules! run {
    ($($year:ident $description:literal $($day:ident)*),*) => {
        [$($(
            Solution {
                year: stringify!($year).unsigned(),
                day: stringify!($day).unsigned(),
                wrapper: |data: &str| {
                    use aoc::$year::$day::*;

                    let input = parse(data);
                    let part1 = part1(&input).to_string();
                    let part2 = part2(&input).to_string();

                    (part1, part2)
                }
            }
        ,)*)*]
    }
}

fn main() {
    // Parse command line options.
    let mut iter = args().skip(1).flat_map(|arg| arg.iter_unsigned().collect::<Vec<u32>>());
    let (year, day) = (iter.next(), iter.next());

    // Build list of all solutions.
    let solutions = aoc::solutions!(run);

    // Run selected solutions.
    let (stars, duration) = solutions
        .into_iter()
        .filter(|s| year.is_none_or(|y| y == s.year))
        .filter(|s| day.is_none_or(|d| d == s.day))
        .fold((0, Duration::ZERO), |(stars, duration), solution| {
            let Solution { year, day, wrapper } = solution;
            let path = format!("input/year{year}/day{day:02}.txt");

            if let Ok(data) = read_to_string(&path) {
                let instant = Instant::now();
                let (part1, part2) = wrapper(&data);
                let elapsed = instant.elapsed();

                println!("{BOLD}{YELLOW}{year} Day {day}{RESET}");
                println!("    Part 1: {part1}");
                println!("    Part 2: {part2}");

                (stars + 2, duration + elapsed)
            } else {
                eprintln!("{BOLD}{RED}{year} Day {day}{RESET}");
                eprintln!("    Missing input!");
                eprintln!("    Place input file in {BOLD}{WHITE}{path}{RESET}");

                (stars, duration)
            }
        });

    // Optionally print totals.
    if args().any(|arg| arg == "--totals") {
        println!("{BOLD}{YELLOW}⭐ {stars}{RESET}");
        println!("{BOLD}{WHITE}🕓 {} ms{RESET}", duration.as_millis());
    }
}
