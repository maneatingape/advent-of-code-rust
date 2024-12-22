//! # Monkey Market
use crate::util::parse::*;
use crate::util::thread::*;
use std::sync::Mutex;

type Input = (usize, u16);

struct Exclusive {
    part_one: usize,
    part_two: Vec<u16>,
}

pub fn parse(input: &str) -> Input {
    let numbers = input.iter_unsigned().collect();
    let mutex = Mutex::new(Exclusive { part_one: 0, part_two: vec![0; 130321] });

    spawn_batches(numbers, |batch| worker(&mutex, &batch));

    let Exclusive { part_one, part_two } = mutex.into_inner().unwrap();
    (part_one, *part_two.iter().max().unwrap())
}

pub fn part1(input: &Input) -> usize {
    input.0
}

pub fn part2(input: &Input) -> u16 {
    input.1
}

fn worker(mutex: &Mutex<Exclusive>, batch: &[usize]) {
    let mut part_one = 0;
    let mut part_two = vec![0; 130321];
    let mut seen = vec![u16::MAX; 130321];

    for (id, number) in batch.iter().enumerate() {
        let id = id as u16;

        let zeroth = *number;
        let first = hash(zeroth);
        let second = hash(first);
        let third = hash(second);

        let mut a;
        let mut b = to_index(zeroth, first);
        let mut c = to_index(first, second);
        let mut d = to_index(second, third);

        let mut number = third;
        let mut previous = third % 10;

        for _ in 3..2000 {
            number = hash(number);
            let price = number % 10;

            (a, b, c, d) = (b, c, d, 9 + price - previous);
            let index = 6859 * a + 361 * b + 19 * c + d;

            if seen[index] != id {
                part_two[index] += price as u16;
                seen[index] = id;
            }

            previous = price;
        }

        part_one += number;
    }

    let mut exclusive = mutex.lock().unwrap();
    exclusive.part_one += part_one;
    exclusive.part_two.iter_mut().zip(part_two).for_each(|(a, b)| *a += b);
}

fn hash(mut n: usize) -> usize {
    n = (n ^ (n << 6)) & 0xffffff;
    n = (n ^ (n >> 5)) & 0xffffff;
    (n ^ (n << 11)) & 0xffffff
}

/// Convert -9..9 to 0..18.
fn to_index(previous: usize, current: usize) -> usize {
    9 + current % 10 - previous % 10
}
