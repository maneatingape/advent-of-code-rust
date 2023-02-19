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

pub fn parse(input: &str) -> Input {
    let (prefix, suffix) = input.split_once("\n\n").unwrap();

    let points: Vec<_> = prefix
        .iter_signed()
        .chunk::<2>()
        .map(|[x, y]| Point { x, y })
        .collect();

    let folds: Vec<_> = suffix
        .lines()
        .map(|line| match line.split_once('=').unwrap() {
            ("fold along x", x) => Fold::Horizontal(from(x)),
            ("fold along y", y) => Fold::Vertical(from(y)),
            _ => unreachable!(),
        })
        .collect();

    Input { points, folds }
}

pub fn part1(input: &Input) -> usize {
    match input.folds[0] {
        Fold::Horizontal(x) => input
            .points
            .iter()
            .map(|&p| fold_horizontal(x, p))
            .collect::<FastSet<_>>()
            .len(),
        Fold::Vertical(y) => input
            .points
            .iter()
            .map(|&p| fold_vertical(y, p))
            .collect::<FastSet<_>>()
            .len(),
    }
}

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

#[inline]
fn fold_horizontal(x: i32, p: Point) -> Point {
    if p.x < x { p } else { Point { x: 2 * x - p.x, y: p.y } }
}

#[inline]
fn fold_vertical(y: i32, p: Point) -> Point {
    if p.y < y { p } else { Point { x: p.x, y: 2 * y - p.y } }
}
