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

    // Filter only white panels.
    let panels: Vec<_> = hull.iter().filter_map(|(&k, &v)| (v == 1).then_some(k)).collect();

    // Get maximum extents.
    let x1 = panels.iter().map(|p| p.x).min().unwrap();
    let x2 = panels.iter().map(|p| p.x).max().unwrap();
    let y1 = panels.iter().map(|p| p.y).min().unwrap();
    let y2 = panels.iter().map(|p| p.y).max().unwrap();

    // Convert panels to characters.
    let width = x2 - x1 + 2; // Leave room for newline character.
    let height = y2 - y1 + 1;
    let mut grid = Grid::new(width, height, '.');

    let offset = Point::new(x1 - 1, y1);
    panels.iter().for_each(|&point| grid[point - offset] = '#');
    (0..height).for_each(|y| grid[Point::new(0, y)] = '\n');

    grid.bytes.iter().collect()
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

        let State::Output(color) = computer.run() else { break };
        *panel = color;

        let State::Output(turn) = computer.run() else { break };
        direction = if turn == 0 { direction.counter_clockwise() } else { direction.clockwise() };
        position += direction;
    }

    hull
}
