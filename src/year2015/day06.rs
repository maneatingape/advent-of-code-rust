//! # Probably a Fire Hazard
//!
//! This problem is easy to brute force but more challenging to solve efficiently.
//!
//! To trick to speed things up is to consider rectangles that have the same instructions instead of
//! calculating point by point. Then for each rectangle we apply the instructions only once and
//! multiply by its area.
//!
//! For example say there is only a single instruction `turn on 300,300 through 700,500`. This
//! looks a little like:
//!
//! ```none
//!     (0,0)
//!     ┌────────────┐
//!     │            │
//!     │   ┌────┐   │
//!     │   │    │   │
//!     │   └────┘   │
//!     │            │
//!     └────────────┘(1000,1000)
//! ```
//!
//! First we compute the x and y intervals:
//!
//! ```none
//!     x: [0, 300, 701, 1000]
//!     y: [0, 300, 501, 1000]
//! ```
//!
//! The intervals are *inclusive*, so the interval after the instruction starts 1 higher. Next we
//! break the grid into 3 x 3 = 9 rectangles, much fewer than the 1,000,000 individual elements.
//!
//! ```none
//!     ┌───────────┐
//!     │ A | B | C │
//!     │...┌───┐...│
//!     │ D │ E │ F │
//!     │...└───┘...│
//!     │ G | H | I │
//!     └───────────┘
//! ```
//!
//! For each of these rectangles we store a boolean if the rectangle to the left or above crosses an
//! instruction boundary.
//!
//! ```none
//!     Left             Up
//!     ┌───────────┐    ┌───────────┐
//!     │ T | F | F │    │ T | T | T │
//!     │...┌───┐...│    │...┌───┐...│
//!     │ T │ T │ T │    │ F │ T │ F │
//!     │...└───┘...│    │...└───┘...│
//!     │ T | F | F │    │ F | T | F │
//!     └───────────┘    └───────────┘
//! ```
//!
//! If there is no boundary then we can re-use the value either from the rectangle to the left or
//! above. For example `D` is the same as `A`, `B` is also the same as `A` and `I` is the same as
//! both `F` and `H`. This further reduces the different instruction sets to compute.
//!
//! For my input, there was ~100,000 rectangles but only ~20,000 different instructions regions
//! needed to be computed. This is a 50x reduction from looking at each light individually.
use crate::util::iter::*;
use crate::util::parse::*;

enum Command {
    On,
    Off,
    Toggle,
}

impl Command {
    fn from(bytes: &[u8]) -> Command {
        match bytes[6] {
            b'n' => Command::On,
            b'f' => Command::Off,
            b' ' => Command::Toggle,
            _ => unreachable!(),
        }
    }
}

struct Rectangle {
    x1: u32,
    x2: u32,
    y1: u32,
    y2: u32,
}

impl Rectangle {
    fn from([x1, y1, x2, y2]: [u32; 4]) -> Rectangle {
        Rectangle { x1, x2, y1, y2 }
    }

    fn contains(&self, x: u32, y: u32) -> bool {
        self.x1 <= x && x <= self.x2 && self.y1 <= y && y <= self.y2
    }
}

struct Instruction {
    command: Command,
    rectangle: Rectangle,
}

impl Instruction {
    fn from((bytes, points): (&[u8], [u32; 4])) -> Instruction {
        let command = Command::from(bytes);
        let rectangle = Rectangle::from(points);
        Instruction { command, rectangle }
    }
}

pub fn parse(input: &str) -> (u32, u32) {
    let first = input.lines().map(|line| line.as_bytes());
    let second = input.iter_unsigned().chunk::<4>();
    let instructions: Vec<_> = first.zip(second).map(Instruction::from).collect();

    let mut xs = vec![0, 1000];
    let mut ys = vec![0, 1000];

    for instruction in instructions.iter() {
        let Rectangle { x1, x2, y1, y2 } = instruction.rectangle;
        xs.push(x1);
        xs.push(x2 + 1);
        ys.push(y1);
        ys.push(y2 + 1);
    }

    xs.sort_unstable();
    xs.dedup();
    ys.sort_unstable();
    ys.dedup();

    let mut x_index_from = [0; 1001];
    for (i, &x) in xs.iter().enumerate() {
        x_index_from[x as usize] = i;
    }

    let mut y_index_from = [0; 1001];
    for (i, &y) in ys.iter().enumerate() {
        y_index_from[y as usize] = i;
    }

    let stride = xs.len();
    let capacity = stride * ys.len();
    let mut up = vec![false; capacity];
    let mut left = vec![false; capacity];
    let mut previous = vec![(false, 0); capacity];

    for instruction in instructions.iter() {
        let Rectangle { x1, x2, y1, y2 } = instruction.rectangle;
        let x1 = x_index_from[x1 as usize];
        let x2 = x_index_from[(x2 + 1) as usize];
        let y1 = y_index_from[y1 as usize];
        let y2 = y_index_from[(y2 + 1) as usize];

        for x in x1..(x2 + 1) {
            up[stride * y1 + x] = true;
            up[stride * y2 + x] = true;
        }
        for y in y1..(y2 + 1) {
            left[stride * y + x1] = true;
            left[stride * y + x2] = true;
        }
    }

    let mut result1 = 0;
    let mut result2 = 0;

    for j in 0..ys.len() - 1 {
        let y1 = ys[j];
        let y2 = ys[j + 1];

        for i in 0..xs.len() - 1 {
            let x1 = xs[i];
            let x2 = xs[i + 1];
            let area = (x2 - x1) * (y2 - y1);
            let index = stride * j + i;

            let current = if i > 0 && !left[index] {
                previous[index - 1]
            } else if j > 0 && !up[index] {
                previous[index - stride]
            } else {
                let mut light = false;
                let mut brightness: u8 = 0;

                for instruction in instructions.iter() {
                    if instruction.rectangle.contains(x1, y1) {
                        match instruction.command {
                            Command::On => {
                                light = true;
                                brightness += 1;
                            }
                            Command::Off => {
                                light = false;
                                brightness = brightness.saturating_sub(1);
                            }
                            Command::Toggle => {
                                light = !light;
                                brightness += 2;
                            }
                        }
                    }
                }

                (light, brightness)
            };

            previous[index] = current;
            if current.0 {
                result1 += area;
            }
            result2 += current.1 as u32 * area;
        }
    }

    (result1, result2)
}

pub fn part1(input: &(u32, u32)) -> u32 {
    input.0
}

pub fn part2(input: &(u32, u32)) -> u32 {
    input.1
}
