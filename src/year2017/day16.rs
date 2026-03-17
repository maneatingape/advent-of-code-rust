//! # Permutation Promenade
//!
//! The key insight is that a complete dance can be represented by just two transformations.
//! The spin and exchange moves compose into a single transformation and the partner swaps compose
//! into a second independent transformation.
//!
//! Each transformation can then be applied to itself to double the effect. For example, a single
//! complete dance turns into two dances, then doubles to four dances and so on.
//!
//! This allows us to compute part two with a similar approach to
//! [exponentiation by squaring](https://en.wikipedia.org/wiki/Exponentiation_by_squaring).
use crate::util::parse::*;
use std::array::from_fn;

#[derive(Copy, Clone)]
pub struct Dance {
    /// The letter in each position from left to right
    /// with `a` represented by 0, `b` by 1 and so on.
    position: [usize; 16],
    /// A map of initial letter to final letter taking into account all partner swaps.
    /// `a` is at index 0, `b` at index 1. For convenience letters are represented by 0..15.
    exchange: [usize; 16],
}

impl Dance {
    /// Creates a new Dance that represents the identity transformation.
    fn new() -> Dance {
        Dance { position: from_fn(|i| i), exchange: from_fn(|i| i) }
    }

    /// Converts a Dance into a string representation.
    fn apply(self) -> String {
        self.position.iter().map(|&i| to_char(self.exchange[i])).collect()
    }

    /// Combines two Dances into a new Dance.
    fn compose(self, other: Dance) -> Dance {
        let position = self.position.map(|i| other.position[i]);
        let exchange = self.exchange.map(|i| other.exchange[i]);
        Dance { position, exchange }
    }
}

/// Reduces all 10,000 individual dance moves into just two independent transformations.
pub fn parse(input: &str) -> Dance {
    // Tokenize the input into two parallel iterators.
    let mut letters = input.bytes().filter(u8::is_ascii_lowercase);
    let mut numbers = input.iter_unsigned::<usize>();

    let mut offset = 0;
    let mut lookup: [usize; 16] = from_fn(|i| i);
    let Dance { mut position, mut exchange } = Dance::new();

    while let Some(op) = letters.next() {
        match op {
            // Increasing the offset has the same effect as rotating elements to the right.
            b's' => {
                offset += 16 - numbers.next().unwrap();
            }
            // Swap two elements taking into account the offset when calculating indices.
            b'x' => {
                let first = numbers.next().unwrap();
                let second = numbers.next().unwrap();
                position.swap((first + offset) % 16, (second + offset) % 16);
            }
            // First lookup the index of each letter, then swap the mapping.
            b'p' => {
                let first = from_byte(letters.next().unwrap());
                let second = from_byte(letters.next().unwrap());
                lookup.swap(first, second);
                exchange.swap(lookup[first], lookup[second]);
            }
            _ => unreachable!(),
        }
    }

    // Rotate the array once to apply all spins.
    position.rotate_left(offset % 16);

    Dance { position, exchange }
}

/// Apply the transformation once.
pub fn part1(input: &Dance) -> String {
    input.apply()
}

/// Repeatedly applying a transformation to itself allows the computation of exponentially
/// more dances, until reaching the complete 1 billion transformations.
pub fn part2(input: &Dance) -> String {
    let mut dance = *input;

    // 1 billion is 0b00111011_10011010_11001010_00000000: 30 bits, with 13 set. Typical
    // exponentiation by squaring would be 30 doubles and 13 additions, or 43 calls to
    // compose; but since one billion is a power of ten, we can do better by 9 cycles of
    // reaching each next power of ten by two doubles, one addition, and one more double
    // per cycle, for a total of only 36 calls to compose.
    for _ in 0..9 {
        let dance2 = dance.compose(dance);
        let dance5 = dance2.compose(dance2).compose(dance);
        dance = dance5.compose(dance5);
    }

    dance.apply()
}

fn from_byte(b: u8) -> usize {
    (b - b'a') as usize
}

fn to_char(i: usize) -> char {
    ((i as u8) + b'a') as char
}
