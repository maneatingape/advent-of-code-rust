//! # Arithmetic Logic Unit
//!
//! There are two ways to solve this problem. We can brute force emulate the ALU, but even with
//! memoization this takes a long time to solve. Instead analyzing and reverse engineering the code
//! shows an insight that reduces the problem to a much simpler constraint satisfaction problem.
//!
//! ## Analysis Summary
//!
//! The code consists of 14 blocks of 18 instructions, each block starting with `inp w`.
//!
//! There are 7 "push" blocks and 7 "pop" blocks.
//! * Push blocks use `z` as a stack, multiplying by 26 and adding `w` plus some constant `k₁`.
//!   Push blocks always add to the stack.
//! * Pop blocks compare the top element of `z` plus some constant `k₂` to `w`. If the values are
//!   equal then `z` is "popped" by dividing by 26. Otherwise `z` is pushed again with `w`
//!   plus some constant.
//!
//! Since the push/pop blocks are equally numbered and we want `z` to be zero (empty stack) at the
//! end of the program, the condition:
//!
//! `w₁ + k₁ + k₂ = w₂`
//!
//! must be true for each matching pair of push/pop blocks. Since we also know that each `w` is
//! between 1 and 9 inclusive, that is enough information to determine maximum and minimum values
//! for each of the fourteen `w` values.
//!
//! For example:
//! * If `k₁ + k₂ = 7` then `w₁ + 7 = w₂`.
//! * Maximum value of `w₁` is 2 when `w₂` is 9.
//! * Minimum value of `w₁` is 1 when `w₂` is 8.
//!
//! ## Detailed Push block analysis
//!
//! ```none
//!     inp w       // w = 1 to 9 inclusive
//!     mul x 0     // x = 0
//!     add x z     // x = z
//!     mod x 26    // x = z % 26
//!     div z 1     // nop
//!     add x 13    // x = z % 26 + 13
//!     eql x w     // if (z % 26 + 13) == w { x = 1 } else { x = 0 }
//!                 // However since w is restricted to 1 to 9 this condition is always false
//!                 // so x is always 0. Other blocks have different constants but always > 9.
//!     eql x 0     // x = 1 (as 0 == 0)
//!     mul y 0     // y = 0
//!     add y 25    // y = 25
//!     mul y x     // y = 25 * 1 = 25
//!     add y 1     // y = 25 + 1 = 26
//!     mul z y     // z = 26 * z
//!     mul y 0     // y = 0
//!     add y w     // y = w
//!     add y 14    // y = w + 14 (k₁ = 14)
//!     mul y x     // y = (w + 14) * 1 = (w + 14)
//!     add z y     // z = (26 * z) + (w + 14)
//! ```
//!
//! ## Detailed Pop block analysis
//!
//! ```none
//!     inp w       // w = 1 to 9 inclusive
//!     mul x 0     // x = 0
//!     add x z     // x = z
//!     mod x 26    // x = z % 26
//!     div z 26    // z /= 26 (pop)
//!     add x -13   // x = z % 26 - 13 (k₂ = -13)
//!     eql x w     // if (z % 26 - 13) == w { x = 1 } else { x = 0 }
//!     eql x 0     // if (z % 26 - 13) == w { x = 0 } else { x = 1 }
//!                 // Inverts the previous conditional.
//!                 // Unlike the push blocks, this may be true or false
//!                 // We'll split into 2 paths, depending on equals (x = 0) or
//!                 // not equal (x = 1).
//!                 | Equals (x = 0)        | Not Equals (x = 1)        |
//!     mul y 0     | y = 0                 | y = 0                     |
//!     add y 25    | y = 25                | y = 25                    |
//!     mul y x     | y = 25 * 0 = 0        | y = 25 * 1 = 25           |
//!     add y 1     | y = 0 + 1 = 1         | y = 25 + 1 = 26           |
//!     mul z y     | z = z (nop)           | z = 26 * z                |
//!     mul y 0     | y = 0                 | y = 0                     |
//!     add y w     | y = w                 | y = w                     |
//!     add y 4     | y = w + 4             | y = w + 4                 |
//!     mul y x     | y = (w + 4) * 0       | y = (w + 4) * 1           |
//!     add z y     | z = z                 | z = (26 * z) * (w + 4)    |
//! ```
use crate::util::parse::*;
use Block::*;

/// Blocks are either "push" or "pop".
enum Block {
    Push(i32),
    Pop(i32),
}

/// Convert matching pairs of blocks into constraints.
/// For the first digit `value` is `-(k₁ + k₂)` and second digit value is `k₁ + k₂`.
pub struct Constraint {
    index: usize,
    value: i32,
}

/// Convert `k₁ + k₂` to min and max values, clamping at 1 and 9 respectively.
impl Constraint {
    fn min(&self) -> i32 {
        (1 + self.value).max(1)
    }

    fn max(&self) -> i32 {
        (9 + self.value).min(9)
    }
}

pub fn parse(input: &str) -> Vec<Constraint> {
    let lines: Vec<_> = input.lines().collect();
    let blocks: Vec<_> = lines
        .chunks(18)
        .map(|chunk| {
            // Parse the last token on the specified line within a block.
            let helper = |i: usize| chunk[i].split_ascii_whitespace().last().unwrap().signed();
            // The 5th instruction in "push" blocks is always a `div z 1`
            // that we can use to figure out what type of block we're dealing with.
            if helper(4) == 1 {
                // `k₁` is always located at the 16th instruction.
                Push(helper(15))
            } else {
                // `k₂` is always located at the 6th instruction.
                Pop(helper(5))
            }
        })
        .collect();

    let mut stack = Vec::new();
    let mut constraints = Vec::new();

    for (index, blocks) in blocks.into_iter().enumerate() {
        match blocks {
            Push(value) => stack.push(Constraint { index, value }),
            Pop(second_value) => {
                // Find the matching "push" instruction at the top of the stack.
                let mut first = stack.pop().unwrap();
                // delta = k₁ + k₂
                let delta = first.value + second_value;
                // w₁ + delta = w₂ <=> w₁ = w₂ - delta
                first.value = -delta;
                let second = Constraint { index, value: delta };
                constraints.push(first);
                constraints.push(second);
            }
        }
    }

    // Sort by original ALU program order
    constraints.sort_unstable_by_key(|c| c.index);
    constraints
}

pub fn part1(input: &[Constraint]) -> String {
    input.iter().map(|c| c.max().to_string()).collect()
}

pub fn part2(input: &[Constraint]) -> String {
    input.iter().map(|c| c.min().to_string()).collect()
}
