//! # Docking Data
//!
//! First we parse each mask into 2 `u64` values, `ones` where a bit is set when there is a
//! corresponding "1" in the mask and `xs` (plural of "x") where a bit is set when there is a
//! corresponding "X" in the mask. A bit will never be set in the same location in both `ones` and
//! `xs`. For example:
//!
//! ```none
//!     Mask: 000000000000000000000000000000X1001X
//!     ones: 000000000000000000000000000000010010
//!     xs:   000000000000000000000000000000100001
//! ```
//!
//! ## Part One
//! The memory values are quite sparse, about 500 discrete values in a address range of about 65,000.
//! This makes a [`FastMap`] a better choice than a large mostly empty array. Storing the correct
//! value is a straightforward application of the problem rules, expressed as bitwise logic.
//!
//!
//! ## Part Two
//! This part is subtly tricky to solve quickly. The maximum number of Xs in any mask is 9 which
//! gives 2⁹ = 512 different memory addresses. A brute force solution will work, but there's a much
//! more elegant approach.
//!
//! We treat each address and mask combination as a set. Then by using the
//! [inclusion-exclusion principle ](https://en.wikipedia.org/wiki/Inclusion%E2%80%93exclusion_principle)
//! we can determine any overlaps with other sets and deduct the correct number of values.
//!
//! For example:
//! ```none
//!     mask = 0000000000000000000000000000000001XX  // A
//!     mem[8] = 3
//!     mask = 00000000000000000000000000000000011X  // B
//!     mem[8] = 5
//!     mask = 000000000000000000000000000000000111  // C
//!     mem[8] = 7
//! ```
//! Results in the following address sets:
//! ```none
//!    ┌──────────────┐A            Set A: 12 13 14 15
//!    │ 12 13        │             Set B: 14 15
//!    │ ┌─────────┐B │             Set C: 15
//!    │ │ 14      │  │
//!    │ │ ┌────┐C │  │
//!    │ │ │ 15 │  │  │
//!    │ │ └────┘  │  │
//!    │ └─────────┘  │
//!    └──────────────┘
//! ```
//!
//! Using the inclusion-exclusion principle the remaining size of A is:
//! 4 (initial size) - 2 (overlap with B) - 1 (overlap with C) + 1 (overlap between B and C) = 2
//! If there were any quadruple overlaps we would add those, subtract quintuple, and so on until
//! there are no more overlaps remaining.
//!
//! To calculate the final answer we treat the value as the weight of the set, in this case:
//! 2 * 3 + 1 * 5 + 1 * 7 = 18
//!
//! The complexity of this approach depends on how many addresses overlap. In my input most
//! addresses overlapped with zero others, a few with one and rarely with more than one.
//! Benchmarking against the brute force solution showed that this approach is ~90x faster.
//!
//! [`FastMap`]: crate::util::hash
use crate::util::hash::*;
use crate::util::parse::*;

#[derive(Copy, Clone)]
pub enum Instruction {
    Mask { ones: u64, xs: u64 },
    Mem { address: u64, value: u64 },
}

impl Instruction {
    fn mask(pattern: &str) -> Instruction {
        let mut ones = 0;
        let mut xs = 0;

        for b in pattern.bytes() {
            ones <<= 1;
            xs <<= 1;
            match b {
                b'1' => ones |= 1,
                b'X' => xs |= 1,
                _ => (),
            }
        }

        Self::Mask { ones, xs }
    }
}

struct Set {
    ones: u64,
    floating: u64,
    weight: u64,
}

impl Set {
    /// The one bits are from the original address, plus any from the mask, less any that
    /// overlap with Xs.
    fn from(address: u64, value: u64, ones: u64, floating: u64) -> Set {
        Set { ones: (address | ones) & !floating, floating, weight: value }
    }

    /// Sets are disjoint if any 2 one bits are different and there is no X in either set.
    ///
    /// The intersection of two masks looks like:
    /// ```none
    ///     First:  0011XXX
    ///     Second: 0X1X01X
    ///     Result: 001101X
    /// ```
    fn intersect(&self, other: &Set) -> Option<Set> {
        let disjoint = (self.ones ^ other.ones) & !(self.floating | other.floating);

        (disjoint == 0).then_some(Set {
            ones: self.ones | other.ones,
            floating: self.floating & other.floating,
            weight: 0,
        })
    }

    fn size(&self) -> i64 {
        1 << self.floating.count_ones()
    }
}

pub fn parse(input: &str) -> Vec<Instruction> {
    let mut instructions = Vec::new();

    for line in input.lines() {
        let instruction = if line.len() == 43 {
            Instruction::mask(&line[7..])
        } else {
            let (address, value) = line[4..].split_once("] = ").unwrap();
            let address = address.unsigned();
            let value = value.unsigned();
            Instruction::Mem { address, value }
        };
        instructions.push(instruction);
    }

    instructions
}

pub fn part1(input: &[Instruction]) -> u64 {
    let mut set = 0;
    let mut keep = 0;
    let mut memory = FastMap::new();

    for &instruction in input {
        match instruction {
            Instruction::Mask { ones, xs } => {
                set = ones;
                keep = ones | xs;
            }
            Instruction::Mem { address, value } => {
                memory.insert(address, (value | set) & keep);
            }
        }
    }

    memory.values().sum()
}

pub fn part2(input: &[Instruction]) -> u64 {
    let mut ones = 0;
    let mut floating = 0;
    let mut sets = Vec::new();

    for &instruction in input {
        match instruction {
            Instruction::Mask { ones: next_ones, xs } => {
                ones = next_ones;
                floating = xs;
            }
            Instruction::Mem { address, value } => {
                sets.push(Set::from(address, value, ones, floating));
            }
        }
    }

    let mut total = 0;
    let mut candidates = Vec::new();

    for (i, set) in sets.iter().enumerate() {
        sets[(i + 1)..]
            .iter()
            .filter_map(|other| set.intersect(other))
            .for_each(|next| candidates.push(next));

        let size = set.size() + subsets(set, -1, &candidates);

        total += size as u64 * set.weight;
        candidates.clear();
    }

    total
}

fn subsets(cube: &Set, sign: i64, candidates: &[Set]) -> i64 {
    let mut total = 0;

    for (i, other) in candidates.iter().enumerate() {
        if let Some(next) = cube.intersect(other) {
            total += sign * next.size() + subsets(&next, -sign, &candidates[(i + 1)..]);
        }
    }

    total
}
