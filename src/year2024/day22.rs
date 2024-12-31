//! # Monkey Market
//!
//! Solves both parts simultaneously, parallelizing the work over multiple threads since
//! each secret number is independent. The process of generating the next secret number is a
//! [linear feedback shift register](https://en.wikipedia.org/wiki/Linear-feedback_shift_register).
//! with a cycle of 2²⁴.
//!
//! Interestingly this means that with some clever math it's possible to generate the `n`th number
//! from any starting secret number with only 24 calculations. Unfortunately this doesn't help for
//! part two since we need to check every possible price change. However to speed things up we can
//! make several optimizations:
//!
//! * First the sequence of 4 prices is converted from -9..9 to a base 19 index of 0..19.
//! * Whether a monkey has seen a sequence before and the total bananas for each sequence are
//!   stored in an array. This is much faster than a `HashMap`. Using base 19 gives much better
//!   cache locality needing only 130321 elements, for example compared to shifting each new cost
//!   by 5 bits and storing in an array of 2²⁰ = 1048675 elements. Multiplication on modern
//!   processors is cheap (and several instructions can issue at once) but random memory access
//!   is expensive.
use crate::util::parse::*;
use crate::util::thread::*;
use std::sync::Mutex;

type Input = (usize, u16);

struct Exclusive {
    part_one: usize,
    part_two: Vec<u16>,
}

pub fn parse(input: &str) -> Input {
    let numbers: Vec<_> = input.iter_unsigned().collect();
    let mutex = Mutex::new(Exclusive { part_one: 0, part_two: vec![0; 130321] });

    // Use as many cores as possible to parallelize the remaining search.
    spawn_parallel_iterator(&numbers, |iter| worker(&mutex, iter));

    let Exclusive { part_one, part_two } = mutex.into_inner().unwrap();
    (part_one, *part_two.iter().max().unwrap())
}

pub fn part1(input: &Input) -> usize {
    input.0
}

pub fn part2(input: &Input) -> u16 {
    input.1
}

fn worker(mutex: &Mutex<Exclusive>, iter: ParIter<'_, usize>) {
    let mut part_one = 0;
    let mut part_two = vec![0; 130321];
    let mut seen = vec![u16::MAX; 130321];

    for (id, number) in iter.enumerate() {
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

            // Compute index into the array.
            (a, b, c, d) = (b, c, d, 9 + price - previous);
            let index = 6859 * a + 361 * b + 19 * c + d;

            // Only sell the first time we see a sequence.
            // By storing the id in the array we don't need to zero every iteration which is faster.
            if seen[index] != id {
                part_two[index] += price as u16;
                seen[index] = id;
            }

            previous = price;
        }

        part_one += number;
    }

    // Merge into global results.
    let mut exclusive = mutex.lock().unwrap();
    exclusive.part_one += part_one;
    exclusive.part_two.iter_mut().zip(part_two).for_each(|(a, b)| *a += b);
}

/// Compute next secret number using a
/// [Xorshift LFSR](https://en.wikipedia.org/wiki/Linear-feedback_shift_register#Xorshift_LFSRs).
fn hash(mut n: usize) -> usize {
    n = (n ^ (n << 6)) & 0xffffff;
    n = (n ^ (n >> 5)) & 0xffffff;
    (n ^ (n << 11)) & 0xffffff
}

/// Convert -9..9 to 0..18.
fn to_index(previous: usize, current: usize) -> usize {
    9 + current % 10 - previous % 10
}
