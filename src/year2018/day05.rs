//! # Alchemical Reduction
//!
//! ## Part One
//!
//! This problem is similar to checking if a parentheses expression is balanced or not.
//! We use a similar approach, maintaining a stack of unreacted polymer units. Each unit from the
//! polymer is compared to the head of the stack using bitwise logic. Lowercase and uppercase ASCII
//! codes for the same lettter are always are 32 apart, which can be checked very quickly using
//! bitwise XOR. For example:
//!
//! ```none
//!         A = 65 = 01000001
//!         a = 97 = 01100001
//!     A ^ a = 32 = 00100000
//! ```
//!
//! If two units are the same type but opposite polarity then they are popped from the stack.
//!
//! ## Part Two
//!
//! An important optimization is to use the already reacted polymer from part one. This is
//! approximately 20% of the size of the raw input. Then this smaller polymer is filtered
//! further for each of the 26 kinds of unit.
pub fn parse(input: &str) -> Vec<u8> {
    collapse(input.trim().bytes())
}

pub fn part1(input: &[u8]) -> usize {
    input.len()
}

pub fn part2(input: &[u8]) -> usize {
    (b'a'..=b'z')
        .map(|kind| collapse(input.iter().copied().filter(|&b| b | 32 != kind)).len())
        .min()
        .unwrap()
}

fn collapse(polymer: impl Iterator<Item = u8>) -> Vec<u8> {
    // It's faster to keep the head of the stack in a dedicated variable. 0 is used as a special
    // sentinel kind to indicate an empty stack as it will never match with any unit kind.
    let mut head = 0;
    let mut stack = Vec::with_capacity(10_000);

    for unit in polymer {
        // Uppercase and lowercase ASCII are always 32 apart.
        if head ^ unit == 32 {
            // The head reacts with the unit to annihilate each other so replace with the next unit
            // from the stack.
            head = stack.pop().unwrap_or(0);
        } else {
            // Don't push sentinel values.
            if head != 0 {
                stack.push(head);
            }
            head = unit;
        }
    }

    if head != 0 {
        stack.push(head);
    }

    stack
}
