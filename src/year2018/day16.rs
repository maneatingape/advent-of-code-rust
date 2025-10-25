//! # Chronal Classification
//!
//! There are only 16 opcodes so we can use bitwise logic to efficiently perform the set operations
//! that uniquely determine the mapping of each opcode to instruction.
//!
//! First we create a bitmask for each instruction block in the first half of the input
//! with a `1` for each potential instruction. For example:
//!
//! ```none
//!     Before: [3, 2, 1, 1]
//!     9 2 1 2
//!     After:  [3, 2, 2, 1]
//!
//!     Possible instructions: mulr, addi, seti
//!     Binary Mask: 0000001000000110
//! ```
//!
//! For part one the [`count_ones`] intrinsic computes the size of each set.
//!
//! For part two we need to determine the mapping of the unknown codes. First we reduce each
//! unknown to a single set by taking the intersection of all examples. Then similar to
//! solving simultaneous equation, we eliminate one unknown at a time, removing it from the other
//! possibilities. This causes a domino effect, continuing until all unknowns are resolved.
//!
//! [`count_ones`]: u32::count_ones
use crate::util::iter::*;
use crate::util::parse::*;

pub struct Input {
    samples: Vec<(usize, u32)>,
    program: Vec<[usize; 4]>,
}

pub fn parse(input: &str) -> Input {
    let (first, second) = input.rsplit_once("\n\n").unwrap();
    let samples = first
        .iter_unsigned()
        .chunk::<4>()
        .chunk::<3>()
        .map(|[before, instruction, after]| {
            let [unknown, a, b, c] = instruction;
            let mut mask = 0;

            // Build set of possible opcodes
            for opcode in 0..16 {
                if cpu(opcode, a, b, &before) == after[c] {
                    mask |= 1 << opcode;
                }
            }

            (unknown, mask)
        })
        .collect();
    let program = second.iter_unsigned().chunk::<4>().collect();

    Input { samples, program }
}

pub fn part1(input: &Input) -> usize {
    input.samples.iter().filter(|(_, mask)| mask.count_ones() >= 3).count()
}

pub fn part2(input: &Input) -> usize {
    // Take intersection of samples, reducing each unknown opcode to a single set of possibilities.
    let mut masks = [0xffff; 16];

    for &(unknown, mask) in &input.samples {
        masks[unknown] &= mask;
    }

    // To uniquely determine the mapping, there must be at least 1 opcode during each iteration
    // that only has one possibility.
    let mut convert = [0; 16];

    while let Some(index) = masks.iter().position(|&n| n.count_ones() == 1) {
        let mask = masks[index];
        // This opcode has only 1 possible mapping, so remove possibility from other opcodes.
        masks.iter_mut().for_each(|m| *m &= !mask);
        // Add mapping.
        convert[index] = mask.trailing_zeros() as usize;
    }

    // Run the program now that we know the mapping.
    let mut register = [0; 4];

    for &[unknown, a, b, c] in &input.program {
        let opcode = convert[unknown];
        register[c] = cpu(opcode, a, b, &register);
    }

    register[0]
}

fn cpu(opcode: usize, a: usize, b: usize, register: &[usize; 4]) -> usize {
    match opcode {
        0 => register[a] + register[b],              // addr
        1 => register[a] + b,                        // addi
        2 => register[a] * register[b],              // mulr
        3 => register[a] * b,                        // muli
        4 => register[a] & register[b],              // banr
        5 => register[a] & b,                        // bani
        6 => register[a] | register[b],              // borr
        7 => register[a] | b,                        // bori
        8 => register[a],                            // setr
        9 => a,                                      // seti
        10 => (a > register[b]) as usize,            // gtir
        11 => (register[a] > b) as usize,            // gtri
        12 => (register[a] > register[b]) as usize,  // gtrr
        13 => (a == register[b]) as usize,           // eqir
        14 => (register[a] == b) as usize,           // eqri
        15 => (register[a] == register[b]) as usize, // eqrr
        _ => unreachable!(),
    }
}
