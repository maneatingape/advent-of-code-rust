//! # An Elephant Named Joseph
//!
//! The title is a reference to the [Josephus problem](https://en.wikipedia.org/wiki/Josephus_problem).
//! We can solve both parts efficiently in `O(1)` constant storage without needing any
//! auxiliary data structures.
//!
//! ## Part One
//!
//! Part one is exactly the Josephus problem with k = 2. Each round the number of elves is either
//! odd or even. If the number of elves is even then every second elf is eliminated and the starting
//! elf remains the same, for example starting with 8 elves and a step size of 1:
//!
//! 1 ~~2~~ 3 ~~4~~ 5 ~~6~~ 7 ~~8~~
//!
//! The next round we double the step size used to find elves from 1 to 2:
//!
//! 1 ~~2~~ ~~3~~ ~~4~~ 5 ~~6~~ ~~7~~ ~~8~~
//!
//! In the special case that the number of elves is power of two then the starting elf will always
//! win.
//!
//! If the number of elves is odd then the last elf will also eliminate the starting elf,
//! so the new starting elf increases by the step size.
//!
//! ~~1~~ ~~2~~ 3 ~~4~~ 5
//!
//! We can represent this as a loop:
//!
//! ```none
//!    let mut n = <starting number of elves>
//!    let mut step = 1;
//!    let mut winner = 1;
//!    while n > 1 {
//!        if n % 2 == 1 {
//!            winner += step * 2;
//!        }
//!        n /= 2;
//!        step *= 2;
//!    }
//! ```
//!
//! If we examine the loop we can see that the winner is simply the binary digits of `n` multiplied
//! by two, excluding the highest bit, with one added. For example for 5 elves:
//!
//! ```none
//!     n = 5 = 101
//!     n * 2 = 10 = 1010
//!     n minus high bit = 010
//!     n plus one = 011 = 3
//! ```
//!
//! The [`ilog2`] function will return the number of zeroes before the highest one bit, for example
//! `10.ilog2() = 3`. We can then shift a bit by that amount and subtract to get the result in
//! constant `O(1)` time.
//!
//!
//! ## Part Two
//!
//! Part two is a variant of the problem. We solve in `log(n)` time by working *backwards*
//! from the winning elf until we reach the starting number of elves.
//! Starting with the winning elf `a` it must have eliminated its neighbor to the right:
//!
//! `a` => `a b`
//!
//! We then choose the previous elf to the left wrapping around to elf `b` in this case. Elf `b`
//! must have eliminated its neighbor 1 step to the right:
//!
//! `a b` => `a b c`
//!
//! Play passes to the left, in this case elf `a` must have eliminated an elf 2 steps away:
//!
//! `a b c` => `a b d c`
//!
//! Play passes to the left, wrapping around to elf `c` that must have eliminated an elf 2 steps
//! away:
//!
//! `a b d c` => `a e b d c`
//!
//! Now that we have 5 elves our starting elf `a` is one step away from `c` so the answer is 2.
//!
//! [`ilog2`]: u32::ilog2
use crate::util::parse::*;

pub fn parse(input: &str) -> u32 {
    input.unsigned()
}

pub fn part1(input: &u32) -> u32 {
    let mut elf = *input;

    elf *= 2;
    // Remove highest 1 bit
    elf -= 1 << elf.ilog2();
    // Elves use 1-based indexing
    elf += 1;

    elf
}

pub fn part2(input: &u32) -> u32 {
    let target = *input;
    let mut elf = 0;
    let mut size = 1;

    while size < target {
        let remaining = target - size;

        // The trick to log(n) time is that we can handle all elves greater than or less than
        // the starting point in one pass, greatly increasing efficiency.
        if elf > size / 2 {
            // If the elf is greater than the half way point, then an elf will be inserted before
            // it. This cancels out moving to the previous elf so our position remains the same.
            let possible = 2 * elf - size;
            size += possible.min(remaining);
        } else {
            // The next elf will be the one before us. If we are at the start then wrap around
            // to the last elf.
            if elf >= remaining {
                elf -= remaining;
                size += remaining;
            } else {
                elf += size;
                size = elf + 1;
            }
        }
    }

    // The winning position is at 0, so its number is the distance from the starting elf.
    target - elf
}
