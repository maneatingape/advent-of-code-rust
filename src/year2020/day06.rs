//! # Custom Customs
//!
//! This is a disguised binary question like the previous [`day 5`].
//!
//! We can store each passenger's answers as an implicit set in a `u32` since the cardinality
//! is only 26. For each yes answer we set a bit, shifting left based on the letter. For example
//! `acf` would be represented as `100101`.
//!
//! The two lines that update the bitset ( bit = 1 << (b & 32); curr |= bit;) can be combined
//! into a single opcode on x86, BTS which combines them into a single-cycle instruction.
//!
//! For part one to find groups where any person answered yes, we reduce the group using
//! [bitwise OR](https://en.wikipedia.org/wiki/Bitwise_operation) then count the number of ones
//! for each group using the blazing fast [`count_ones`] intrinsic. (popcount on x86)
//!
//! Part two is very similar, except that we use a bitwise AND instead.
//!
//! [`day 6`]: crate::year2020::day06
//! [`count_ones`]: u32::count_ones

type Input = [u32;2];

pub fn parse(input: &str) -> Input 
{
    let mut input = input.to_string();
    if input.as_bytes()[input.len()-1] != b'\n' { input.push('\n')}
    input.push('\n');

    let bytes = input.as_bytes();
    let mut i = 0;
    let mut part1 = 0;
    let mut part2 = 0;
    let mut all:u32 = u32::MAX;
    let mut any:u32 = 0;
    let mut curr = 0;
    let len = bytes.len();
    loop {
        let mut b = bytes[i]; i += 1;
        loop {
            let bit = 1 << (b & 31);
            curr |= bit;
            b = bytes[i]; i += 1;
            if b == b'\n' {break}
        }
        all &= curr;
        any |= curr;
        curr = 0;
        if bytes[i] == b'\n' { // Is this a double newline ending a section? Safe look-ahead due to input padding!
            part1 += any.count_ones();
            part2 += all.count_ones();
            all = u32::MAX;
            any = 0;
            i += 1;
            if i >= len {break} // The input can only end here, so the main loop does not have to test
        }
    }
    [part1, part2]

}

pub fn part1(input: &Input) -> u32 {
    input[0]
}

pub fn part2(input: &Input) -> u32 {
    input[1]
}

