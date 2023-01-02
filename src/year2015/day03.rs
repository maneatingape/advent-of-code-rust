use std::collections::HashSet;
use crate::util::point::*;

pub fn parse(input: &str) -> Vec<Point> {
    fn helper(c: char) -> Point {
        match c {
            '^' => UP,
            'v' => DOWN,
            '<' => LEFT,
            '>' => RIGHT,
            _ => panic!("Unexpected input")
        }
    }
    input.trim().chars().map(helper).collect()
}

pub fn part1(input: &[Point]) -> usize {
    deliver(input, |_| true)
}

pub fn part2(input: &[Point]) -> usize {
    deliver(input, |i| i % 2 == 0)
}

fn deliver(input: &[Point], predicate: fn(usize) -> bool) -> usize {
    let origin = Point(0, 0);
    let mut santa = origin;
    let mut robot = origin;
    let mut set = HashSet::from([origin]);

    for (index, point) in input.iter().enumerate() {
        if predicate(index) {
            santa += *point;
            set.insert(santa);
        } else {
            robot += *point;
            set.insert(robot);
        }
    }

    set.len()    
}
