//! # Space Police
//!
//! This problem is a variant of [Langton's Ant](https://en.wikipedia.org/wiki/Langton%27s_ant).
use super::intcode::*;
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
    let panels: Vec<_> = hull.iter().filter(|&(_, &v)| v == 1).map(|(&k, _)| k).collect();

    // Get maximum extents
    let mut x1 = i32::MAX;
    let mut x2 = i32::MIN;
    let mut y1 = i32::MAX;
    let mut y2 = i32::MIN;

    for &point in &panels {
        x1 = x1.min(point.x);
        x2 = x2.max(point.x);
        y1 = y1.min(point.y);
        y2 = y2.max(point.y);
    }

    // Convert panels to characters
    let width = (x2 - x1 + 1) as usize;
    let height = (y2 - y1 + 1) as usize;
    let offset = Point::new(x1, y1);
    let mut image = vec!['.'; width * height];

    for &point in &panels {
        let adjusted = point - offset;
        let index = (width * adjusted.y as usize) + (adjusted.x as usize);
        image[index] = '#';
    }

    // Convert to multiline string
    let mut result = image
        .chunks_exact(width)
        .map(|row| row.iter().collect())
        .collect::<Vec<String>>()
        .join("\n");
    result.insert(0, '\n');
    result
}

fn paint(input: &[i64], initial: i64) -> FastMap<Point, i64> {
    let mut computer = Computer::new(input);
    let mut position = ORIGIN;
    let mut direction = UP;
    let mut hull = FastMap::with_capacity(5_000);

    hull.insert(position, initial);

    loop {
        let panel = hull.get(&position).unwrap_or(&0);
        computer.input(*panel);

        match computer.run() {
            State::Output(color) => {
                hull.insert(position, color);
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
