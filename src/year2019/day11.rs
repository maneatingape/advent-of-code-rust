//! # Space Police
//!
//! This problem is a variant of [Langton's Ant](https://en.wikipedia.org/wiki/Langton%27s_ant).
use super::intcode::*;
use crate::util::grid::*;
use crate::util::hash::*;
use crate::util::parse::*;
use crate::util::point::*;

pub fn parse(input: &str) -> Vec<i64> {
    input.iter_signed().collect()
}

pub fn part1(input: &[i64]) -> usize {
    paint(input, 0).len()
}

pub fn part2(input: &[i64]) -> String {
    let hull = paint(input, 1);

    // Filter only white panels
    let panels: Vec<_> = hull.iter().filter_map(|(&k, &v)| (v == 1).then_some(k)).collect();

    // Get maximum extents
    let (x1, x2, y1, y2) = panels.iter().fold(
        (i32::MAX, i32::MIN, i32::MAX, i32::MIN),
        |(min_x, max_x, min_y, max_y), p| {
            (min_x.min(p.x), max_x.max(p.x), min_y.min(p.y), max_y.max(p.y))
        },
    );

    // Convert panels to characters
    let width = x2 - x1 + 2; // Leave room for newline character.
    let height = y2 - y1 + 1;
    let mut image = Grid::new(width, height, b'.');

    let offset = Point::new(x1 - 1, y1);
    panels.iter().for_each(|&point| image[point - offset] = b'#');

    (0..height).for_each(|y| image[Point::new(0, y)] = b'\n');

    String::from_utf8(image.bytes).unwrap()
}

fn paint(input: &[i64], initial: i64) -> FastMap<Point, i64> {
    let mut computer = Computer::new(input);
    let mut position = ORIGIN;
    let mut direction = UP;
    let mut hull = FastMap::with_capacity(5_000);

    hull.insert(position, initial);

    loop {
        let panel = hull.entry(position).or_default();
        computer.input(*panel);

        match computer.run() {
            State::Output(color) => {
                *panel = color;
            }
            _ => break,
        }

        match computer.run() {
            State::Output(next) => {
                direction =
                    if next == 0 { direction.counter_clockwise() } else { direction.clockwise() };
                position += direction;
            }
            _ => break,
        }
    }

    hull
}
