use aoc::util::ansi::*;
use aoc::util::parse::*;
use aoc::*;
use std::env::args;
use std::fs::read_to_string;
use std::iter::empty;
use std::time::{Duration, Instant};

fn main() {
    // Parse command line options
    let args: Vec<_> = args().collect();
    let args: Vec<_> = args.iter().map(String::as_str).collect();
    let mut iter = args.iter().flat_map(|arg| arg.iter_unsigned::<u32>());
    let (year, day) = (iter.next(), iter.next());

    // Filter solutions then pretty print output.
    let (stars, duration) = empty()
        .chain(year2015())
        .chain(year2016())
        .chain(year2017())
        .chain(year2018())
        .chain(year2019())
        .chain(year2020())
        .chain(year2021())
        .chain(year2022())
        .chain(year2023())
        .chain(year2024())
        .filter(|solution| year.is_none_or(|y| y == solution.year))
        .filter(|solution| day.is_none_or(|d| d == solution.day))
        .fold((0, Duration::ZERO), |(stars, duration), Solution { year, day, wrapper }| {
            let path = format!("input/year{year}/day{day:02}.txt");

            if let Ok(data) = read_to_string(&path) {
                let instant = Instant::now();
                let (part1, part2) = wrapper(data);
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
    if args.contains(&"--totals") {
        println!("{BOLD}{YELLOW}⭐ {}{RESET}", stars);
        println!("{BOLD}{WHITE}🕓 {} ms{RESET}", duration.as_millis());
    }
}

struct Solution {
    year: u32,
    day: u32,
    wrapper: fn(String) -> (String, String),
}

macro_rules! run {
    ($year:tt $($day:tt),*) => {
        fn $year() -> Vec<Solution> {
            vec![$({
                let year = stringify!($year).unsigned();
                let day = stringify!($day).unsigned();
                let wrapper = |data: String| {
                    use $year::$day::*;

                    let input = parse(&data);
                    let part1 = part1(&input);
                    let part2 = part2(&input);

                    (part1.to_string(), part2.to_string())
                };

                Solution { year, day, wrapper }
            },)*]
        }
    }
}

run!(year2015
    day01, day02, day03, day04, day05, day06, day07, day08, day09, day10, day11, day12, day13,
    day14, day15, day16, day17, day18, day19, day20, day21, day22, day23, day24, day25
);

run!(year2016
    day01, day02, day03, day04, day05, day06, day07, day08, day09, day10, day11, day12, day13,
    day14, day15, day16, day17, day18, day19, day20, day21, day22, day23, day24, day25
);

run!(year2017
    day01, day02, day03, day04, day05, day06, day07, day08, day09, day10, day11, day12, day13,
    day14, day15, day16, day17, day18, day19, day20, day21, day22, day23, day24, day25
);

run!(year2018
    day01, day02, day03, day04, day05, day06, day07, day08, day09, day10, day11, day12, day13,
    day14, day15, day16, day17, day18, day19, day20, day21, day22, day23, day24, day25
);

run!(year2019
    day01, day02, day03, day04, day05, day06, day07, day08, day09, day10, day11, day12, day13,
    day14, day15, day16, day17, day18, day19, day20, day21, day22, day23, day24, day25
);

run!(year2020
    day01, day02, day03, day04, day05, day06, day07, day08, day09, day10, day11, day12, day13,
    day14, day15, day16, day17, day18, day19, day20, day21, day22, day23, day24, day25
);

run!(year2021
    day01, day02, day03, day04, day05, day06, day07, day08, day09, day10, day11, day12, day13,
    day14, day15, day16, day17, day18, day19, day20, day21, day22, day23, day24, day25
);

run!(year2022
    day01, day02, day03, day04, day05, day06, day07, day08, day09, day10, day11, day12, day13,
    day14, day15, day16, day17, day18, day19, day20, day21, day22, day23, day24, day25
);

run!(year2023
    day01, day02, day03, day04, day05, day06, day07, day08, day09, day10, day11, day12, day13,
    day14, day15, day16, day17, day18, day19, day20, day21, day22, day23, day24, day25
);

run!(year2024
    day01, day02, day03, day04, day05, day06, day07, day08, day09, day10, day11, day12, day13,
    day14, day15, day16, day17, day18, day19, day20, day21, day22, day23, day24, day25
);
