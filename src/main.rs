use aoc::util::macros::*;
use aoc::*;

fn main() {
    for Solution { year, day, input, wrapper } in solutions() {
        if year == 2022 && day == 13 {
            let (answer1, answer2) = (wrapper)(input);
            println!("Year {year} Day {day:02}");
            println!("    Part 1: {answer1}");
            println!("    Part 2: {answer2}");
        }
    }
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
    ]
}
