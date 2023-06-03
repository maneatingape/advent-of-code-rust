//! # Not Quite Lisp
//!
//! The input is first converted into bytes. This is safe as it contains only ASCII characters.
//! Then each parenthesis is parsed into either +1 or -1, treating the trailing newline
//! as a special case of 0.
pub fn parse(input: &str) -> Vec<i32> {
    fn helper(b: &u8) -> i32 {
        match b {
            b'(' => 1,
            b')' => -1,
            _ => 0,
        }
    }
    input.as_bytes().iter().map(helper).collect()
}

pub fn part1(input: &[i32]) -> i32 {
    input.iter().sum()
}

pub fn part2(input: &[i32]) -> usize {
    let mut floor = 0;

    for (i, x) in input.iter().enumerate() {
        floor += x;
        if floor < 0 {
            return i + 1;
        }
    }

    unreachable!()
}
