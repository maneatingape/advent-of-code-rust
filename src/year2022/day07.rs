//! # No Space Left On Device
//!
//! Some up-front analysis of the input data helps us develop an efficient solving algorithm (this
//! is a regular theme in AoC!). Looking at the directory commands shows 2 key insights:
//! * We never return to a previously visited directory
//! * Directory traversal is only up or down in steps of one.
//!
//! This allows us to infer:
//! * `$ ls` lines contain no useful information and can be ignored.
//! * `dir foo` lines also contain no useful information and can be ignored.
//! * Only the size in `12345 foo.bar` file listings is useful.
//! * `cd foo` commands imply a "down" direction, but the name is not needed and can be ignored.
//! * `cd ..` commands imply that we are finished with the current directory.
//!
//! For my input data this meant that 58% of it was unnecessary! Our algorithm will be:
//! * If we encounter a file listing then add its size to the current running total.
//! * Create a `vec` to function as a stack of incomplete directories. Anytime we encounter a
//!   `cd foo` command, then we push the size of the current directory to this stack to save for
//!   later, then reset our running total to 0.
//! * Create a second `vec` to store the sizes of completed directories. Anytime we encounter
//!   a `cd ..` then we can "complete" the current directory and add its size to this list. To find
//!   our new running total we then pop the previous unfinished directory off the stack
//!   (and this is the neat part) *add* the size of the just completed directory, since we know
//!   that it must have been a child of the directory at the top of the stack.
//!
//!   Note that the end of the file is essentially an sequence of implicit `cd ..` commands
//!   all the way to the root. Another nice side effect is that the root directory is always the
//!   last element in our `vec`.
//!
//! This means that the algorithm is extremely efficient and the data structures are very
//! straightforward. For example there's no need to store the current path names, or to recursively
//! update upwards whenever a file is encountered.
use crate::util::parse::*;

/// Tokenize the input and return a `vec` of directory sizes.
pub fn parse(input: &str) -> Vec<u32> {
    let mut cd = false;
    let mut total = 0;
    let mut stack: Vec<u32> = vec![];
    let mut sizes: Vec<u32> = vec![];

    for token in input.split_ascii_whitespace() {
        if cd {
            if token == ".." {
                sizes.push(total);
                total += stack.pop().unwrap();
            } else {
                stack.push(total);
                total = 0;
            }
            cd = false;
        } else if token == "cd" {
            cd = true;
        } else if token.as_bytes()[0].is_ascii_digit() {
            total += from::<u32>(token);
        }
    }

    while !stack.is_empty() {
        sizes.push(total);
        total += stack.pop().unwrap();
    }

    sizes
}

/// Sum all directories 100,000 bytes or less.
pub fn part1(input: &[u32]) -> u32 {
    input.iter().filter(|&&x| x <= 100_000).sum()
}

/// Find the smallest directory that can be deleted to free up the necessary space.
pub fn part2(input: &[u32]) -> u32 {
    let root = input.last().unwrap();
    let needed = 30_000_000 - (70_000_000 - root);
    *input.iter().filter(|&&x| x >= needed).min().unwrap()
}
