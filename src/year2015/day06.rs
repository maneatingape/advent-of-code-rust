//! # Probably a Fire Hazard
//!
//! Brute force approach that calculates each row independently, parallelizing the work across
//! multiple threads.
use crate::util::iter::*;
use crate::util::parse::*;
use crate::util::thread::*;

#[derive(Clone, Copy)]
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

#[derive(Clone, Copy)]
struct Rectangle {
    x1: usize,
    x2: usize,
    y1: usize,
    y2: usize,
}

impl Rectangle {
    // Add one to both x2 and y2 to make ranges easier.
    fn from([x1, y1, x2, y2]: [usize; 4]) -> Rectangle {
        Rectangle { x1, y1, x2: x2 + 1, y2: y2 + 1 }
    }
}

#[derive(Clone, Copy)]
pub struct Instruction {
    command: Command,
    rectangle: Rectangle,
}

impl Instruction {
    fn from((bytes, points): (&[u8], [usize; 4])) -> Instruction {
        let command = Command::from(bytes);
        let rectangle = Rectangle::from(points);
        Instruction { command, rectangle }
    }
}

pub fn parse(input: &str) -> Vec<Instruction> {
    let first = input.lines().map(str::as_bytes);
    let second = input.iter_unsigned().chunk::<4>();
    first.zip(second).map(Instruction::from).collect()
}

pub fn part1(input: &[Instruction]) -> u32 {
    let items: Vec<_> = (0..1000).collect();
    let result = spawn_parallel_iterator(&items, |iter| worker_one(input, iter));
    result.into_iter().sum()
}

pub fn part2(input: &[Instruction]) -> u32 {
    let items: Vec<_> = (0..1000).collect();
    let result = spawn_parallel_iterator(&items, |iter| worker_two(input, iter));
    result.into_iter().sum()
}

fn worker_one(input: &[Instruction], iter: ParIter<'_, usize>) -> u32 {
    iter.map(|row| {
        let mut grid = [0_u8; 1_024];

        for &Instruction { command, rectangle: Rectangle { x1, y1, x2, y2 } } in input {
            if (y1..y2).contains(row) {
                let iter = grid[x1..x2].iter_mut();
                match command {
                    Command::On => iter.for_each(|b| *b = 1),
                    Command::Off => iter.for_each(|b| *b = 0),
                    Command::Toggle => iter.for_each(|b| *b ^= 1),
                }
            }
        }

        grid.into_iter().map(|b| b as u32).sum::<u32>()
    })
    .sum()
}

fn worker_two(input: &[Instruction], iter: ParIter<'_, usize>) -> u32 {
    iter.map(|row| {
        let mut grid = [0_u8; 1_024];

        for &Instruction { command, rectangle: Rectangle { x1, y1, x2, y2 } } in input {
            if (y1..y2).contains(row) {
                let iter = grid[x1..x2].iter_mut();
                match command {
                    Command::On => iter.for_each(|b| *b += 1),
                    Command::Off => iter.for_each(|b| *b = b.saturating_sub(1)),
                    Command::Toggle => iter.for_each(|b| *b += 2),
                }
            }
        }

        grid.into_iter().map(|b| b as u32).sum::<u32>()
    })
    .sum()
}
