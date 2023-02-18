use crate::util::chunk::*;
use crate::util::hash::*;
use crate::util::parse::*;
use crate::util::point::*;

type Input = (Point, u32);

pub fn parse(input: &str) -> Vec<Input> {
    input
        .split_ascii_whitespace()
        .chunk::<2>()
        .map(|[d, n]| {
            let point = Point::from_string(d);
            let amount = from(n);
            (point, amount)
        })
        .collect()
}

pub fn part1(input: &[Input]) -> usize {
    simulate(input, 2)
}

pub fn part2(input: &[Input]) -> usize {
    simulate(input, 10)
}

fn simulate(input: &[Input], size: usize) -> usize {
    let mut rope: Vec<Point> = vec![ORIGIN; size];
    let mut tail: FastSet<Point> = FastSetBuilder::with_capacity(5_000);

    for (step, amount) in input {
        for _ in 0..*amount {
            rope[0] += *step;
            for i in 1..size {
                if apart(rope[i - 1], rope[i]) {
                    let next = delta(rope[i - 1], rope[i]);
                    rope[i] += next;
                }
            }
            tail.insert(rope[size - 1]);
        }
    }

    tail.len()
}

fn apart(a: Point, b: Point) -> bool {
    (a.x - b.x).abs() > 1 || (a.y - b.y).abs() > 1
}

fn delta(a: Point, b: Point) -> Point {
    Point {
        x: (a.x - b.x).signum(),
        y: (a.y - b.y).signum(),
    }
}
