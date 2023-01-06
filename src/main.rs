use aoc::*;
use aoc::util::macros::*;

fn main() {
    for Solution { year, day, input, wrapper } in solutions() {
        let (answer1, answer2) = (wrapper)(input);
        println!("Year {year} Day {day:02}");
        println!("    Part 1: {answer1}");
        println!("    Part 2: {answer2}");
    }
}

fn solutions() -> Vec<Solution> {
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
        solution!(year2022, day11),
    ]
}
