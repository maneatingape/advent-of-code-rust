//! # Set and Forget
//!
//! The key insight is that this is not a path finding problem but a *compression*
//! problem. We need to reduce the robot's path into repetitions of three patterns.
//! This is essentially a very simple version of the well known
//! [LZW](https://en.wikipedia.org/wiki/Lempel%E2%80%93Ziv%E2%80%93Welch)
//! algorithm used by the `GIF` and `ZIP` file formats.
//!
//! First we find the complete path with a simple heuristic:
//! * Rotate left or right to face the current path segment (a horizontal or vertical line).
//! * Go forwards until we hit the end of the current path segment.
//! * If it's a dead end then finish.
//!
//! Then we look for three patterns that can be repeated in any order to form the whole path.
//! Without loss of any generality the first pattern anchored at the start is always `A`,
//! the next `B` and the last `C`.
use super::intcode::*;
use crate::util::hash::*;
use crate::util::parse::*;
use crate::util::point::*;
use std::fmt::Write as _;
use std::ops::ControlFlow;

pub struct Input {
    code: Vec<i64>,
    scaffold: FastSet<Point>,
    position: Point,
    direction: Point,
}

struct Movement<'a> {
    routine: String,
    functions: [Option<&'a str>; 3],
}

/// The camera output points from left to right, top to bottom.
pub fn parse(input: &str) -> Input {
    let code: Vec<_> = input.iter_signed().collect();
    let mut computer = Computer::new(&code);

    let mut x = 0;
    let mut y = 0;
    let mut scaffold = FastSet::new();
    let mut position = ORIGIN;
    let mut direction = ORIGIN;

    while let State::Output(next) = computer.run() {
        match next {
            // '\n'
            10 => {
                y += 1;
            }
            // '#'
            35 => {
                scaffold.insert(Point::new(x, y));
            }
            // '<'
            60 => {
                position = Point::new(x, y);
                direction = LEFT;
            }
            // '>'
            62 => {
                position = Point::new(x, y);
                direction = RIGHT;
            }
            // '^'
            94 => {
                position = Point::new(x, y);
                direction = UP;
            }
            // 'v'
            118 => {
                position = Point::new(x, y);
                direction = DOWN;
            }
            // '.'
            _ => (),
        }
        x = if next == 10 { 0 } else { x + 1 };
    }

    Input { code, scaffold, position, direction }
}

pub fn part1(input: &Input) -> i32 {
    let Input { scaffold, .. } = input;
    let mut result = 0;

    for &point in scaffold {
        if ORTHOGONAL.iter().all(|&delta| scaffold.contains(&(point + delta))) {
            result += point.x * point.y;
        }
    }

    result
}

pub fn part2(input: &Input) -> i64 {
    let path = build_path(input);
    let mut movement = Movement { routine: String::new(), functions: [None; 3] };

    compress(&path, &mut movement);

    // Convert trailing comma ',' into a trailing newline '\n'
    let mut rules = String::new();
    let mut newline_ending = |s: &str| {
        rules.push_str(s);
        rules.pop();
        rules.push('\n');
    };

    newline_ending(&movement.routine);
    movement.functions.into_iter().flatten().for_each(newline_ending);

    let mut modified = input.code.clone();
    modified[0] = 2;

    let mut computer = Computer::new(&modified);
    computer.input_ascii(&rules);

    visit(computer)
}

/// Use a simple heuristic to build a path that visits every part of the scaffold at least once.
/// This string will be too long to use directly in the robot's movement functions, so we'll
/// need to compress it first.
fn build_path(input: &Input) -> String {
    let scaffold = &input.scaffold;
    let mut position = input.position;
    let mut direction = input.direction;
    let mut path = String::new();

    loop {
        let left = direction.counter_clockwise();
        let right = direction.clockwise();

        if scaffold.contains(&(position + left)) {
            direction = left;
        } else if scaffold.contains(&(position + right)) {
            direction = right;
        } else {
            break path;
        }

        let mut next = position + direction;
        let mut magnitude = 0;

        while scaffold.contains(&next) {
            position = next;
            next += direction;
            magnitude += 1;
        }

        let direction = if direction == left { 'L' } else { 'R' };
        let _ = write!(path, "{direction},{magnitude},");
    }
}

/// Find three patterns that can be repeated in any order to build the whole path.
///
/// Uses a greedy backtracking algorithm that attempts to match as much of the remaining string
/// as possible with known patterns, before trying combinations of a new pattern (up to the maximum
/// movement function length of 20 characters).
fn compress<'a>(path: &'a str, movement: &mut Movement<'a>) -> ControlFlow<()> {
    // Nothing left to match, we've finished successfully.
    if path.is_empty() {
        return ControlFlow::Break(());
    }
    // Safety check just in case very short sequences can match the entire input.
    if movement.routine.len() > 21 {
        return ControlFlow::Continue(());
    }

    for (i, &name) in ['A', 'B', 'C'].iter().enumerate() {
        movement.routine.push(name);
        movement.routine.push(',');

        if let Some(needle) = movement.functions[i] {
            // Try known patterns first
            if let Some(remaining) = path.strip_prefix(needle) {
                compress(remaining, movement)?;
            }
        } else {
            // Then combinations up to length 20 characters
            for (needle, remaining) in segments(path) {
                movement.functions[i] = Some(needle);
                compress(remaining, movement)?;
                movement.functions[i] = None;
            }
        }

        movement.routine.pop();
        movement.routine.pop();
    }

    ControlFlow::Continue(())
}

/// Fun with iterators.
fn segments(path: &str) -> impl Iterator<Item = (&str, &str)> {
    path.bytes()
        .enumerate()
        // Index of every comma ',' in the string
        .filter_map(|(i, b)| (b == b',').then_some(i))
        // Maximum length for movement function is 20 characters
        .take_while(|&i| i < 21)
        // Include trailing comma in "needle" to make matching easier
        .map(|i| path.split_at(i + 1))
        // Movement is always pairs of (rotation, magnitude) so return every second comma
        .skip(1)
        .step_by(2)
}

#[cfg(not(feature = "frivolity"))]
fn visit(mut computer: Computer) -> i64 {
    // Disable continous video feed
    computer.input_ascii("n\n");

    let mut result = 0;
    while let State::Output(next) = computer.run() {
        result = next;
    }
    result
}

/// Non essential but fun. Animates the robot traversing the scaffold.
#[cfg(feature = "frivolity")]
fn visit(mut computer: Computer) -> i64 {
    use crate::util::ansi::*;
    use std::thread::sleep;
    use std::time::Duration;

    let mut result = 0;
    let mut previous = ' ';
    let mut buffer = String::new();

    // Enable continous video feed
    computer.input_ascii("y\n");

    while let State::Output(next) = computer.run() {
        result = next;
        let ascii = (next as u8) as char;

        // Highlight the robot's position
        match ascii {
            '^' | 'v' | '<' | '>' => {
                let _ = write!(&mut buffer, "{BOLD}{YELLOW}{ascii}{RESET}");
            }
            _ => buffer.push(ascii),
        }

        // Each frame is separated by a blank line
        if ascii == '\n' && previous == '\n' {
            print!("{HOME}{CLEAR}{buffer}");
            sleep(Duration::from_millis(25));
            buffer.clear();
        }

        previous = ascii;
    }

    result
}
