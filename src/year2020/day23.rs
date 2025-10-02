//! # Crab Cups
//!
//! The cups form a [singly linked list](https://en.wikipedia.org/wiki/Linked_list).
//!
//! For performance instead of using pointers, we store the cups in a `vec` where an element
//! at index `i` stores the index of the next cup. For example `cup[1]` points to the first cup
//! after cup one and `cup[cup[1]]` points to second cup after cup one.
//!
//! Notes:
//! * One million is approximately 2²⁰ so the closest integer size that fits is `u32`.
//!   Using `u32` instead of `usize` increases speed due to better cache locality.
//! * Cups use one based indexing so the vec is one longer than the number of cups and the zeroth
//!   index is unused.
use crate::util::parse::*;

pub fn parse(input: &str) -> Vec<u32> {
    input.trim().bytes().map(|b| b.to_decimal() as u32).collect()
}

pub fn part1(input: &[u32]) -> u32 {
    let start = input[0] as usize;
    let mut current = start;
    let mut cups = vec![0; 10];

    // Link the 9 input cups, wrapping around to the start.
    for &next in &input[1..] {
        cups[current] = next;
        current = next as usize;
    }
    cups[current] = start as u32;

    play(&mut cups, start, 100);

    (0..8).fold((0, 1), |(acc, i), _| (10 * acc + cups[i], cups[i] as usize)).0
}

pub fn part2(input: &[u32]) -> usize {
    let start = input[0] as usize;
    let mut current = start;
    let mut cups: Vec<_> = (1..1_000_002).collect();

    // Link the 9 input cups, continuing to the extra elements.
    for &next in &input[1..] {
        cups[current] = next;
        current = next as usize;
    }
    cups[current] = 10;

    // Wrap around to the start
    cups[1_000_000] = start as u32;

    play(&mut cups, start, 10_000_000);

    let first = cups[1] as usize;
    let second = cups[first] as usize;
    first * second
}

fn play(cups: &mut [u32], mut current: usize, rounds: usize) {
    for _ in 0..rounds {
        // Pickup three cups (a, b, c)
        let a = cups[current] as usize;
        let b = cups[a] as usize;
        let c = cups[b] as usize;

        // Calculate destination
        let mut dest = if current > 1 { current - 1 } else { cups.len() - 1 };
        while dest == a || dest == b || dest == c {
            dest = if dest > 1 { dest - 1 } else { cups.len() - 1 };
        }

        // Link current cup to the fourth cup after the three cups that have just been picked up.
        cups[current] = cups[c];
        current = cups[c] as usize;

        // Insert the three picked up cups into their new location
        cups[c] = cups[dest];
        cups[dest] = a as u32;
    }
}
