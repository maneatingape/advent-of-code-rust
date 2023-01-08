use crate::util::parse::to_vec;
use crate::util::point::*;
use std::collections::HashSet;

pub fn parse(input: &str) -> HashSet<Point> {
    let mut rock = HashSet::new();

    for line in input.lines() {
        to_vec::<i32>(line)
            .windows(4)
            .step_by(2)
            .for_each(|window| {
                if let [x1, y1, x2, y2] = window {
                    helper(&mut rock, *x1, *y1, *x2, *y2);
                }
            });
    }

    fn helper(rock: &mut HashSet<Point>, x1: i32, y1: i32, x2: i32, y2: i32) {
        for x in x1.min(x2)..=x1.max(x2) {
            for y in y1.min(y2)..=y1.max(y2) {
                rock.insert(Point(x, y));
            }
        }
    }

    rock
}

pub fn part1(input: &HashSet<Point>) -> usize {
    simulate(input, false)
}

pub fn part2(input: &HashSet<Point>) -> usize {
    simulate(input, true)
}

pub fn simulate(rock: &HashSet<Point>, floor: bool) -> usize {
    fn helper(rock: &HashSet<Point>, sand: &mut HashSet<Point>, max_y: i32, unit: Point, floor: bool) -> bool {
        if rock.contains(&unit) || sand.contains(&unit) {
            true
        }
        else if unit.1 == max_y + 2 {
            floor
        }
        else if helper(rock, sand, max_y, Point(unit.0, unit.1 + 1), floor)
            && helper(rock, sand, max_y, Point(unit.0 - 1, unit.1 + 1), floor)
            && helper(rock, sand, max_y, Point(unit.0 + 1, unit.1 + 1), floor)
        {
            sand.insert(unit);
            true
        }
        else {
            false
        }
    }

    let max_y = rock.iter().map(|p| p.1).max().unwrap();
    let mut sand: HashSet<Point> = HashSet::new();
    helper(rock, &mut sand, max_y, Point(500, 0), floor);
    sand.len()
}
