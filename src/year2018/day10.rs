//! # The Stars Align
use crate::util::iter::*;
use crate::util::parse::*;
use crate::util::point::*;

type Input = (String, i32);

pub fn parse(input: &str) -> Input {
    let (mut points, velocity): (Vec<_>, Vec<_>) = input
        .iter_signed::<i32>()
        .chunk::<4>()
        .map(|[x, y, dx, dy]| (Point::new(x, y), Point::new(dx, dy)))
        .unzip();

    // Find two points traveling in opposite directions.
    let up = velocity.iter().position(|v| v.y < 0).unwrap();
    let down = velocity.iter().position(|v| v.y > 0).unwrap();

    // Use relative velocity and position to find the time when they are 10 units apart.
    let p = (points[up].y - points[down].y).abs();
    let v = (velocity[up].y - velocity[down].y).abs();
    let mut time = (p - 10) / v;

    // Fast forward time.
    tick(&mut points, &velocity, time);

    // Message is 62 wide and 10 high (8 characters each 6 wide with 2 space gap between).
    // Shrink one second at a time until area of points is exactly 620.
    let mut area = size(&points);

    while area > 620 {
        tick(&mut points, &velocity, 1);
        area = size(&points);
        time += 1;
    }

    // Move top left corner of points to origin.
    adjust(&mut points);

    // Convert points to human readable string.
    let mut grid = ['.'; 620];
    points.iter().for_each(|p| grid[(62 * p.y + p.x) as usize] = '#');

    let mut message = grid
        .chunks_exact(62)
        .map(|chunk| chunk.iter().collect())
        .collect::<Vec<String>>()
        .join("\n");
    message.insert(0, '\n');

    (message, time)
}

pub fn part1(input: &Input) -> &str {
    &input.0
}

pub fn part2(input: &Input) -> i32 {
    input.1
}

fn bounding_box(points: &[Point]) -> (i32, i32, i32, i32) {
    points.iter().fold(
        (i32::MAX, i32::MIN, i32::MAX, i32::MIN),
        |(min_x, max_x, min_y, max_y), p| {
            (min_x.min(p.x), max_x.max(p.x), min_y.min(p.y), max_y.max(p.y))
        },
    )
}

fn size(points: &[Point]) -> i32 {
    let (min_x, max_x, min_y, max_y) = bounding_box(points);
    (max_x - min_x + 1) * (max_y - min_y + 1)
}

fn adjust(points: &mut [Point]) {
    let (min_x, _, min_y, _) = bounding_box(points);
    let top_left = Point::new(min_x, min_y);
    points.iter_mut().for_each(|p| *p -= top_left);
}

fn tick(points: &mut [Point], velocity: &[Point], time: i32) {
    points.iter_mut().zip(velocity.iter()).for_each(|(p, v)| {
        *p += *v * time;
    });
}
