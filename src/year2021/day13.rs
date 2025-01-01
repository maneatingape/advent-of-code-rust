//! # Transparent Origami
//!
//! There are 2 possible approaches to tracking the position of dots after each fold:
//! * A `HashSet` that will collapse duplicate entries
//! * An array of sufficient dimensions to track every possible coordinate.
//!
//! We will use both approaches for speed, the first in part 1 and the second in part 2.
//!
//! For part 2 we can determine the final size of the paper by taking the *last* x and y
//! coordinates from the fold instructions. It's then faster and more convenienent to process
//! each point completely and update the final location, than to step through intermediate folds.
use crate::util::hash::*;
use crate::util::iter::*;
use crate::util::parse::*;
use crate::util::point::*;

#[derive(Clone, Copy)]
pub enum Fold {
    Horizontal(i32),
    Vertical(i32),
}

pub struct Input {
    points: Vec<Point>,
    folds: Vec<Fold>,
}

/// Parse the input into collections of [`Point`] and [`Fold`] structs.
pub fn parse(input: &str) -> Input {
    let (prefix, suffix) = input.split_once("\n\n").unwrap();

    let points: Vec<_> = prefix.iter_signed().chunk::<2>().map(|[x, y]| Point::new(x, y)).collect();

    let folds: Vec<_> = suffix
        .lines()
        .map(|line| match line.split_once('=').unwrap() {
            ("fold along x", x) => Fold::Horizontal(x.signed()),
            ("fold along y", y) => Fold::Vertical(y.signed()),
            _ => unreachable!(),
        })
        .collect();

    Input { points, folds }
}

/// Fold once then count dots. The sample data folds along `y` and my input folded along `x`
/// testing both possibilities.
pub fn part1(input: &Input) -> usize {
    match input.folds[0] {
        Fold::Horizontal(x) => {
            input.points.iter().map(|&p| fold_horizontal(x, p)).collect::<FastSet<_>>().len()
        }
        Fold::Vertical(y) => {
            input.points.iter().map(|&p| fold_vertical(y, p)).collect::<FastSet<_>>().len()
        }
    }
}

/// Decode secret message.
///
/// The output is a multi-line string to allow integration testing. The final dimensions of the
/// paper are found from the last `x` and `y` fold coordinates.
pub fn part2(input: &Input) -> String {
    let mut width = 0;
    let mut height = 0;

    for &fold in &input.folds {
        match fold {
            Fold::Horizontal(x) => width = x,
            Fold::Vertical(y) => height = y,
        }
    }

    let mut grid = vec![false; (width * height) as usize];

    for point in &input.points {
        let mut point = *point;

        for &fold in &input.folds {
            point = match fold {
                Fold::Horizontal(x) => fold_horizontal(x, point),
                Fold::Vertical(y) => fold_vertical(y, point),
            }
        }

        grid[(point.y * width + point.x) as usize] = true;
    }

    let mut code = String::new();
    for y in 0..height {
        code.push('\n');
        for x in 0..width {
            let c = if grid[(y * width + x) as usize] { '#' } else { '.' };
            code.push(c);
        }
    }
    code
}

/// Fold point at `x` coordinate, doing nothing if the point is to the left of the fold line.
#[inline]
fn fold_horizontal(x: i32, p: Point) -> Point {
    if p.x < x { p } else { Point::new(2 * x - p.x, p.y) }
}

/// Fold point at `y` coordinate, doing nothing if the point is above the fold line.
#[inline]
fn fold_vertical(y: i32, p: Point) -> Point {
    if p.y < y { p } else { Point::new(p.x, 2 * y - p.y) }
}
