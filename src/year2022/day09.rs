use crate::util::collection::*;
use crate::util::parse::*;
use crate::util::point::*;
use std::collections::HashSet;

pub fn parse(input: &str) -> Vec<Point> {
    let mut iter = input.split_ascii_whitespace();
    let mut points: Vec<Point> = vec![];

    while let (Some(d), Some(n)) = (iter.next(), iter.next()) {
        let point = match d {
            "U" => UP,
            "D" => DOWN,
            "L" => LEFT,
            "R" => RIGHT,
            _ => unreachable!(),
        };
        let amount = to::<u32>(n);
        for _ in 0..amount {
            points.push(point);
        }
    }

    points
}

pub fn part1(input: &[Point]) -> usize {
    simulate(input, 2)
}

pub fn part2(input: &[Point]) -> usize {
    simulate(input, 10)
}

fn simulate(input: &[Point], size: usize) -> usize {
    let mut rope: Vec<Point> = Vec::fill(size, ORIGIN);
    let mut tail: HashSet<Point> = HashSet::new();

    for step in input {
        rope[0] += *step;
        for i in 1..size {
            if apart(rope[i - 1], rope[i]) {
                let next = delta(rope[i - 1], rope[i]);
                rope[i] += next;
            }
        }
        tail.insert(rope[size - 1]);
    }

    tail.len()
}

fn apart(a: Point, b: Point) -> bool {
    (a.0 - b.0).abs() > 1 || (a.1 - b.1).abs() > 1
}

fn delta(a: Point, b: Point) -> Point {
    Point((a.0 - b.0).signum(), (a.1 - b.1).signum())
}
