use aoc::util::scaffold::*;

fn main() {
    solutions().iter().filter(|s| s.year == 2022).for_each(run);
}

fn run(solution: &Solution) {
    let Solution { year, day, raw, wrapper } = solution;
    let (answer1, answer2) = (wrapper)(raw);
    println!("Year {year} Day {day:02}");
    println!("    Part 1: {answer1}");
    println!("    Part 2: {answer2}");
}
